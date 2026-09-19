CREATE TABLE stock_reservations (
 id UUID PRIMARY KEY,
 payload JSONB NOT NULL,
 created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE TABLE stock_reservation_items (
 reservation_id UUID NOT NULL REFERENCES stock_reservations(id),
 product_id UUID NOT NULL REFERENCES products(id),
 farm_id UUID NOT NULL,
 quantity NUMERIC(12,3) NOT NULL CHECK(quantity > 0),
 released BOOLEAN NOT NULL DEFAULT false,
 PRIMARY KEY(reservation_id,product_id)
);
CREATE INDEX stock_reservation_farm ON stock_reservation_items(reservation_id,farm_id);
