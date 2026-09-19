-- Compatibility migration for the legacy product state column.
ALTER TABLE products ADD COLUMN IF NOT EXISTS state TEXT NOT NULL DEFAULT 'storage';
UPDATE products SET state = CASE WHEN is_active THEN 'on_sale' ELSE 'storage' END;
CREATE INDEX IF NOT EXISTS idx_products_state ON products(state);
