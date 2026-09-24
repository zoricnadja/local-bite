CREATE TABLE type_catalog (farm_id UUID, name TEXT NOT NULL CHECK (length(btrim(name)) BETWEEN 1 AND 100));
CREATE UNIQUE INDEX type_catalog_farm_name ON type_catalog (farm_id, lower(name)) WHERE farm_id IS NOT NULL;
CREATE UNIQUE INDEX type_catalog_default_name ON type_catalog (lower(name)) WHERE farm_id IS NULL;
INSERT INTO type_catalog (name) VALUES ('Sausage production'), ('Cheese production'), ('Preserves production'), ('Other production');
INSERT INTO type_catalog (farm_id, name)
SELECT DISTINCT ON (farm_id, lower(process_type)) farm_id, process_type FROM production_batches
WHERE NOT EXISTS (SELECT 1 FROM type_catalog t WHERE lower(t.name) = lower(production_batches.process_type))
ON CONFLICT DO NOTHING;
