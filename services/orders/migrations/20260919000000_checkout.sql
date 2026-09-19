CREATE TABLE checkout_jobs (
 id UUID PRIMARY KEY,
 customer_id UUID NOT NULL,
 request JSONB NOT NULL,
 payload JSONB NOT NULL,
 status TEXT NOT NULL DEFAULT 'PENDING' CHECK(status IN ('PENDING','COMPLETED','FAILED')),
 response JSONB,
 error TEXT,
 created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
 updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX checkout_pending ON checkout_jobs(created_at) WHERE status='PENDING';
CREATE TABLE order_stock_links (
 order_id UUID PRIMARY KEY REFERENCES orders(id) ON DELETE CASCADE,
 checkout_id UUID NOT NULL REFERENCES checkout_jobs(id),
 farm_id UUID NOT NULL
);
CREATE TABLE stock_release_jobs (
 order_id UUID PRIMARY KEY REFERENCES orders(id) ON DELETE CASCADE,
 checkout_id UUID NOT NULL,
 farm_id UUID NOT NULL,
 completed_at TIMESTAMPTZ,
 created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
