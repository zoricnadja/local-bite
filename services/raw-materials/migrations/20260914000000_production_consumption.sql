CREATE TABLE production_consumption (
    operation_id UUID PRIMARY KEY,
    farm_id UUID NOT NULL,
    raw_material_id UUID NOT NULL REFERENCES raw_materials(id),
    quantity NUMERIC(12,3) NOT NULL CHECK (quantity > 0),
    unit VARCHAR(50) NOT NULL,
    released BOOLEAN NOT NULL DEFAULT FALSE
);
