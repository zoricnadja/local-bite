-- Auth Service - Initial Schema

CREATE TABLE IF NOT EXISTS farms (
                                     id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name       VARCHAR(255) NOT NULL,
    owner_id   UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
    );

CREATE TABLE IF NOT EXISTS users (
                                     id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email         VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role          VARCHAR(50) NOT NULL DEFAULT 'CUSTOMER'
    CHECK (role IN ('SYSTEM_ADMIN','FARM_OWNER','WORKER','CUSTOMER')),
    farm_id       UUID REFERENCES farms(id) ON DELETE SET NULL,
    created_at    TIMESTAMP NOT NULL DEFAULT NOW()
    );

-- Add FK from farms.owner_id → users.id (added after users table exists)
ALTER TABLE farms ADD CONSTRAINT fk_farms_owner
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE;

CREATE INDEX idx_users_email   ON users(email);
CREATE INDEX idx_users_farm_id ON users(farm_id);
CREATE INDEX idx_farms_owner   ON farms(owner_id);
