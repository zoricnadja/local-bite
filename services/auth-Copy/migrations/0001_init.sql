CREATE TABLE IF NOT EXISTS users (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email         VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role          VARCHAR(50) NOT NULL DEFAULT 'CUSTOMER'
    CHECK (role IN ('SYSTEM_ADMIN', 'FARM_OWNER', 'WORKER', 'CUSTOMER')),
    created_at    TIMESTAMP NOT NULL DEFAULT NOW()
    );

CREATE INDEX idx_users_email ON users(email);