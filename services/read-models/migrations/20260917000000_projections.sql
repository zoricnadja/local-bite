CREATE TABLE projection_entities (
 source TEXT NOT NULL, entity_type TEXT NOT NULL, entity_id UUID NOT NULL,
 sequence BIGINT NOT NULL, deleted BOOLEAN NOT NULL, data JSONB NOT NULL,
 projected_at TIMESTAMPTZ NOT NULL DEFAULT now(),
 PRIMARY KEY(source,entity_type,entity_id)
);
CREATE INDEX projection_farm ON projection_entities(entity_type,(data->>'farm_id')) WHERE NOT deleted;
CREATE INDEX projection_customer ON projection_entities((data->>'customer_id')) WHERE entity_type='orders' AND NOT deleted;
CREATE INDEX projection_batch ON projection_entities(entity_type,(data->>'batch_id')) WHERE NOT deleted;
CREATE INDEX projection_qr ON projection_entities((data->>'qr_token')) WHERE entity_type='products' AND NOT deleted;
CREATE TABLE projection_receipts (
 source TEXT NOT NULL, sequence BIGINT NOT NULL,
 applied_at TIMESTAMPTZ NOT NULL DEFAULT now(), PRIMARY KEY(source,sequence)
);
