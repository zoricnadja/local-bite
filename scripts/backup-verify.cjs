// Non-destructive backup and restore drill. Never restores over application databases.
// Run from repository root: node scripts/backup-verify.cjs
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const { execFileSync } = require('node:child_process');
const services = ['auth', 'raw-materials', 'productions', 'products', 'orders', 'read-models'];
const destination = path.resolve('tmp/backups', new Date().toISOString().replace(/[:.]/g, '-'));
const scratch = 'local-bite-restore-' + crypto.randomUUID();
const docker = (args, options = {}) => execFileSync('docker', args, { maxBuffer: 256 * 1024 * 1024, timeout: 120000, ...options });
const manifest = { createdAt: new Date().toISOString(), consistency: 'individual database snapshots; coordinated maintenance required for cross-service recovery', files: [], verified: false };
function save(name, bytes) {
  fs.writeFileSync(path.join(destination, name), bytes, { flag: 'wx' });
  manifest.files.push({ name, bytes: bytes.length, sha256: crypto.createHash('sha256').update(bytes).digest('hex') });
}
async function main() {
  fs.mkdirSync(destination, { recursive: true });
  for (const service of services) {
    // PostgreSQL reads the configured username inside the container, without disclosing secrets.
    save(service + '.dump', docker(['exec', service + '-db', 'sh', '-c', 'exec pg_dump -U "$POSTGRES_USER" -d "$POSTGRES_DB" -Fc --no-owner --no-acl']));
  }
  save('uploads.tar', docker(['exec', 'products-service', 'tar', '-C', '/app/uploads', '-cf', '-', '.']));
  fs.writeFileSync(path.join(destination, 'manifest.json'), JSON.stringify(manifest, null, 2));
  let started = false;
  try {
    docker(['run', '-d', '--rm', '--name', scratch, '--network', 'none', '-e', 'POSTGRES_HOST_AUTH_METHOD=trust', 'postgres:15']);
    started = true;
    let ready = false;
    for (let i = 0; i < 30; i++) {
      try { docker(['exec', scratch, 'pg_isready', '-U', 'postgres'], { stdio: 'pipe' }); ready = true; break; }
      catch { await new Promise(resolve => setTimeout(resolve, 500)); }
    }
    if (!ready) throw Error('Isolated restore database did not become ready');
    for (const service of services) {
      const db = service.replaceAll('-', '_') + '_db';
      docker(['exec', scratch, 'createdb', '-U', 'postgres', db]);
      docker(['exec', '-i', scratch, 'pg_restore', '-U', 'postgres', '-d', db, '--no-owner', '--no-acl', '--exit-on-error'], { input: fs.readFileSync(path.join(destination, service + '.dump')) });
      const migrations = docker(['exec', scratch, 'psql', '-U', 'postgres', '-d', db, '-Atc', 'SELECT count(*) FROM _sqlx_migrations WHERE success']).toString().trim();
      if (!(Number(migrations) > 0)) throw Error(service + ': restored migrations missing');
      console.log('PASS isolated restore: ' + service + ' (' + migrations + ' migrations)');
    }
    manifest.verified = true;
    manifest.verifiedAt = new Date().toISOString();
    fs.writeFileSync(path.join(destination, 'manifest.json'), JSON.stringify(manifest, null, 2));
    console.log('Backup and restore report: ' + path.relative(process.cwd(), destination));
  } finally {
    if (started) docker(['stop', scratch]);
  }
}
main().catch(error => { console.error(error.message); process.exitCode = 1; });
