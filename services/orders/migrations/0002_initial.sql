-- Orders Service - Initial Schema

CREATE TABLE IF NOT EXISTS orders (
    id           UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    farm_id      UUID          NOT NULL,
    customer_id  UUID,                             -- NULL = walk-in / anonymous
    customer_name  VARCHAR(255),                   -- denormalized snapshot
    customer_email VARCHAR(255),
    status       VARCHAR(20)   NOT NULL DEFAULT 'PENDING'
                     CHECK (status IN ('PENDING','CONFIRMED','SHIPPED','DELIVERED','CANCELLED')),
    total_price  NUMERIC(12,2) NOT NULL DEFAULT 0,
    notes        TEXT,
    is_deleted   BOOLEAN       NOT NULL DEFAULT FALSE,
    created_at   TIMESTAMP     NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMP     NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS order_items (
    id             UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id       UUID          NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id     UUID          NOT NULL,          -- cross-service ref, no FK
    product_name   VARCHAR(255)  NOT NULL,          -- snapshot at order time
    product_type   VARCHAR(100)  NOT NULL,
    unit_price     NUMERIC(12,2) NOT NULL,
    quantity       NUMERIC(12,3) NOT NULL,
    unit           VARCHAR(50)   NOT NULL,
    subtotal       NUMERIC(12,2) NOT NULL GENERATED ALWAYS AS (unit_price * quantity) STORED
);

CREATE INDEX idx_orders_farm_id    ON orders(farm_id);
CREATE INDEX idx_orders_customer   ON orders(customer_id);
CREATE INDEX idx_orders_status     ON orders(farm_id, status);
CREATE INDEX idx_orders_created    ON orders(farm_id, created_at DESC);
CREATE INDEX idx_items_order_id    ON order_items(order_id);
CREATE INDEX idx_items_product_id  ON order_items(product_id);

-- Auto-update updated_at
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_orders_updated_at
    BEFORE UPDATE ON orders
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
