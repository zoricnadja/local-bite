ALTER TABLE products ALTER COLUMN is_active SET DEFAULT FALSE;
CREATE TABLE production_outputs (batch_id UUID PRIMARY KEY, product_id UUID NOT NULL UNIQUE REFERENCES products(id));
CREATE TABLE production_states(batch_id UUID PRIMARY KEY, farm_id UUID NOT NULL, status TEXT NOT NULL, sequence BIGINT NOT NULL, deleted BOOLEAN NOT NULL DEFAULT FALSE);
