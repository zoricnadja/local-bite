CREATE OR REPLACE FUNCTION release_removed_material() RETURNS TRIGGER AS $$
BEGIN
 INSERT INTO material_release_jobs(operation_id,business_id) VALUES(OLD.id,OLD.business_id) ON CONFLICT DO NOTHING;
 RETURN OLD;
END; $$ LANGUAGE plpgsql;
CREATE OR REPLACE FUNCTION protect_batch_and_release() RETURNS TRIGGER AS $$
BEGIN
 IF OLD.status='COMPLETED' AND NEW IS DISTINCT FROM OLD THEN
  RAISE EXCEPTION 'Completed production is immutable' USING ERRCODE='23514';
 END IF;
 IF (NEW.status='CANCELLED' AND OLD.status<>'CANCELLED') OR (NEW.is_deleted AND NOT OLD.is_deleted) THEN
  INSERT INTO material_release_jobs(operation_id,business_id) SELECT id,business_id FROM batch_raw_materials WHERE batch_id=NEW.id ON CONFLICT DO NOTHING;
 END IF;
 RETURN NEW;
END; $$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION integration_payload(kind TEXT, row_data JSONB) RETURNS JSONB AS $$
 SELECT CASE
 WHEN kind='users' THEN jsonb_build_object('id',row_data->'id','role',row_data->'role','business_id',row_data->'business_id')
 WHEN kind='businesses' THEN jsonb_build_object('id',row_data->'id','name',row_data->'name','owner_id',row_data->'owner_id')
 WHEN kind='orders' THEN row_data - ARRAY['customer_email','customer_name','notes']
 ELSE row_data - ARRAY['notes'] END;
$$ LANGUAGE SQL IMMUTABLE;
UPDATE integration_outbox SET entity_type='businesses' WHERE entity_type='farms';
-- Preserve applied migration checksums; migrate existing installations in place.
DO $$
DECLARE c RECORD;
BEGIN
 FOR c IN SELECT table_name FROM information_schema.columns WHERE table_schema=current_schema() AND column_name='farm_id' LOOP
  EXECUTE format('ALTER TABLE %I RENAME COLUMN farm_id TO business_id',c.table_name);
 END LOOP;
END $$;

-- Convert only structural keys and role values, never user-entered text.
CREATE FUNCTION migrate_business_json(value JSONB) RETURNS JSONB LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE result JSONB; k TEXT; v JSONB; new_key TEXT;
BEGIN
 IF jsonb_typeof(value)='object' THEN
  result='{}'::jsonb;
  FOR k,v IN SELECT * FROM jsonb_each(value) LOOP
   new_key=CASE k WHEN 'farm_id' THEN 'business_id' WHEN 'farm_name' THEN 'business_name' WHEN 'farm' THEN 'business' ELSE k END;
   IF k='role' AND v='"FARM_OWNER"'::jsonb THEN v='"BUSINESS_OWNER"'::jsonb;
   ELSIF k='role' AND v='"FarmOwner"'::jsonb THEN v='"BusinessOwner"'::jsonb;
   ELSE v=migrate_business_json(v); END IF;
   result=result || jsonb_build_object(new_key,v);
  END LOOP;
  RETURN result;
 ELSIF jsonb_typeof(value)='array' THEN
  SELECT coalesce(jsonb_agg(migrate_business_json(item) ORDER BY ordinal),'[]'::jsonb) INTO result FROM jsonb_array_elements(value) WITH ORDINALITY AS a(item,ordinal);
  RETURN result;
 END IF;
 RETURN value;
END $$;

DO $$
DECLARE c RECORD;
BEGIN
 FOR c IN SELECT table_name,column_name FROM information_schema.columns WHERE table_schema=current_schema() AND data_type='jsonb' AND table_name IN ('integration_outbox','projection_entities','checkout_jobs','stock_reservations') LOOP
  EXECUTE format('UPDATE %I SET %I=migrate_business_json(%I) WHERE %I IS DISTINCT FROM migrate_business_json(%I)',c.table_name,c.column_name,c.column_name,c.column_name,c.column_name);
 END LOOP;
END $$;
DROP FUNCTION migrate_business_json(JSONB);

-- Rename catalog objects without rebuilding data or changing their semantics.
DO $$
DECLARE obj RECORD; new_name TEXT;
BEGIN
 FOR obj IN SELECT conrelid::regclass AS tbl,conname FROM pg_constraint WHERE connamespace=current_schema()::regnamespace AND conname LIKE '%farm%' LOOP
  new_name=replace(replace(obj.conname,'farms','businesses'),'farm','business');
  EXECUTE format('ALTER TABLE %s RENAME CONSTRAINT %I TO %I',obj.tbl,obj.conname,new_name);
 END LOOP;
 FOR obj IN SELECT indexname FROM pg_indexes WHERE schemaname=current_schema() AND indexname LIKE '%farm%' LOOP
  new_name=replace(replace(obj.indexname,'farms','businesses'),'farm','business');
  EXECUTE format('ALTER INDEX %I RENAME TO %I',obj.indexname,new_name);
 END LOOP;
END $$;
