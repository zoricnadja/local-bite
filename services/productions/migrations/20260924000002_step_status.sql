-- Keep the historical type column only for data preservation; it is no longer used.
ALTER TABLE production_batches ALTER COLUMN process_type SET DEFAULT '';
ALTER TABLE process_steps ADD COLUMN status TEXT NOT NULL DEFAULT 'PLANNED'
    CHECK (status IN ('PLANNED', 'IN_PROGRESS', 'COMPLETED'));
-- Schema migration holds the table lock until commit; bypass only the old
-- immutability guard while backfilling historical completed rows.
ALTER TABLE process_steps DISABLE TRIGGER immutable_steps;
UPDATE process_steps s SET status = 'COMPLETED'
FROM production_batches b WHERE s.batch_id = b.id AND b.status = 'COMPLETED';
ALTER TABLE process_steps ENABLE TRIGGER immutable_steps;

-- Serialize step changes with closing the parent batch, including concurrent requests.
CREATE FUNCTION guard_step_state() RETURNS trigger AS $$
DECLARE parent_status TEXT;
BEGIN
    SELECT status INTO parent_status FROM production_batches
    WHERE id = CASE WHEN TG_OP = 'DELETE' THEN OLD.batch_id ELSE NEW.batch_id END
    FOR UPDATE;
    IF parent_status IN ('COMPLETED', 'CANCELLED') THEN
        RAISE EXCEPTION 'Cannot change steps of a closed batch' USING ERRCODE = '23514';
    END IF;
    IF TG_OP = 'INSERT' AND NEW.status <> 'PLANNED' THEN
        RAISE EXCEPTION 'New steps must be planned' USING ERRCODE = '23514';
    END IF;
    IF TG_OP = 'UPDATE' AND NEW.status <> OLD.status AND NOT (
        (OLD.status = 'PLANNED' AND NEW.status = 'IN_PROGRESS') OR
        (OLD.status = 'IN_PROGRESS' AND NEW.status = 'COMPLETED')
    ) THEN
        RAISE EXCEPTION 'Invalid step status transition' USING ERRCODE = '23514';
    END IF;
    IF TG_OP = 'DELETE' THEN RETURN OLD; END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER guard_step_state BEFORE INSERT OR UPDATE OR DELETE ON process_steps
FOR EACH ROW EXECUTE FUNCTION guard_step_state();

CREATE FUNCTION start_batch_from_step() RETURNS trigger AS $$
BEGIN
    IF NEW.status = 'IN_PROGRESS' THEN
        UPDATE production_batches SET status = 'IN_PROGRESS', start_date = COALESCE(start_date, CURRENT_DATE)
        WHERE id = NEW.batch_id AND status = 'PLANNED';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER start_batch_from_step AFTER UPDATE OF status ON process_steps
FOR EACH ROW EXECUTE FUNCTION start_batch_from_step();

CREATE FUNCTION guard_batch_step_completion() RETURNS trigger AS $$
BEGIN
    IF NEW.status = 'COMPLETED' AND OLD.status <> 'COMPLETED' AND (
        NOT EXISTS (SELECT 1 FROM process_steps WHERE batch_id = NEW.id) OR
        EXISTS (SELECT 1 FROM process_steps WHERE batch_id = NEW.id AND status <> 'COMPLETED')
    ) THEN
        RAISE EXCEPTION 'Complete all process steps before completing production' USING ERRCODE = '23514';
    END IF;
    IF NEW.status = 'IN_PROGRESS' AND OLD.status = 'PLANNED' AND NOT EXISTS (
        SELECT 1 FROM process_steps WHERE batch_id = NEW.id AND status = 'IN_PROGRESS'
    ) THEN
        RAISE EXCEPTION 'Start a process step to start production' USING ERRCODE = '23514';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER guard_batch_step_completion BEFORE UPDATE OF status ON production_batches
FOR EACH ROW EXECUTE FUNCTION guard_batch_step_completion();
