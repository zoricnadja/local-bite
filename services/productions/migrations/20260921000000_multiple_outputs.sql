ALTER TABLE production_batches ADD COLUMN outputs JSONB NOT NULL DEFAULT '[]'::jsonb;
