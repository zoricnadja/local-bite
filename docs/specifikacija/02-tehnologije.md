# 02 — Tehnologije i njihova primena

[Sadržaj specifikacije](README.md)

Verzije ispod su zahtevi deklarisani u manifestima ili tagovi Docker slika iz repozitorijuma, a ne tvrdnja o najnovijim dostupnim verzijama. Konkretno zaključane Rust verzije određuje `Cargo.lock`, a npm zavisnosti `package-lock.json` kada je prisutan. Deklarisana zavisnost ne znači nužno i aktivnu upotrebu.

## 1. Rust i Cargo workspace

Svih šest backend procesa napisano je u Rust-u, sa `edition = "2021"`. Root `Cargo.toml` grupiše servise i `libs/common` u jedan workspace, koristi resolver 2 i centralizuje većinu verzija. Svaki servis i dalje ima sopstveni paket, `main.rs`, konfiguraciju, bazu i izvršni fajl.

Rust strukture opisuju modele i DTO objekte; `Option<T>` predstavlja odsustvo vrednosti, a `Result<T, E>` uspeh ili grešku. UUID se prenosi kao tip umesto neproverenog stringa. Vlasništvo i borrowing ograničavaju nekontrolisano deljenje memorije, dok `Arc<T>` deli repozitorijume i servise između HTTP zahteva. To rešava upravljanje memorijom procesa, ali samo po sebi ne rešava konkurentne SQL izmene ili distribuirane transakcije.

Konstrukcija objekata je eksplicitna: `main.rs` formira pool, repozitorijume i servise, a Axum ih prosleđuje kroz `Extension<Arc<...>>`. Uobičajeni tok je handler → servis → repozitorijum. Kod pojedinih novih integracionih putanja SQL se nalazi direktno u handleru ili consumer-u, pa slojevitost nije apsolutna.

Docker build koristi `cargo build --locked --release --workspace`: `--locked` zahteva postojeći lockfile, `--release` pravi optimizovane izvršne fajlove, a `--workspace` kompajlira sve članove. Ne postoji numerički pinovana verzija Rust toolchain-a u prikazanim Dockerfile-ovima (`FROM rust`).

## 2. Axum 0.8.8 i HTTP sloj

Axum je HTTP framework. `Router` povezuje metode i putanje sa async funkcijama, a `nest` dodaje prefiks servisa: `/products`, `/orders`, `/batches`, `/raw_materials`. Auth rute su definisane direktno u `main.rs`.

Projektna primena extractora:

| Extractor / mehanizam | Primena |
|---|---|
| `Path<Uuid>` | Tipiziran ID proizvoda, porudžbine, serije ili firme |
| `Query<ListQuery>` | Paginacija, pretraga, tip/status, filter proizvođača |
| `Json<Request>` | Deserijalizacija ulaznog JSON-a u DTO |
| `Multipart` | Upload polja `image` |
| `Extension<Arc<Service>>` | Dohvatanje prethodno konstruisanog servisa |
| `AuthClaims` | Projektni extractor koji proverava Bearer JWT |
| `IntoResponse` | Standardni JSON odgovori, greške, fajlovi i PDF |

Handler treba da izdvoji identitet, proveri ulogu i pozove poslovnu operaciju. Na primer, `materials::create` dobija gazdinstvo iz JWT-a, a ne iz korisničkog JSON-a. Neispunjen uslov postaje `AppError`. `#[debug_handler]` pomaže proveri potpisa handlera pri kompilaciji; nije sistem revizionog logovanja.

`http` 1.4.0 daje tipove metoda, statusa i zaglavlja. `hyper` 1.8.1 je deklarisan na nivou workspace-a; ne treba ga opisivati kao posebno implementirani aplikacioni server mimo Axum sloja.

## 3. Tokio 1, futures 0.3 i asinhroni rad

`#[tokio::main]` inicijalizuje runtime, a `tokio::net::TcpListener` prihvata HTTP konekcije. Async SQL i REST omogućavaju da proces obrađuje druge zahteve dok čeka I/O. Outbox i consumers rade kao pozadinski taskovi preko `tokio::spawn`.

`tokio::try_join!` paralelno pribavlja nezavisne rezultate, npr. korake i sirovine serije ili više agregata analitike. Orders koristi `futures::future::join_all` za učitavanje proizvoda i naknadno umanjenje količina. `StreamExt` čita AMQP deliveries. Paralelizacija ne uvodi zajedničku transakciju preko servisa.

Timeout postoji na većini međuservisnih poziva: obično pet sekundi za čitanje, deset za potrošnju sirovina, dvadeset za PDF. Outbox i consumers ponovo uspostavljaju vezu posle tri sekunde. Neki putevi, poput orders poziva `/me`, nemaju eksplicitni projektni timeout. Sinhrona obrada slike i pojedine filesystem operacije nisu izdvojene u `spawn_blocking`, što može opteretiti runtime pri velikom broju zahteva.

`tokio-util` 0.7 omogućava `ReaderStream` za slanje slike/QR fajla kao HTTP stream-a, bez ručnog pravljenja celog byte niza u handleru.

## 4. PostgreSQL 15

Compose definiše šest PostgreSQL 15 instanci: po jednu za svaku servisnu bazu. Svaka ima poseban Docker volume. Razdvojene baze daju lokalno vlasništvo i nezavisne migracije, ali ne i međubazne strane ključeve.

Konkretne mogućnosti PostgreSQL-a korišćene u projektu:

- `UUID` primarni ključevi i `gen_random_uuid()` kao default na glavnim tabelama.
- `NUMERIC(12,3)` za količine i `NUMERIC(12,2)` za cene i iznose.
- `DATE`, `TIMESTAMP` i `TIMESTAMPTZ` za različite kategorije vremena.
- `JSONB` za integracione snimke i univerzalni read model.
- `CHECK` ograničenja statusa, pozitivne potrošnje i pozitivne konačne izlazne količine.
- `UNIQUE` za email, QR token, redni broj koraka u seriji i materijal u seriji.
- `GENERATED ALWAYS AS (...) STORED` za subtotal stavke porudžbine.
- Trigger-i za `updated_at` i kreiranje outbox događaja u istoj transakciji.
- Parcijalni indeksi za neposlate događaje i vidljive projekcije.
- `FOR UPDATE`, uslovni `UPDATE` i advisory locks za određene konkurentne tokove.
- `ON CONFLICT` za deduplikaciju potrošnje, receipta i projekcije.

`projection_entities` je aplikaciona tabela projekcija, a ne PostgreSQL `MATERIALIZED VIEW`. Dashboard trenutno učitava skupove projekcionih redova i deo agregacija radi u Rust-u; time se olakšava objedinjavanje podataka, ali nije potvrđena skalabilnost za velike baze.

## 5. SQLx 0.8.6

SQLx je asinhroni SQL alat; projekat koristi eksplicitni SQL, ne ORM mapiranje kroz aktivne entitete. Uključene su podrške za PostgreSQL, Tokio/Rustls, UUID, chrono i BigDecimal.

`query!` i `query_as!` proveravaju strukturu SQL rezultata pri kompilaciji na osnovu baze ili spremljenih metapodataka. Folder `.sqlx` omogućava offline build sa `SQLX_OFFLINE=true`. Promena šeme ili makro upita može zahtevati osvežavanje ovih metapodataka; samo promeniti Rust strukturu nije dovoljno.

Postoje i dinamički `query`, `query_as::<_, T>` i `query_scalar`, gde je provera ugovora odložena do izvršavanja. Parametri se vezuju preko `.bind`, što izbegava direktno ubacivanje korisničkih vrednosti u SQL tekst. Dinamički konstruktor filtera za proizvode ubacuje kontrolisani SQL izraz, a korisničke vrednosti i dalje vezuje kao parametre.

Pet domenskih servisa pravi pool sa najviše pet konekcija i timeout-om pribavljanja od pet sekundi. Read-models koristi najviše deset. Pri startu se izvršava `sqlx::migrate!().run(...)`; migracije nisu odvojen manuelni obavezni korak za standardan start servisa.

## 6. Serde 1 i serde_json 1

Serde izvodi JSON serializaciju/deserijalizaciju iz struktura. DTO zahteva je odvojen od SQL modela: `CreateProductionBatchRequest`, na primer, sadrži listu zahtevanih sirovina, dok tabela serije ne sadrži tu listu u koloni. DTO odgovora `ProductionBatchResponse` sastavlja seriju, korake i sirovine u jedan objekat.

`serde_json::Value` koristi se za integracioni payload i projekcije, gde više vrsta entiteta deli skladišnu strukturu. To olakšava integraciju, ali zahteva validaciju verzije, izvora i identiteta. `projector::valid` proverava envelope, ali ne kompletnu semantičku šemu svih polja svakog entiteta.

## 7. UUID, chrono i decimalni brojevi

`uuid` 1 generiše v4 identifikatore za poslovne entitete, QR token i idempotency operacije. `chrono` 0.4.44 daje `NaiveDate`, `NaiveDateTime` i UTC timestamp tipove. Razlika je bitna: `NaiveDateTime` nema vremensku zonu, dok auth i integracioni timestamp-i koriste zonu.

`bigdecimal` 0.4.10 mapira decimalne baze. DTO zahtevi često koriste `f64`, pa pre upisa pretvaraju vrednost kroz string u BigDecimal. Odgovori nekad imaju decimalne stringove, a nekad eksplicitno konvertovane `f64`. To znači da frontend i integracije ne treba da pretpostave jedinstven JSON tip brojeva. `rust_decimal` 1.40.0 je deklarisan u orders manifestu, ali dominantni model cena i SQLx integracija koristi BigDecimal.

## 8. JWT i Argon2

`jsonwebtoken` 9 kodira i dekodira JWT sa podrazumevanim header-om/validacijom biblioteke i deljenim `JWT_SECRET`. Claims sadrži korisnički ID, email, ulogu, gazdinstvo i vremena. API zahtevi prenose `Authorization: Bearer ...`; token nije serverska sesija u tabeli.

`argon2` 0.5 hash-uje lozinku uz nasumičnu so generisanu pomoću `rand` 0.8.5 i proverava login hash. U bazi je `password_hash`, ne čista lozinka; polje je isključeno iz serializovanog User odgovora. Argon2 ne predstavlja enkripciju koja se kasnije dešifruje.

Tehnički `TRACEABILITY` token traje 60 sekundi. Izdaje ga products servis za čitanje konkretne projekcije ili proveru konkretne serije. Sve usluge dele simetričnu tajnu: kompromitovan servis sa tom tajnom može izdavati tokene, pa je to važna granica poverenja.

## 9. RabbitMQ i Lapin 2.5.5

`rabbitmq:3-management-alpine` pruža AMQP broker i administratorski HTTP interfejs. `lapin` je Rust AMQP klijent, pinovan na `=2.5.5` u common/products/read-models paketima.

Aktuelna topologija:

| Element | Ime | Namena |
|---|---|---|
| Durable topic exchange | `local_bite.events.v1` | Distribucija integracionih događaja |
| Projekcioni red | `local_bite.read_models.v1` | Sve promene preko binding-a `#` |
| Dead-letter red | `local_bite.read_models.dead.v1` | Nevalidni projekcioni događaji |
| Red izlaza proizvodnje | `local_bite.production_outputs.v1` | Binding `productions.production_batches.*` |

Objavljivanje koristi persistent delivery mode, `mandatory` i publisher confirms. Tek potvrđen događaj dobija `published_at`. Potrošač projekcija potvrđuje poruku posle commit-a projekcije i receipta. To daje at-least-once isporuku sa idempotentnom primenom u projekciji; ne daje globalnu exactly-once transakciju.

## 10. Reqwest 0.13.2

Reqwest pravi JSON REST zahteve između servisa. Productions čita i troši sirovine, products proverava seriju i čita read model, orders učitava proizvode i umanjuje stanje, a handler kreiranja orders-a učitava profil. Korisnički token se prosleđuje tamo gde je potreban korisnički scope; za poreklo se izdaje poseban tehnički token.

Baze URL-ova čitaju se iz okruženja uz Docker DNS podrazumevane vrednosti. Nema zasebnog service discovery registra. Klijenti se često konstruišu po pozivu, pa reuse konekcija nije centralno organizovan kroz zajednički pool klijenata.

## 11. Slike i QR

`image` 0.25.9 dekodira i smanjuje sliku na najviše 800×800 uz očuvan odnos stranica, pa je čuva kao JPEG pod nasumičnim nazivom. U manifestu su eksplicitno uključeni PNG i JPEG, uz isključene default features. MIME allowlist prihvata i WebP, ali feature `webp` nije naveden: to je neusaglašenost, ne potvrđena podrška WebP-u.

`qrcode` 0.14.1 generiše QR iz URL-a `PUBLIC_TRACE_URL/<qr_token>` i renderuje PNG sa quiet zone. Naziv fajla sadrži token i hash URL-a; promena javnog hosta zato daje novu putanju slike. Hash služi keširanju naziva, ne digitalnom potpisivanju porekla.

Slike i QR fajlovi su u lokalnom uploads direktorijumu, mapiranom na volume. Baza čuva relativnu putanju. Ne koristi se S3 ili drugi object storage.

## 12. Python 3 i ReportLab

Python nije zaseban mikroservis. Products pokreće subprocess pomoću `tokio::process::Command`, ubacuje sadržaj `assets/certificate.py` kroz `include_str!`, prosleđuje strukturisan JSON na stdin i preuzima PDF sa stdout-a. Ne konstruiše shell komandu od korisničkog teksta.

ReportLab Platypus sastavlja paragrafe, tabele, razmake, zaglavlje i podnožje na A4 formatu. DejaVu fontovi podržavaju znakove u nazivima i poreklu. Korisnički tekst prolazi XML escaping pre Paragraph renderovanja. PDF se vraća uz `application/pdf`, a proces ima timeout i prekid pri odustajanju.

Verzija ReportLab-a zavisi od Debian paketa `python3-reportlab`; nije posebno pinovana requirements fajlom. `CERTIFICATE_PYTHON` i `CERTIFICATE_FONT_DIR` omogućavaju prilagođavanje lokalnog izvršavanja.

## 13. Angular 21 i TypeScript 5.9

Frontend je SPA sa standalone komponentama. Manifest navodi većinu Angular paketa kao `^21.0.0`, Material i animations `^21.2.0`, CLI/build `^21.0.5`, TypeScript `~5.9.2`. App se pokreće kroz `bootstrapApplication`, a providers se nalaze u `app.config.ts`.

Lazy `loadComponent` i `loadChildren` odlažu učitavanje feature-a do navigacije. Komponente održavaju lokalno stanje pomoću `signal` i izvode vrednosti preko `computed`. Template koristi `@if` i `@for`, uz poneke starije strukturne direktive. Nema uvedenog NgRx store-a.

Reactive Forms služe unosu i validaciji, a FormsModule i `ngModel` jednostavnijim filterima. Zajednički validatori proveravaju neprazan tekst, URL i redosled datuma. `FieldErrorsDirective` objedinjuje inline poruke i ARIA atribute.

Angular Material postoji kao zavisnost i kroz zajedničke importe; značajan deo UI-ja je ručno stilizovan CSS-om i HTML-om, pa aplikaciju nije tačno opisati kao potpuno zasnovanu na Material komponentama. `zone.js` je uključen u build polyfills.

## 14. RxJS 7.8 i HttpClient

API servisi vraćaju `Observable`, transformišu odgovor preko `map`, a sporedne efekte poput čuvanja tokena preko `tap`. Dashboard koristi `interval(5000)` i `takeUntilDestroyed` za obustavu osvežavanja pri uništenju komponente. Javni QR prikaz koristi `retry` ograničen na 409.

Funkcionalni HTTP interceptor dodaje JWT zaglavlje. Route guard osvežava profil pre ulaska na zaštićenu rutu, a permission guard i `appCan` odlučuju koje akcije UI prikazuje. Ovi mehanizmi poboljšavaju korisnički tok; serverska autorizacija ostaje nezavisna obaveza.

## 15. Node.js, npm, Vitest i jsdom

Node služi Angular alatima i regresionim `.cjs` skriptama. Frontend Docker koristi `node:20-alpine`, a manifest navodi package manager `npm@11.6.2`. To su odvojene deklaracije; Dockerfile ne instalira eksplicitno baš tu npm verziju. Ako postoji lockfile izvršava `npm ci`, inače `npm install`.

Vitest `^4.0.8` i jsdom `^27.1.0` koriste se preko Angular unit-test builder-a. Angular TestBed i HttpTestingController proveravaju komponente, dozvole i HTTP sesiju bez pravog backend-a. Node skripte koriste ugrađene `fetch`, `assert`, `crypto` i `child_process` za integracione provere Docker okruženja.

## 16. Docker Compose, Nginx i Cloudflare tunnel

Compose opisuje procese, varijable, mrežu, volumene i zavisnosti. Višefazni Rust Docker build odvaja kompajler od runtime slike. BuildKit cache ubrzava registry/target slojeve. Runtime je Debian Trixie slim; products dodatno instalira Python, ReportLab i fontove.

Nginx je reverse proxy sa `/api` rutama i proxy-em za statički Angular production container. U javnom režimu zaseban Nginx služi production bundle i allowlist QR API-ja. Cloudflared Quick Tunnel daje privremenu HTTPS adresu za pristup telefonom. To nije trajna hosting konfiguracija niti HA klaster.

## 17. Ostale biblioteke i alati

| Tehnologija | Primena i granica |
|---|---|
| `thiserror` 2.0.18 | Izvođenje `AppError` tipa i poruka grešaka |
| `anyhow` 1.0.102 | Kontekst infrastrukturnih grešaka i propagacija internog uzroka |
| `tracing` 0.1 / subscriber 0.3 | Strukturisani log pozivi i `RUST_LOG` filter; nema potvrđene distribuirane trace platforme |
| `tower-http` 0.5 | CORS; deklarisan trace feature nije dokaz instaliranog TraceLayer-a |
| `dotenvy` 0.15 | Učitavanje lokalnog `.env` pri startu |
| pgAdmin | Ručna administracija baza; nije deo korisničkih poslovnih tokova |
| PowerShell | Start javnog QR pristupa i replay outbox događaja |
| Typst | Izvorni materijal diplomskog rada, van runtime arhitekture aplikacije |
| Python DOCX skripte | Generisanje akademskih dokumenata; odvojeno od PDF sertifikata proizvoda |

Primarni izvori: root i servisni `Cargo.toml`, `Cargo.lock`, frontend `package.json`, `angular.json`, `Dockerfile*`, `docker-compose*.yml`, `libs/common/src`, `services/products/assets/certificate.py`.
