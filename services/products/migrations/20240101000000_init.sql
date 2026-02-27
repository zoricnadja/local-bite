-- Product Service - Initial Schema

CREATE TABLE IF NOT EXISTS products (
    id             UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    farm_id        UUID          NOT NULL,
    name           VARCHAR(255)  NOT NULL,
    product_type   VARCHAR(100)  NOT NULL,        -- e.g. "kulen", "ajvar", "med", "sok"
    description    TEXT,
    quantity       NUMERIC(12,3) NOT NULL DEFAULT 0,
    unit           VARCHAR(50)   NOT NULL,         -- e.g. "kg", "jar", "bottle"
    price          NUMERIC(12,2) NOT NULL,
    batch_id       UUID,                           -- links to Production Service
    image_path     VARCHAR(500),                   -- relative path to uploaded image
    qr_token       UUID          NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    qr_path        VARCHAR(500),                   -- path to generated QR PNG
    is_active      BOOLEAN       NOT NULL DEFAULT TRUE,
    is_deleted     BOOLEAN       NOT NULL DEFAULT FALSE,
    created_at     TIMESTAMP     NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMP     NOT NULL DEFAULT NOW()
    );

CREATE INDEX idx_products_farm_id    ON products(farm_id);
CREATE INDEX idx_products_type       ON products(product_type);
CREATE INDEX idx_products_active     ON products(farm_id, is_active, is_deleted);
CREATE UNIQUE INDEX idx_products_qr  ON products(qr_token);

-- Auto-update updated_at
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_products_updated_at
    BEFORE UPDATE ON products
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
