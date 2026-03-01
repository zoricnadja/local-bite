-- Production Service - Initial Schema

-- Batch status enum check
CREATE TABLE IF NOT EXISTS production_batches (
    id           UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    farm_id      UUID         NOT NULL,
    name         VARCHAR(255) NOT NULL,
    process_type VARCHAR(100) NOT NULL,   -- e.g. "dimljenje", "fermentacija", "sušenje"
    start_date   DATE,
    end_date     DATE,
    status       VARCHAR(20)  NOT NULL DEFAULT 'PLANNED'
                     CHECK (status IN ('PLANNED', 'IN_PROGRESS', 'COMPLETED', 'CANCELLED')),
    notes        TEXT,
    is_deleted   BOOLEAN      NOT NULL DEFAULT FALSE,
    created_at   TIMESTAMP    NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMP    NOT NULL DEFAULT NOW()
);

-- Process steps within a batch (ordered)
CREATE TABLE IF NOT EXISTS process_steps (
    id             UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    batch_id       UUID          NOT NULL REFERENCES production_batches(id) ON DELETE CASCADE,
    farm_id        UUID          NOT NULL,
    step_order     INT           NOT NULL,
    name           VARCHAR(255)  NOT NULL,
    description    TEXT,
    duration_hours NUMERIC(8,2),
    temperature    NUMERIC(6,2),  -- celsius, nullable
    created_at     TIMESTAMP     NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMP     NOT NULL DEFAULT NOW(),
    UNIQUE (batch_id, step_order)
);

-- Raw materials used in a batch (many-to-many with quantity)
-- raw_material_id references Raw Materials service (cross-service — no FK)
CREATE TABLE IF NOT EXISTS batch_raw_materials (
    id                 UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    batch_id           UUID          NOT NULL REFERENCES production_batches(id) ON DELETE CASCADE,
    farm_id            UUID          NOT NULL,
    raw_material_id    UUID          NOT NULL,
    raw_material_name  VARCHAR(255)  NOT NULL,   -- denormalized snapshot at time of use
    material_type      VARCHAR(100)  NOT NULL,
    quantity_used      NUMERIC(12,3) NOT NULL,
    unit               VARCHAR(50)   NOT NULL,
    origin             VARCHAR(255),
    supplier           VARCHAR(255),
    UNIQUE (batch_id, raw_material_id)
);

CREATE INDEX idx_batches_farm_id   ON production_batches(farm_id);
CREATE INDEX idx_batches_status    ON production_batches(farm_id, status);
CREATE INDEX idx_steps_batch_id    ON process_steps(batch_id);
CREATE INDEX idx_brm_batch_id      ON batch_raw_materials(batch_id);
CREATE INDEX idx_brm_material_id   ON batch_raw_materials(raw_material_id);

-- Auto-update updated_at
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_batches_updated_at
    BEFORE UPDATE ON production_batches
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER trg_steps_updated_at
    BEFORE UPDATE ON process_steps
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
