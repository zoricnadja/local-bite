CREATE TABLE material_intents (
 operation_id UUID PRIMARY KEY, farm_id UUID NOT NULL,
 created_at TIMESTAMPTZ NOT NULL DEFAULT now(), completed_at TIMESTAMPTZ
);
CREATE TABLE material_release_jobs (
 operation_id UUID PRIMARY KEY, farm_id UUID NOT NULL,
 created_at TIMESTAMPTZ NOT NULL DEFAULT now(), completed_at TIMESTAMPTZ
);
CREATE FUNCTION protect_production_children() RETURNS TRIGGER AS $$
DECLARE parent_id UUID; parent_status TEXT; parent_deleted BOOLEAN;
BEGIN
 parent_id=CASE WHEN TG_OP='DELETE' THEN OLD.batch_id ELSE NEW.batch_id END;
 SELECT status,is_deleted INTO parent_status,parent_deleted FROM production_batches WHERE id=parent_id FOR UPDATE;
 IF parent_status IN ('COMPLETED','CANCELLED') OR parent_deleted THEN
   RAISE EXCEPTION 'Production is immutable' USING ERRCODE='23514';
 END IF;
 IF TG_OP='DELETE' THEN RETURN OLD; END IF;
 RETURN NEW;
END; $$ LANGUAGE plpgsql;
CREATE TRIGGER immutable_steps BEFORE INSERT OR UPDATE OR DELETE ON process_steps FOR EACH ROW EXECUTE FUNCTION protect_production_children();
CREATE TRIGGER immutable_materials BEFORE INSERT OR UPDATE OR DELETE ON batch_raw_materials FOR EACH ROW EXECUTE FUNCTION protect_production_children();
CREATE FUNCTION release_removed_material() RETURNS TRIGGER AS $$
BEGIN
 INSERT INTO material_release_jobs(operation_id,farm_id) VALUES(OLD.id,OLD.farm_id) ON CONFLICT DO NOTHING;
 RETURN OLD;
END; $$ LANGUAGE plpgsql;
CREATE TRIGGER release_material AFTER DELETE ON batch_raw_materials FOR EACH ROW EXECUTE FUNCTION release_removed_material();
CREATE FUNCTION protect_batch_and_release() RETURNS TRIGGER AS $$
BEGIN
 IF OLD.status='COMPLETED' AND NEW IS DISTINCT FROM OLD THEN
  RAISE EXCEPTION 'Completed production is immutable' USING ERRCODE='23514';
 END IF;
 IF (NEW.status='CANCELLED' AND OLD.status<>'CANCELLED') OR (NEW.is_deleted AND NOT OLD.is_deleted) THEN
  INSERT INTO material_release_jobs(operation_id,farm_id) SELECT id,farm_id FROM batch_raw_materials WHERE batch_id=NEW.id ON CONFLICT DO NOTHING;
 END IF;
 RETURN NEW;
END; $$ LANGUAGE plpgsql;
CREATE TRIGGER batch_integrity BEFORE UPDATE ON production_batches FOR EACH ROW EXECUTE FUNCTION protect_batch_and_release();
