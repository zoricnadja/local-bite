-- migrations/001_create_users.sql
CREATE TABLE users (
                       id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                       email           TEXT NOT NULL UNIQUE,
                       password_hash   TEXT NOT NULL,
                       role            TEXT NOT NULL DEFAULT 'CUSTOMER',
                       farm_id         UUID,

    -- Required profile
                       first_name      TEXT NOT NULL,
                       last_name       TEXT NOT NULL,
                       address         TEXT NOT NULL,

    -- Optional profile
                       phone           TEXT,
                       photo_url       TEXT,
                       date_of_birth   DATE,

                       created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
                       updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- migrations/002_create_farms.sql
CREATE TABLE farms (
                       id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                       name        TEXT NOT NULL,
                       owner_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Required
                       address     TEXT NOT NULL,

    -- Optional
                       phone       TEXT,
                       description TEXT,
                       website     TEXT,

                       created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
                       updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Auto-update updated_at on row change
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER farms_updated_at
    BEFORE UPDATE ON farms
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
CREATE INDEX idx_users_email   ON users(email);
CREATE INDEX idx_users_farm_id ON users(farm_id);
CREATE INDEX idx_farms_owner   ON farms(owner_id);
