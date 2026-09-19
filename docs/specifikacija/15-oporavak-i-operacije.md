# 15 — Oporavak, retention i merljive performanse

[Sadržaj specifikacije](README.md)

## Backup skup i izvršena provera

Autoritativni skup čine pet write baza, `read_models` baza i `products_uploads`. Ključevi se čuvaju odvojeno, ne u backup arhivi. RabbitMQ se obnavlja iz konfiguracije; outbox je izvor ponovnog emitovanja dok se istorija čuva.

`node scripts/backup-verify.cjs` pravi custom-format dump bez owner/ACL podataka, uploads tar i manifest sa SHA-256. Svaki dump vraća u zasebnu PostgreSQL 15 instancu sa `--network none` i proverava `_sqlx_migrations`. Skripta ne radi restore preko aktivne baze. Izvršeni drill je uspešno vratio svih šest baza.

## Koordinisana obnova

1. Zaustaviti write saobraćaj i zabeležiti checkpoint.
2. Sačuvati pet write baza i uploads u istom maintenance prozoru; read-model je opcioni akcelerator.
3. Proveriti manifest i SHA-256.
4. Vratiti write baze i uploads.
5. Vratiti read-model ili napraviti praznu bazu i izvršiti `Replay-Events.ps1` do praznog backlog-a.
6. Proveriti health, pending outbox, DLQ, farm scope, javni trag i checkout/cancel smoke test.
7. Otvoriti write saobraćaj.

Individualni `pg_dump` nije globalni atomski snapshot. Za strogi cross-service RPO koristi se write freeze ili infrastrukturni PITR checkpoint.

## Retention politika

Pre brisanja mora postojati uspešan verifikovan backup.

| Podaci | Početno zadržavanje | Pravilo |
|---|---:|---|
| Neobjavljeni outbox | Bez isteka | Ne brisati dok `published_at IS NULL` |
| Objavljeni outbox | 90 dana | Backup i nema replay potrebe starijeg perioda |
| Projection receipts | Najmanje kao replayable outbox | Inače duplikat može ponovo biti primenjen |
| Tombstone/sequence | Dok je ID replayable | Brisati samo sa prekidom replay istorije |
| Završeni workflow poslovi | 1 godina | Pending/failed ne brisati automatski |
| Poslovni trag/porudžbine | Po pravnim zahtevima | Ne određivati tehničkim cleanup-om |
| Logovi | 30 dana početno | Bez tokena/payload-a |
| Backup | Dnevni 30 dana, mesečni 12 meseci | Konačni RPO/RTO odobrava vlasnik |

Retention posao prvo prikazuje broj kandidata (dry-run), zatim briše male batch-eve i beleži audit. Automatski destruktivni posao nije dodat jer rokovi nisu poslovno odobreni.

## Lokalni performance baseline

`scripts/benchmark.cjs` je izvršio po 100 read-only zahteva sa konkurentnošću 5: 500/500 uspešnih. Lokalni p95 je 8,15 ms za `/`, 2,00 ms auth health, 2,68 ms products health, 1,88 ms orders health i 2,05 ms queries health. Sirovi rezultat je u `docs/verification/performance.json`.

Ovo meri loopback, zagrejan Docker i uglavnom liveness. Ne meri autentifikovane liste, checkout, PDF, broker backlog ili realnu mrežu. Produkcioni test mora definisati podatke, ramp-up, p95/p99, error budget i maksimalno kašnjenje projekcije.

## Operativni alarmi

Pratiti starost neobjavljenog outbox-a, dubinu oba reda i DLQ-a, `consumer_connected`, broj nezavršenih checkout/material/stock-release poslova i starost najstarijeg posla. Health `ok` je liveness, ne dokaz praznog backlog-a ili dostupnosti svih zavisnosti.
