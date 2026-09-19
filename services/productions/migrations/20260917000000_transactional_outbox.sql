CREATE TABLE integration_outbox (
 sequence BIGSERIAL PRIMARY KEY,
 entity_type TEXT NOT NULL, entity_id UUID NOT NULL,
 operation TEXT NOT NULL CHECK (operation IN ('INSERT','UPDATE','DELETE')),
 data JSONB NOT NULL, occurred_at TIMESTAMPTZ NOT NULL DEFAULT now(), published_at TIMESTAMPTZ
);
CREATE INDEX integration_outbox_pending ON integration_outbox(sequence) WHERE published_at IS NULL;

-- Public/auth projection events deliberately exclude credentials and private profiles.
CREATE FUNCTION integration_payload(kind TEXT, row_data JSONB) RETURNS JSONB AS $$
 SELECT CASE
 WHEN kind='users' THEN jsonb_build_object('id',row_data->'id','role',row_data->'role','farm_id',row_data->'farm_id')
 WHEN kind='farms' THEN jsonb_build_object('id',row_data->'id','name',row_data->'name','owner_id',row_data->'owner_id')
 WHEN kind='orders' THEN row_data - ARRAY['customer_email','customer_name','notes']
 ELSE row_data - ARRAY['notes'] END;
$$ LANGUAGE SQL IMMUTABLE;

CREATE FUNCTION capture_integration_event() RETURNS TRIGGER AS $$
DECLARE row_data JSONB;
BEGIN
 IF TG_OP='DELETE' THEN row_data=to_jsonb(OLD); ELSE row_data=to_jsonb(NEW); END IF;
 INSERT INTO integration_outbox(entity_type,entity_id,operation,data)
 VALUES(TG_TABLE_NAME,(row_data->>'id')::uuid,TG_OP,integration_payload(TG_TABLE_NAME,row_data));
 IF TG_OP='DELETE' THEN RETURN OLD; END IF;
 RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER integration_event AFTER INSERT OR UPDATE OR DELETE ON production_batches FOR EACH ROW EXECUTE FUNCTION capture_integration_event();
INSERT INTO integration_outbox(entity_type,entity_id,operation,data) SELECT 'production_batches',id,'INSERT',integration_payload('production_batches',to_jsonb(t)) FROM production_batches t;

CREATE TRIGGER integration_event AFTER INSERT OR UPDATE OR DELETE ON process_steps FOR EACH ROW EXECUTE FUNCTION capture_integration_event();
INSERT INTO integration_outbox(entity_type,entity_id,operation,data) SELECT 'process_steps',id,'INSERT',integration_payload('process_steps',to_jsonb(t)) FROM process_steps t;

CREATE TRIGGER integration_event AFTER INSERT OR UPDATE OR DELETE ON batch_raw_materials FOR EACH ROW EXECUTE FUNCTION capture_integration_event();
INSERT INTO integration_outbox(entity_type,entity_id,operation,data) SELECT 'batch_raw_materials',id,'INSERT',integration_payload('batch_raw_materials',to_jsonb(t)) FROM batch_raw_materials t;
