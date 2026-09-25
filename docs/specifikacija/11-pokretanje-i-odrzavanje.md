# 11 — Pokretanje, konfiguracija i održavanje

[Sadržaj specifikacije](README.md)

## 1. Pretpostavke okruženja

Standardna putanja je Docker Compose sa postojećim `.env`. Za frontend van Dockera potrebni su Node/npm, za backend Rust/Cargo i pristup bazama, a za products PDF dodatno Python 3, ReportLab i DejaVu fontovi. Konfiguracione vrednosti treba uskladiti sa deklaracijama projekta; dokument ne objavljuje stvarne tajne.

Kod svežeg checkout-a proveriti da li je frontend submodule preuzet (`.gitmodules`). Ako nije, inicijalizovati ga komandom `git submodule update --init --recursive` pre frontend build-a. Ne menjati postojeći checkout submodule-a tokom pregleda dokumentacije.

```powershell
docker compose up -d --build
docker compose ps
docker compose logs --tail 100 products-service read-models-service
```

Ove komande su operativno uputstvo; tokom izrade specifikacije nisu pokretane niti je postojeći stack restartovan.

## 2. Portovi i skladištenje

| Komponenta | Docker mreža / host |
|---|---|
| Glavni Nginx | Host 80 |
| Auth, materials, products, productions, orders | Očekivano 3001, 3002, 3003, 3004, 3005; host mapping iz env-a |
| Read-models | Interno 3006; preko Nginx query ruta |
| PostgreSQL auth/materials/products/productions/orders | Host 5432/5433/5434/5435/5436 → container 5432 |
| PostgreSQL read-models | 127.0.0.1:5437 → container 5432 |
| RabbitMQ | Host 5672 AMQP, 15672 management |
| pgAdmin | Host 5050 |
| Frontend development | Container 4200; Compose navodi samo container port, bez fiksnog host broja |
| Public gateway | 127.0.0.1:8081 → container 8080 |

Trajni podaci su u šest DB volumena, `rabbitmq_data` i `products_uploads`. Običan restart kontejnera ih ne briše. `docker compose down -v` bi uklonio volumene i podatke, pa nije deo uobičajenog održavanja ili ove procedure.

## 3. Konfiguracione promenljive

| Promenljiva / grupa | Potrošač i značenje |
|---|---|
| `POSTGRES_USER`, `POSTGRES_PASSWORD` | Compose inicijalizacija DB kontejnera |
| `AUTH_DATABASE_URL`, `RAW_MATERIALS_DATABASE_URL`, `PRODUCTS_DATABASE_URL`, `PRODUCTIONS_DATABASE_URL`, `ORDERS_DATABASE_URL` | Compose ih prosleđuje servisima kao DATABASE_URL |
| `DATABASE_URL` | Direktna Rust konekcija konkretnog servisa |
| `JWT_SECRET` | Deljena tajna JWT potpisa/provere |
| `AUTH_PORT`, `RAW_MATERIALS_PORT`, `PRODUCTS_PORT`, `PRODUCTIONS_PORT`, `ORDERS_PORT` | Compose vrednosti prosleđene kao PORT |
| `PORT` | Bind port Rust procesa |
| `*_RUST_LOG`, `RUST_LOG` | Nivo logovanja; Compose prevodi servisne promenljive |
| `RABBITMQ_DEFAULT_USER`, `RABBITMQ_DEFAULT_PASS` | Broker nalog i sastavljanje URL-a |
| `RABBITMQ_URL` | AMQP URL koji koristi common connect |
| `RAW_MATERIALS_SERVICE_URL` | Productions HTTP baza za sirovine |
| `PRODUCTS_SERVICE_URL` | Orders HTTP baza za proizvode |
| `AUTH_SERVICE_URL` | Orders profile fetch i preostali products business helper; fallback auth-service:3001 |
| `PRODUCTION_SERVICE_URL` | Products provera serije; jednina u imenu promenljive, fallback productions-service:3004 |
| `READ_MODELS_URL` | Products provenance query, fallback read-models-service:3006 |
| `PUBLIC_TRACE_URL` | Pun javni prefiks uključujući `/trace`; fallback http://localhost/trace |
| `UPLOADS_DIR` | Products direktorijum; fallback ./uploads |
| `CERTIFICATE_PYTHON` | Izvršni Python; fallback python3 |
| `CERTIFICATE_FONT_DIR` | Python font direktorijum; Linux DejaVu fallback |
| `SQLX_OFFLINE` | Build proverava SQL preko .sqlx metapodataka |
| `PGADMIN_DEFAULT_EMAIL`, `PGADMIN_DEFAULT_PASSWORD` | pgAdmin pristup |
| `NODE_ENV` | Prosleđeno frontend kontejneru; ne menja samo po sebi ng serve u production build |
| `RABBITMQ_MANAGEMENT_URL` | Prosleđen products-u u Compose-u; ne predstavlja aktivnu zavisnost novog outbox publisher-a |

## 4. Razlike default portova

Auth binary bez PORT-a bira 8080, productions binary bira 3003, products 3003, raw-materials 3002, orders 3005, read-models 3006. Nginx, međutim, očekuje auth na 3001 i productions na 3004. Za standardni Compose `.env` mora ih tako podesiti. EXPOSE 8080 u nekim Dockerfile-ovima je metadata i ne menja stvarni Rust listener.

Pri lokalnom pokretanju van Docker mreže treba zameniti Docker DNS hostove sa odgovarajućim localhost adresama. Menjanje samo PORT-a nije dovoljno ako Nginx upstream ili servisni URL i dalje pokazuje na staru vrednost.

## 5. Build i migracije

Svaki Rust Dockerfile kopira workspace, lockfile, servise, common i `.sqlx`, koristi BuildKit cache i kompajlira ceo workspace. Finalna slika dobija konkretan binary. SQL migracije ugrađene su kroz SQLx i izvršavaju se pri startu pool-a.

Za lokalnu proveru bez pristupa compile-time bazi:

```powershell
$env:SQLX_OFFLINE = 'true'
cargo check --workspace --locked
cargo test --workspace --locked
```

Uspeh zavisi od dostupnog toolchain-a, paketa i usklađene `.sqlx` kolekcije. Ignorisani DB testovi ne pokreću se ovim običnim test pozivom. Ako se menja šema ili SQL, pripremiti nove migracije i ažurirati SQLx metadata u razvojnom okruženju; ne menjati već primenjene migracije proizvoljno jer SQLx vodi njihovu istoriju i checksum.

Frontend:

```powershell
Set-Location frontend/local-bite-frontend
npm ci
npm run build
npm test -- --watch=false
```

Oba frontend Dockerfile-a koriste Node 22, npm ci sa lockfile-om i production build. Osnovni frontend služi statičke fajlove kroz Nginx na portu 80, bez source volume mount-a. Public allowlist prihvata hashovane main/polyfills/chunk JS i styles CSS fajlove, bez source map-a.

## 6. Health i startup

DB healthcheck-i koriste `pg_isready`, interval pet sekundi i čekanje pre starta servisnih zavisnosti. U komandama je korisnik `postgres` hardkodovan, dok se POSTGRES_USER konfiguriše: promena korisnika zahteva i proveru healthcheck-a.

Read-models i products čekaju da RabbitMQ servis bude pokrenut, ne dokazano spreman; reconnect loop popravlja deo tog prozora. Ostali publisher-i pokreću retry kada broker nije dostupan. HTTP `ok` health endpoint-i ne proveravaju svaki zavisni servis. Auth /health je implementiran kao liveness provera.

## 7. Javni QR pristup

```powershell
./scripts/Start-PublicQr.ps1 -Build
```

Skripta build-uje public gateway po potrebi, startuje gateway/tunnel, čita novonastalu trycloudflare adresu iz logova, upisuje `PUBLIC_TRACE_URL=<host>/trace` u `.env`, usklađuje products kontejner i restartuje lokalni Nginx. Dakle, ovo nije read-only skripta.

Dozvoljene javne rute su `/trace/<token>`, potrebni statički JS/CSS/favicon fajlovi i `/api/products/public/<token>` sa opcionalnim `/certificate.pdf`. Interni query API, administracija, source map-e i write endpoint-i nisu na allowlist-i. Cela aplikacija se ipak build-uje; mrežna allowlist ne znači da frontend bundle sadrži isključivo kod trace komponente.

Quick Tunnel adresa je privremena. Posle promene adrese treba ponovo otvoriti/preuzeti QR slike. Već odštampan QR ostaje vezan za staru adresu. Za stabilne deklaracije potrebno je planirati stalni domen i odgovarajuću tunel/hosting konfiguraciju.

```powershell
docker compose -f docker-compose.yml -f docker-compose.public.yml stop public-tunnel public-gateway
```

## 8. Monitoring i replay

Pratiti: dubinu oba consumer reda, broj poruka u projekcionom dead-letter redu, starost najstarijeg neposlatog outbox događaja, consumer_connected i logove retry-a. `asOf` dashboard-a je korisna informacija, ali nije pouzdan globalni backlog pokazatelj.

Read-only primer upita nad svakom domenskom bazom:

```sql
SELECT count(*) AS pending, min(occurred_at) AS oldest_pending
FROM integration_outbox
WHERE published_at IS NULL;
```

Posle obnove read baze, sa postojećom outbox istorijom:

```powershell
./scripts/Replay-Events.ps1
```

Skripta radi UPDATE outbox metapodataka u svih pet baza. Ne menja poslovne redove, ali ponovo šalje događaje i može opteretiti sistem. Ne izvršavati replay kao zamenu za razumevanje izgubljene domenske baze. Read model se može rekonstruisati iz dovoljne istorije; obrisani poslovni podaci i nestali upload fajlovi ne vraćaju se iz query projekcije ovom skriptom.

## 9. Backup/restore procedura

scripts/backup-verify.cjs pravi pg_dump svih šest baza i arhivu uploads-a, računa SHA-256 i vraća dump-ove u zaseban PostgreSQL kontejner bez mreže. Ne prepisuje aktivne baze. Za koordinisanu obnovu celog sistema obavezna je procedura iz [15 — Oporavak i operacije](15-oporavak-i-operacije.md). Operativni zahtevi:

1. Čuvati odvojene backup-e svih write baza, uploads volumena i konfiguracije bez javnog objavljivanja tajni.
2. Definisati da li i kako se čuva broker stanje, uz outbox kao izvor ponovnog emitovanja.
3. Uskladiti tačke oporavka između baza, naročito `production_outputs`, potrošnje i orders stanja.
4. Obnoviti vlasničke baze i medije, pokrenuti migracije, zatim rekonstruisati/obnoviti query bazu i kontrolisano ponoviti događaje.
5. Proveriti broj redova, reference, zalihe, QR/PDF i idempotency pre ponovnog prihvatanja write zahteva.

Retention pravila moraju razlikovati poslovne podatke, outbox istoriju, receipts i tombstone-e. Brisanje receipts/tombstone-a može promeniti bezbednost replay-a.

## 10. Materijali diplomskog rada

`typst-ftn` sadrži poglavlja, metapodatke, bibliografiju, slike/logotipe, akademske formulare i generisane PDF/DOCX dokumente. `build_local_bite_docx.py` i `build_ftn_template_docx.py` služe akademskom dokumentu, nisu pozivani iz backend servisa. Promene ove specifikacije ne menjaju automatski te artefakte.

Tekst rada treba uskladiti sa novom arhitekturom i statusom obrazaca pre sledećeg generisanja. Ova isporuka je Markdown specifikacija; nije regeneracija diplomskog rada ili sertifikata.
