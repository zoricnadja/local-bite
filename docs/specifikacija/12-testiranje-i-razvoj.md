# 12 — Testiranje, verifikacija i dalji razvoj

[Sadržaj specifikacije](README.md)

## Izvršene provere

Provere su izvršene 17/18. septembra 2026. nad lokalnim Docker okruženjem. Fixture-i koriste nasumične UUID/email vrednosti i brišu samo svoje zapise.

| Provera | Rezultat | Dokazano ponašanje |
|---|---|---|
| Rust `cargo test --release --workspace` kroz `scripts/Dockerfile.tests` | PASS | Workspace build i poison validator |
| Ignorisani DB consumption test | PASS | Atomski consume, scope, preciznost, idempotency i release |
| `scripts/test-certificate.py` | 2 PASS | Builder/Director redosled i PDF sa dugim Unicode sadržajem |
| `verify-role-stock.cjs` | PASS | Potrošnja, rollback, duplikati, nedovoljno stanje i role |
| `verify-farm-trace.cjs` | PASS | Sesija, Storage, public DTO/PDF, scope, konkurentna kupovina i cancel release |
| `verify-cqrs.cjs` | PASS | Pet publisher-a, rollback, duplicate/out-of-order, broker outage/recovery i query izolacija |
| `verify-output-events.cjs` | PASS | Stari događaj ne oživljava stock, legacy ne izmišlja stock, poison DLQ ne blokira red |
| `backup-verify.cjs` | PASS, 6/6 | Dump i restore svih baza u zasebnom PostgreSQL 15 kontejneru |
| Angular production Docker build | PASS | Tipovi, bundle i statički Nginx image |
| Lokalni HTTP baseline | 500/500 bez greške | Frontend i četiri health rute |

`verify-farm-trace.cjs` proverava i zabranu javnog admin/worker naloga, spoof identiteta, tuđi order 404, uklonjen decrement, nepromenljivu seriju, jednog pobednika za poslednju zalihu, replay checkout ključa, konflikt istog ključa sa drugim telom, vraćanje stock-a i recovery nakon restarta Orders servisa.

## Frontend testovi

Vitest testovi navigacije/dozvola, farm sesije, profila, field errors i `orders.service.spec.ts` za očuvanje checkout ključa prolaze: **6 test fajlova i 9 testova**. Production build takođe prolazi. Lockfile je regenerisan npm 10 verzijom koju koristi Node 22 image, kako bi sadržao Linux optional Rollup zavisnosti.

## Matrica pokrivenosti

| Oblast | Pokriveno | Još korisno |
|---|---|---|
| Identitet/autorizacija | Role whitelist, refresh farm tokena, order/product scope | Istek/revocation i admin lifecycle |
| Sirovine/proizvodnja | Atomski consume, rollback, immutable completed, output | Duži chaos test mrežnog partition-a |
| Prodaja | Konkurencija, idempotency, restart recovery, cancellation | Više Orders instanci i veliki checkout batch |
| Messaging/CQRS | Broker outage, replay, stale, poison, DLQ | RabbitMQ quorum/HA i višesatni backlog |
| Javni trag | Allowlist DTO, Unicode PDF, 404/409 retry | Accessibility i vizuelni PDF snapshot |
| Operacije | Backup/restore i read-only baseline | Periodični drill, PITR i produkcioni load test |

## Pokretanje

```powershell
docker compose up -d nginx
node scripts/verify-role-stock.cjs
node scripts/verify-farm-trace.cjs
node scripts/verify-cqrs.cjs
node scripts/verify-output-events.cjs
node scripts/benchmark.cjs
node scripts/backup-verify.cjs
```

CQRS skripta namerno zaustavlja i vraća RabbitMQ i source servise. Farm/trace skripta kontroliše restart Orders servisa. Koristiti izdvojeno razvojno okruženje.

## Završene stavke prethodnog plana

1. Zatvoreni su registracione uloge, order/product objektna autorizacija i javni decrement.
2. Implementirani su rezervacija, trajni checkout, recovery i povraćaj pri otkazivanju.
3. Implementirani su nepromenljivost proizvodnje, trajna kompenzacija, sequence provera i output DLQ.
4. Usklađeni su frontend wrapper-i, decimalni wire tipovi, admin naziv i auth health.
5. Fixture-i koriste proizvodni tok i pokrivaju konkurentne/failure scenarije.
6. Postoje Public DTO, redakcija logova, backup/restore provera i lokalni performance baseline.
7. Strategy/Simple Factory i PDF Builder/Director su aktivni i testirani.

## Sledeća iteracija

Prioriteti su medijski tok O-01/O-02, automatski retention uz odobren period, centralne metrike/correlation ID, CI sa istim Docker test skupom i produkcioni load test sa dogovorenim SLO/RPO/RTO vrednostima. Plaćanje ili AI prognoza dolaze posle tih operativnih osnova.
