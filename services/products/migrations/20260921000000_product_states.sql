ALTER TABLE products ADD COLUMN status TEXT NOT NULL DEFAULT 'STORAGE' CHECK (status IN ('PRODUCTION','STORAGE','ON_SALE'));
UPDATE products SET status=CASE WHEN is_active THEN 'ON_SALE' ELSE 'STORAGE' END;
ALTER TABLE production_outputs DROP CONSTRAINT production_outputs_pkey;
ALTER TABLE production_outputs ADD PRIMARY KEY(product_id);
CREATE INDEX production_outputs_batch_idx ON production_outputs(batch_id);
ALTER TABLE production_outputs ADD COLUMN output_id UUID;
CREATE UNIQUE INDEX production_outputs_identity ON production_outputs(batch_id,output_id);
