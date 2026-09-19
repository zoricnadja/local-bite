ALTER TABLE production_batches
 ADD COLUMN output_name TEXT, ADD COLUMN output_type TEXT, ADD COLUMN output_unit TEXT,
 ADD COLUMN output_quantity DOUBLE PRECISION CHECK(output_quantity > 0 AND output_quantity < 'Infinity'::float8),
 ADD COLUMN output_expiry_date DATE;
ALTER TABLE batch_raw_materials ADD COLUMN harvest_date DATE;
