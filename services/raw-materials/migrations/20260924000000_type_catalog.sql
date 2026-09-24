CREATE TABLE type_catalog (farm_id UUID, name TEXT NOT NULL CHECK (length(btrim(name)) BETWEEN 1 AND 100));
CREATE UNIQUE INDEX type_catalog_farm_name ON type_catalog (farm_id, lower(name)) WHERE farm_id IS NOT NULL;
CREATE UNIQUE INDEX type_catalog_default_name ON type_catalog (lower(name)) WHERE farm_id IS NULL;
INSERT INTO type_catalog (name) VALUES ('meat'), ('dairy'), ('vegetable'), ('fruit'), ('grain'), ('spice'), ('herb'), ('other');
INSERT INTO type_catalog (farm_id, name)
SELECT DISTINCT ON (farm_id, lower(material_type)) farm_id, material_type FROM raw_materials
WHERE NOT EXISTS (SELECT 1 FROM type_catalog t WHERE lower(t.name) = lower(raw_materials.material_type))
ON CONFLICT DO NOTHING;
