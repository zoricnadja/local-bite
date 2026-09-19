CREATE TABLE consumption_tombstones(operation_id UUID PRIMARY KEY,farm_id UUID NOT NULL,created_at TIMESTAMPTZ NOT NULL DEFAULT now());
