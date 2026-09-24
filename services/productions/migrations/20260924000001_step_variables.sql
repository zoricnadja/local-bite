ALTER TABLE process_steps ADD COLUMN variables JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(variables) = 'array');
UPDATE process_steps SET variables =
    CASE WHEN duration_hours IS NULL THEN '[]'::jsonb ELSE jsonb_build_array(jsonb_build_object('name', 'Duration (hours)', 'value', duration_hours::text)) END ||
    CASE WHEN temperature IS NULL THEN '[]'::jsonb ELSE jsonb_build_array(jsonb_build_object('name', 'Temperature (°C)', 'value', temperature::text)) END;
