$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path $PSScriptRoot -Parent
$sql = @'
BEGIN;
CREATE SCHEMA step_status_test;
SET LOCAL search_path = step_status_test;
CREATE TABLE production_batches (id INT PRIMARY KEY, process_type TEXT NOT NULL, status TEXT NOT NULL DEFAULT 'PLANNED', start_date DATE);
CREATE TABLE process_steps (id INT PRIMARY KEY, batch_id INT REFERENCES production_batches(id));
INSERT INTO production_batches VALUES (1, 'legacy', 'PLANNED', NULL), (2, 'legacy', 'COMPLETED', NULL), (3, '', 'PLANNED', NULL);
INSERT INTO process_steps VALUES (1, 1), (2, 1), (3, 2);
CREATE FUNCTION protect_existing_steps() RETURNS trigger AS $$ BEGIN
    IF EXISTS (SELECT 1 FROM production_batches WHERE id = NEW.batch_id AND status = 'COMPLETED') THEN
        RAISE EXCEPTION 'Production is immutable' USING ERRCODE = '23514';
    END IF;
    RETURN NEW;
END $$ LANGUAGE plpgsql;
CREATE TRIGGER immutable_steps BEFORE UPDATE ON process_steps FOR EACH ROW EXECUTE FUNCTION protect_existing_steps();
'@
$sql += Get-Content -Raw -LiteralPath (Join-Path $repoRoot 'services/productions/migrations/20260924000002_step_status.sql')
$sql += @'
CREATE FUNCTION expect_blocked(command TEXT) RETURNS VOID AS $$ BEGIN
    BEGIN
        EXECUTE command;
    EXCEPTION WHEN check_violation THEN RETURN;
    END;
    RAISE EXCEPTION 'Expected rejection: %', command;
END $$ LANGUAGE plpgsql;
DO $$ BEGIN
    IF (SELECT status FROM process_steps WHERE id = 3) <> 'COMPLETED' THEN RAISE EXCEPTION 'Historical completion lost'; END IF;
END $$;
SELECT expect_blocked('UPDATE production_batches SET status = ''COMPLETED'' WHERE id = 3');
SELECT expect_blocked('UPDATE production_batches SET status = ''IN_PROGRESS'' WHERE id = 1');
SELECT expect_blocked('UPDATE process_steps SET status = ''COMPLETED'' WHERE id = 1');
UPDATE process_steps SET status = 'IN_PROGRESS' WHERE id = 1;
DO $$ BEGIN
    IF (SELECT status FROM production_batches WHERE id = 1) <> 'IN_PROGRESS' THEN RAISE EXCEPTION 'Batch did not start'; END IF;
    IF (SELECT start_date FROM production_batches WHERE id = 1) IS NULL THEN RAISE EXCEPTION 'Missing start date'; END IF;
END $$;
SELECT expect_blocked('UPDATE production_batches SET status = ''COMPLETED'' WHERE id = 1');
UPDATE process_steps SET status = 'COMPLETED' WHERE id = 1;
SELECT expect_blocked('UPDATE production_batches SET status = ''COMPLETED'' WHERE id = 1');
UPDATE process_steps SET status = 'IN_PROGRESS' WHERE id = 2;
SELECT expect_blocked('UPDATE process_steps SET status = ''PLANNED'' WHERE id = 2');
UPDATE process_steps SET status = 'COMPLETED' WHERE id = 2;
DO $$ BEGIN
    IF (SELECT status FROM production_batches WHERE id = 1) <> 'IN_PROGRESS' THEN RAISE EXCEPTION 'Batch completed without product confirmation'; END IF;
END $$;
UPDATE production_batches SET status = 'COMPLETED' WHERE id = 1;
SELECT expect_blocked('INSERT INTO process_steps (id, batch_id) VALUES (4, 1)');
SELECT expect_blocked('DELETE FROM process_steps WHERE id = 1');
UPDATE production_batches SET status = 'CANCELLED' WHERE id = 3;
SELECT expect_blocked('INSERT INTO process_steps (id, batch_id) VALUES (5, 3)');
ROLLBACK;
'@
$OutputEncoding = [System.Text.UTF8Encoding]::new($false)
$sql | docker exec -i productions-db sh -c 'psql -U $POSTGRES_USER -d $POSTGRES_DB -v ON_ERROR_STOP=1'
if ($LASTEXITCODE -ne 0) { throw 'Step workflow tests failed' }
