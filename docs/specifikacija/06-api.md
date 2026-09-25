# 06 — HTTP API ugovori

[Sadržaj specifikacije](README.md) · [Polja DTO tipova](13-katalog-tipova.md)

## 1. Konvencije

Rute u tabelama su putanje preko glavnog Nginx gateway-a. Backend router-i koriste `{id}`, dok frontend koristi `:id`; obe notacije ovde znače promenljivi segment. Uloge: **O** vlasnik, **W** radnik, **C** kupac, **A** SYSTEM_ADMIN, **T** TRACEABILITY. „Business“ znači obavezan `business_id` iz tokena. „Sve“ znači četiri korisničke uloge, ne anonimni zahtev.

Standardni uspeh je 200 i `{ "data": rezultat }`, kreiranje obično 201 sa istim envelope-om, brisanje 204. Paginacija je uglavnom **ugnježđena**:

```json
{"data":{"data":[],"total":0,"page":1,"limit":20}}
```

Greške `AppError` koriste `{ "error": "poruka" }`: 400 neispravan zahtev, 401 nedostajući/nevažeći token, 403 nedozvoljena akcija, 404 nepostojeći zapis, 409 konflikt i 500 infrastruktura. Axum extractors i auth middleware mogu vratiti drugi oblik greške, pa jedinstven envelope nije apsolutan.

Login/register vraćaju `{ "token": "..." }` bez `data`. `/me` vraća User direktno i novi token u zaglavlju `x-session-token`. Mediji vraćaju binarne podatke. Nema verzionisanog `/v1` REST prefiksa niti OpenAPI specifikacije u pregledanom stablu; events imaju odvojenu verziju.

## 2. Auth

Prefiks gateway-a `/api/auth` uklanja se pre prosleđivanja auth servisu.

| Metoda i putanja | Ulaz / rezultat | Pristup i napomena |
|---|---|---|
| POST `/api/auth/register` | RegisterRequest → token, 201 | Javno; samo CUSTOMER/BUSINESS_OWNER |
| POST `/api/auth/login` | LoginRequest → token, 200 | Javno |
| GET `/api/auth/me` | User + x-session-token | Važeći JWT; nalog se čita iz baze |
| GET `/api/auth/users` | Niz User | Samo A |
| PUT `/api/auth/users/{id}` | UpdateUserRequest → User | Sam korisnik; za drugi nalog/ulogu proverava se `SYSTEM_ADMIN` |
| DELETE `/api/auth/users/{id}` | 204 | Sam korisnik osim W, ili A za drugi nalog |
| POST `/api/auth/businesses` | CreateBusinessRequest → `{business,token}`, 201 | O bez postojeće firme |
| GET `/api/auth/businesses/{id}` | Business | Stvarni vlasnik ili W iste firme |
| PUT `/api/auth/businesses/{id}` | UpdateBusinessRequest → Business | Vlasnik tog zapisa ili A |
| DELETE `/api/auth/businesses/{id}` | 204 | Vlasnik tog zapisa ili A |
| GET `/api/auth/businesses/{id}/workers` | WorkerOut niz | O koji je vlasnik navedene firme |
| POST `/api/auth/businesses/{id}/workers` | RegisterRequest → WorkerOut, 200 | Ista vlasnička provera; role se postavlja na W |
| GET `/api/auth/businesses/{id}/trace` | `{name}` | T sa odgovarajućim business_id |

GET kolekcije `/businesses` registrovan je samo za SYSTEM_ADMIN. Add-worker handler koristi profilna polja iz `RegisterRequest` i uvek nameće ulogu WORKER i firmu vlasnika.

## 3. Sirovine

Gateway `/api/raw-materials` mapira na servisni `/raw_materials`.

| Metoda i putanja | Ulaz / rezultat | Pristup |
|---|---|---|
| GET `/api/raw-materials` | Paginirane sirovine | O/W/A + Business |
| POST `/api/raw-materials` | CreateRawMaterialRequest → RawMaterial, 201 | O/W + Business |
| GET `/api/raw-materials/low-stock` | Niz RawMaterial u data | O/W + Business |
| GET `/api/raw-materials/{id}` | RawMaterial | O/W/A + Business i scoped upit |
| PUT `/api/raw-materials/{id}` | UpdateRawMaterialRequest → RawMaterial | O/W + Business |
| DELETE `/api/raw-materials/{id}` | Soft-delete, 204 | O + Business |
| POST `/api/raw-materials/{id}/adjust` | `{delta, reason?}` → RawMaterial | O/W + Business; reason nema trajni audit zapis u ovoj putanji |

Potrošnja je zamišljena kao međuservisna operacija, ali je dostupna i preko glavnog gateway-a sa korisničkim tokenom. Nema posebne service-only autentifikacije tih ruta.

## 4. Proizvodnja

Gateway `/api/productions/batches` mapira na `/batches`.

| Metoda i putanja | Ulaz / rezultat | Pristup |
|---|---|---|
| GET `/api/productions/batches` | Paginirane ProductionBatch vrednosti | O/W + Business |
| POST `/api/productions/batches` | CreateProductionBatchRequest → detalj, 201 | O/W + Business |
| GET `/api/productions/batches/{id}` | ProductionBatchResponse | O/W + Business |
| PUT `/api/productions/batches/{id}` | UpdateProductionBatchRequest → detalj | O/W + Business; statusni uslovi |
| DELETE `/api/productions/batches/{id}` | Soft-delete, 204 | O + Business; samo dozvoljeni status |
| GET `/api/productions/batches/{id}/trace` | Detalj serije | T sa subject=id i Business |
| GET `/api/productions/batches/{id}/steps` | Detalj serije, ne samo niz koraka | O/W + Business |
| POST `/api/productions/batches/{id}/steps` | CreateProcessStepRequest → ProcessStep, 201 | O/W + Business |
| PUT `/api/productions/batches/{id}/steps/{step_id}` | UpdateProcessStepRequest → ProcessStep | O/W + Business |
| DELETE `/api/productions/batches/{id}/steps/{step_id}` | 204 | O/W + Business |
| POST `/api/productions/batches/{id}/materials` | RawMaterialRequest → BatchRawMaterial, 201 | O/W + Business |
| DELETE `/api/productions/batches/{id}/materials/{material_id}` | 204 | O/W + Business; material_id je izvorna sirovina |

Ne postoji registrovan PUT materijala u seriji, iako poruka o duplikatu sugeriše korišćenje update endpointa.

## 5. Proizvodi

Gateway `/api/products` mapira na `/products`.

| Metoda i putanja | Ulaz / rezultat | Pristup / stvarno ponašanje |
|---|---|---|
| GET `/api/products` | Paginirani Product | C/A; C je prisilno ograničen na aktivnu ponudu |
| GET `/api/products/business` | Paginirani Product | Sve + Business; business scope se uzima iz tokena |
| POST `/api/products` | CreateProductRequest | O/W + Business, ali servis uvek vraća 400: kreirati kroz završetak proizvodnje |
| GET `/api/products/{id}` | Product | O/W svoja firma; C samo efektivno aktivan proizvod; A globalno |
| PUT `/api/products/{id}` | UpdateProductRequest → Product | O/W + Business, scoped izmena |
| DELETE `/api/products/{id}` | Soft-delete, 204 | O + Business |
| GET `/api/products/{id}/provenance` | ProvenanceResponse | Ista objektna autorizacija kao detalj proizvoda |
| POST `/api/products/{id}/image` | multipart polje image → Product | O/W + Business |
| GET `/api/products/{id}/image` | JPEG/PNG stream | JWT i objektna autorizacija |
| GET `/api/products/{id}/qr` | PNG stream | JWT i objektna autorizacija; može osvežiti QR putanju |
| POST `/api/products/{id}/qr/regenerate` | Product | O + sopstvena firma |
| GET `/api/products/public/{qr_token}` | PublicProvenance | Javno, aktivan/neobrisan proizvod i završena serija |
| GET `/api/products/public/{qr_token}/certificate.pdf` | application/pdf | Iste public provere |

Direktan /uploads ServeDir je uklonjen. UI preuzima medije kroz autorizovane image/qr putanje i privremene browser Blob URL-ove.

## 6. Porudžbine

| Metoda i putanja | Ulaz / rezultat | Pristup |
|---|---|---|
| GET `/api/orders` | Paginirani OrderResponse | O/W/A + Business |
| POST `/api/orders` | CreateOrderRequest → `{orders:[...]}`, 201 | Samo C; obavezan UUID Idempotency-Key |
| GET `/api/orders/user/{id}` | Paginirani OrderResponse | Samo C, id mora biti claims.sub |
| GET `/api/orders/{id}` | OrderResponse sa stavkama | C svoja porudžbina; O/W svoja firma; A globalno |
| PUT `/api/orders/{id}/status` | `{status}` → OrderResponse | O/W + Business; CANCELLED samo O |
| DELETE `/api/orders/{id}` | Soft-delete, 204 | O + Business, PENDING/CANCELLED |
| GET `/api/orders/analytics` | AnalyticsResponse | O/A + Business |

Ime i email kupca se prepisuju profilom iz auth-a, customer_id iz JWT-a. POST zahteva Idempotency-Key. 409 sa porukom Checkout is pending znači ponavljanje istog ključa; mrežni timeout ne znači da porudžbina nije nastala.

## 7. Query i health API

| Gateway / servisna ruta | Rezultat i pristup |
|---|---|
| GET `/api/queries/dashboard` → `/dashboard` | Sve, podaci prema ulozi; business za O/W |
| GET `/api/queries/producers` → `/producers` | Sve, ID i naziv proizvođača |
| GET `/internal/provenance/{id}` direktno read-models | Samo T, subject i business scope; nema javnog Nginx mapiranja |
| GET `/health/queries` | `{status,consumer_connected}` |
| GET `/health/materials`, `/health/products`, `/health/productions`, `/health/orders` | Tekst `ok`; nije dubinska DB/broker provera |
| GET `/health/auth` | Tekst ok; liveness, bez provere baze |

## 8. Filteri i paginacija

| Kolekcija | Parametri |
|---|---|
| Sirovine | page, limit, material_type, search |
| Serije | page, limit, status, process_type, search |
| Proizvodi | page, limit, product_type, search, business_id, is_active, active_only |
| Porudžbine | page, limit, status, search, business_id; business_id posebno koristi kupčev pregled |
| Analitika | from, to kao datum; prazna vrednost znači bez odgovarajuće granice |

Default page=1 i limit=20, najviše 100. Offset normalizuje page najmanje na 1, ali vraćeni `page` može zadržati originalnu vrednost. Limit nema dosledno donje ograničenje, pa negativni ili nulti parametri nisu uredno obrađeni. Search je tipično case-insensitive SQL pretraga naziva, a kod porudžbina imena/email-a kontakta.

## 9. Primeri zahteva

Slede primeri ugovora, ne skripta koja samostalno pokreće ceo scenario. UUID vrednosti treba zameniti stvarnim ID-evima, a prethodne statuse i zavisnosti zadovoljiti.

```json
{"name":"Mleko","material_type":"dairy","quantity":30,"unit":"l","origin":"Šumadija","harvest_date":"2026-09-14","received_date":"2026-09-15","expiry_date":"2026-09-20","low_stock_threshold":5}
```

POST sirovine zahteva JWT vlasnika ili radnika. Naredni primer je POST serije:

```json
{"name":"Sir — serija 01","process_type":"fermentation","start_date":"2026-09-17","raw_materials":[{"raw_material_id":"11111111-1111-4111-8111-111111111111","quantity_used":10,"unit":"l"}]}
```

Posle prelaska u IN_PROGRESS, PUT završetka:

```json
{"status":"COMPLETED","end_date":"2026-09-18","output_name":"Domaći sir","output_type":"cheese","output_quantity":2,"output_unit":"kg","output_expiry_date":"2026-09-30"}
```

Nakon asinhronog pojavljivanja proizvoda u skladištu, PUT proizvoda može imati `{"price":8,"is_active":true}`. Kupac zatim šalje:

```json
{"items":[{"product_id":"22222222-2222-4222-8222-222222222222","quantity":1}],"notes":"Preuzimanje po dogovoru"}
```

## 10. Preciznost API ugovora

Frontend deklaracije nisu uvek tačne kopije HTTP odgovora: pojedini servisi tipizuju pagination bez spoljnog data wrapper-a, a komponente koriste cast. Backend DTO i handler su merodavni. Predlog je OpenAPI ugovor sa jedinstvenim envelope-om, generisanim frontend tipovima i contract testovima.

Putanje kolekcija u Axum `nest` router-u i Nginx exact/trailing-slash pravilima treba proveriti i sa i bez završnog `/` u izvršenom okruženju; sama tabela ne potvrđuje oba oblika kao ekvivalentna.

## Interni stock ugovori

Bez Nginx javnog mapiranja: POST /internal/reservations/{id} prima stavke product_id, quantity i unit_price (decimalni stringovi), zahteva ORDER_STOCK sa sub=id. POST /internal/reservations/{id}/release/{business} dodatno zahteva business_id=business. Raw Materials POST /internal/production-consumption i /internal/production-consumption/release zahtevaju MATERIAL_STOCK token sa business scope-om. Obični korisnički JWT nije dovoljan. Oba tehnička tokena traju 60 sekundi.
