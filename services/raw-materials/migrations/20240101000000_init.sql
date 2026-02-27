CREATE TABLE IF NOT EXISTS raw_materials (
    id            UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    farm_id       UUID        NOT NULL,
    name          VARCHAR(255) NOT NULL,
    material_type VARCHAR(100) NOT NULL,          -- e.g. "meat", "fruit", "dairy", "spice"
    quantity      NUMERIC(12,3) NOT NULL DEFAULT 0,
    unit          VARCHAR(50)  NOT NULL,           -- e.g. "kg", "L", "piece"
    supplier      VARCHAR(255),
    origin        VARCHAR(255),
    harvest_date  DATE,
    expiry_date   DATE,
    notes         TEXT,
    low_stock_threshold NUMERIC(12,3),            -- NULL = no alert
    is_deleted    BOOLEAN     NOT NULL DEFAULT FALSE,
    created_at    TIMESTAMP   NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMP   NOT NULL DEFAULT NOW()
    );

CREATE INDEX idx_raw_materials_farm_id ON raw_materials(farm_id);
CREATE INDEX idx_raw_materials_type    ON raw_materials(material_type);
CREATE INDEX idx_raw_materials_active  ON raw_materials(farm_id, is_deleted);

-- Auto-update updated_at on every row change
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_raw_materials_updated_at
    BEFORE UPDATE ON raw_materials
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
