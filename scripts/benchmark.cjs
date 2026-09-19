// Read-only baseline, no authentication or business fixtures required.
const fs = require('node:fs');
const { performance } = require('node:perf_hooks');
async function measure(path, count = 100, concurrency = 5) {
  const samples = []; let next = 0, failures = 0;
  const start = performance.now();
  await Promise.all(Array.from({ length: concurrency }, async () => {
    while (next++ < count) {
      const t = performance.now();
      try { const r = await fetch('http://localhost' + path, { signal: AbortSignal.timeout(5000) }); await r.arrayBuffer(); if (!r.ok) failures++; }
      catch { failures++; }
      samples.push(performance.now() - t);
    }
  }));
  const elapsed = performance.now() - start;
  samples.sort((a, b) => a - b);
  const at = p => Number(samples[Math.ceil(p * samples.length) - 1].toFixed(2));
  return { path, count, concurrency, failures, p50Ms: at(.5), p95Ms: at(.95), p99Ms: at(.99), requestsPerSecond: Number((count * 1000 / elapsed).toFixed(2)) };
}
(async () => {
  const results = [];
  for (const path of ['/', '/health/auth', '/health/products', '/health/orders', '/health/queries']) results.push(await measure(path));
  const report = { measuredAt: new Date().toISOString(), environment: 'local Docker; loopback HTTP; warm services; not a production capacity estimate', results };
  fs.mkdirSync('docs/verification', { recursive: true });
  fs.writeFileSync('docs/verification/performance.json', JSON.stringify(report, null, 2));
  console.table(results);
  if (results.some(r => r.failures)) process.exitCode = 1;
})().catch(error => { console.error(error.message); process.exitCode = 1; });
