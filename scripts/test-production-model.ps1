$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path $PSScriptRoot -Parent

# Test in a transaction and isolated schema; leave application data unchanged.
$catalogFixture = @'
INSERT INTO type_catalog (farm_id, name) VALUES ('00000000-0000-0000-0000-000000000001', 'Custom type')
ON CONFLICT (farm_id, lower(name)) WHERE farm_id IS NOT NULL
DO UPDATE SET name = type_catalog.name;
INSERT INTO type_catalog (farm_id, name) VALUES ('00000000-0000-0000-0000-000000000001', 'CUSTOM TYPE')
ON CONFLICT (farm_id, lower(name)) WHERE farm_id IS NOT NULL
DO UPDATE SET name = type_catalog.name;
INSERT INTO type_catalog (farm_id, name) VALUES ('00000000-0000-0000-0000-000000000002', 'Other farm type');
DO $$ BEGIN
    IF (SELECT count(*) FROM type_catalog WHERE farm_id = '00000000-0000-0000-0000-000000000001' AND lower(name) = 'custom type') <> 1 THEN
        RAISE EXCEPTION 'Duplicate type was created';
    END IF;
    IF EXISTS (SELECT 1 FROM type_catalog WHERE (farm_id IS NULL OR farm_id = '00000000-0000-0000-0000-000000000001') AND name = 'Other farm type') THEN
        RAISE EXCEPTION 'Farm type isolation failed';
    END IF;
    IF NOT EXISTS (SELECT 1 FROM type_catalog WHERE farm_id = '00000000-0000-0000-0000-000000000001' AND name = 'Legacy custom') THEN
        RAISE EXCEPTION 'Legacy type missing';
    END IF;
END $$;
'@

foreach ($service in @('productions', 'raw-materials')) {
    $table = if ($service -eq 'productions') { 'production_batches' } else { 'raw_materials' }
    $column = if ($service -eq 'productions') { 'process_type' } else { 'material_type' }
    $sql = "BEGIN; CREATE SCHEMA production_model_test; SET LOCAL search_path = production_model_test;`n"
    $sql += "CREATE TABLE $table (farm_id UUID, $column TEXT);`n"
    $sql += "INSERT INTO $table VALUES ('00000000-0000-0000-0000-000000000001', 'Legacy custom');`n"
    $sql += Get-Content -Raw -LiteralPath (Join-Path $repoRoot "services/$service/migrations/20260924000000_type_catalog.sql")
    $sql += $catalogFixture
    if ($service -eq 'productions') {
        $sql += @'
CREATE TABLE process_steps (id INT, duration_hours NUMERIC, temperature NUMERIC);
INSERT INTO process_steps VALUES (1, 48, 0), (2, NULL, NULL);
'@
        $sql += Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'services/productions/migrations/20260924000001_step_variables.sql')
        $sql += @'
DO $$ BEGIN
    IF (SELECT variables FROM process_steps WHERE id = 1) <> '[{"name":"Duration (hours)","value":"48"},{"name":"Temperature (°C)","value":"0"}]'::jsonb THEN
        RAISE EXCEPTION 'Legacy variables not preserved';
    END IF;
    IF (SELECT variables FROM process_steps WHERE id = 2) <> '[]'::jsonb THEN
        RAISE EXCEPTION 'Absent legacy variables should produce empty array';
    END IF;
END $$;
UPDATE process_steps SET variables = '[{"name":"Humidity","value":"75 %"},{"name":"Culture","value":"starter A"}]'::jsonb WHERE id = 1;
'@
    }
    $sql += "`nROLLBACK;"
    $OutputEncoding = [System.Text.UTF8Encoding]::new($false)
    $sql | docker exec -i "$service-db" sh -c 'psql -U $POSTGRES_USER -d $POSTGRES_DB -v ON_ERROR_STOP=1'
    if ($LASTEXITCODE -ne 0) { throw "Migration checks failed for $service" }
}
