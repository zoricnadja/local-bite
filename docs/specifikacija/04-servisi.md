# 04 — Backend servisi i zajednička biblioteka

[Sadržaj specifikacije](README.md)

Portovi 3001–3006 predstavljaju očekivanja Nginx/Docker mreže. `PORT` u pojedinim binarnim servisima ima drugi fallback; videti dokument 11.

## 1. Auth — identitet, gazdinstva i zaposleni

**Paket:** `services/auth`; **baza:** `auth_db`; **mrežni port:** 3001.

`AuthService` registruje korisnika, hash-uje lozinku, proverava login, izdaje i verifikuje JWT. `UserService` čita/menja/briše profile. `BusinessService` upravlja gazdinstvom i registruje radnika sa fiksnom ulogom `WORKER` i `business_id` iz putanje vlasničkog zahteva.

`UserRepository` upisuje i pronalazi naloge po email-u/ID-u i menja business assignment. `BusinessRepository` radi sa gazdinstvima i upitom radnika. `auth_middleware` proverava JWT i stavlja korisnički UUID u extensions; drugi handleri dodatno koriste common `AuthClaims` za kompletne claims.

`/me` čita korisnika iz baze i vraća novi `x-session-token`, pa promenjen business assignment može da se prenese u naredne zahteve. Vraća User direktno, za razliku od većine ruta koje koriste `{data: ...}`. `User.password_hash` ima `skip_serializing`.

Objavljuje promene `users` i `businesses` preko outbox-a. Korisnički payload sadrži samo ID, role i business assignment; firma samo ID, naziv i vlasnički ID. Ne šalje lozinku, adresu ili profil u projekciju.

Registracija dozvoljava samo CUSTOMER i BUSINESS_OWNER. Admin provere koriste SYSTEM_ADMIN; GET /businesses vezan je za administratorski handler, a /health vraća ok. Kreiranje firme i dodela vlasniku su jedna transakcija sa zaključanim korisnikom; brisanje odvaja sve članove. To nije distribuirano brisanje ostalih domena.

## 2. Raw Materials — evidencija ulaznih sirovina

**Paket:** `services/raw-materials`; **baza:** `raw_materials_db`; **port:** 3002.

`RawMaterialService` implementira CRUD, validaciju datuma, pregled niske zalihe i korekciju. `RawMaterialRepository` izvršava upite sa `business_id`, filtrima i soft-delete uslovom. `adjust_quantity` je relativna korekcija `delta`, a ne unos apsolutne nove količine; repository čuva nenegativnost uslovnim SQL-om.

`handlers/consumption.rs` je transakcioni podmodul za proizvodni utrošak. Prima niz operacija, sortira ih po sirovini, vezuje UUID operacije za količinu/jedinicu/gazdinstvo i izvršava umanjenje samo ako postoji dovoljno stanja. Sve stavke zahteva pripadaju jednoj lokalnoj transakciji.

Ponovljen operation ID sa istim podacima ne troši ponovo. Isti ID sa izmenjenim podacima ili već oslobođenom operacijom daje konflikt. `release` postavlja `released=true` i vraća količinu samo jednom. Ledger nije opšti dnevnik svih ručnih korekcija; čuva proizvodnu potrošnju.

Objavljuje samo promene `raw_materials`, ne ledger `production_consumption`. Datumi prijema/berbe/isteka podržani su u unosu i snimku proizvodne sirovine. Ne radi pretvaranje jedinica niti vodi zasebne lotove unutar jednog raw_material zapisa.

## 3. Productions — serije, proces i proizvodni izlaz

**Paket:** `services/productions`; **baza:** `productions_db`; **port:** 3004.

`BatchService` kreira i prikazuje seriju, validira status, datume i izlaz, a detalj sastavlja iz batch, step i material podataka. `StepService` upravlja redosledom i parametrima koraka. `RawMaterialsService` je kombinacija poslovne koordinacije i HTTP adaptera za sirovine.

Kreiranje serije sa sirovinama najpre proverava sve ulaze. Potom u lokalnoj transakciji upisuje seriju i istorijske snimke. Svaki snimak dobija isti UUID koji se koristi kao operation ID kod raw-materials potrošnje. Ako udaljena potrošnja ili lokalni commit ne uspeju, pokušava `release` kompenzaciju.

Dodavanje sirovine postojećoj seriji dodatno zaključava red serije pomoću `FOR UPDATE`, ponovo proverava status i tek tada izvršava upis i potrošnju. Brisanje veze trigger-om kreira trajni release posao; recovery petlja vraća sirovinu.

Završetak zahteva definisan izlaz i blokira naredne batch izmene. Repository koristi očekivani raniji status pri update-u radi detekcije konkurentne promene. `process_steps` ima SQL jedinstvenost `(batch_id, step_order)`. Dodavanje koraka je blokirano za završene/otkazane serije, a DB trigger štiti sve izmene koraka i materijala završene serije.

Objavljuje `production_batches`, `process_steps` i `batch_raw_materials`. Putanja `/batches/{id}/trace` prima samo `TRACEABILITY` token vezan za taj batch i firmu; koristi se pri aktiviranju proizvoda. Za javno čitanje porekla proizvoda glavna putanja više ne radi direktan fan-out ka ovom servisu.

## 4. Products — skladište, ponuda, mediji i poreklo

**Paket:** `services/products`; **baza:** `products_db`; **port:** 3003; **fajlovi:** `uploads/images`, `uploads/qr`.

| Deo | Odgovornost |
|---|---|
| `ProductService` | Liste, detalj, izmena, brisanje, provera aktiviranja; rezervacije su u reservations.rs |
| `ProductRepository` | SQL nad proizvodima i putanjama medija; filtriranje efektivne aktivnosti |
| `output_consumer` | Kreiranje gotove zalihe iz događaja završene serije |
| `ProvenanceService` | Aktuelni proizvod + read-model projekcija porekla |
| `ImageService` | Provera MIME-a, obrada i zamena slike, SQL putanja |
| `QrService` | Dobavljanje/obnavljanje PNG-a koji vodi na javni frontend |
| `CertificateService` | Dohvatanje porekla i pokretanje Python PDF renderera |
| `product_policy` | Aktivne Strategy/Factory validacije izmene i proizvodnog izlaza |
| `utils` | Obrada slika, QR kodiranje i filesystem putanje |

Output consumer upisuje poslednje stanje serije u `production_states`. Za `COMPLETED` sa izlaznom količinom formira proizvod u transakciji, uz advisory lock po batch-u i jedinstvenu vezu u `production_outputs`. Stare završene serije bez podatka o prinosu ne stvaraju izmišljenu količinu.

Efektivna aktivnost na listi je `products.is_active` zajedno sa postojanjem završene, neobrisane serije iste firme u `production_states`. Skladište je filter efektivne aktivnosti false; nije poseban servis niti posebna tabela skladišnih stavki. Cena 0 i `is_active=false` su početno stanje automatskog izlaza.

Public QR tok dodatno čita stvarni proizvod iz products baze. Proverava `is_active`, `is_deleted` i QR token; proveru završene serije radi preko lokalne asinhrone projekcije. Zbog toga „trenutna provera vidljivosti“ važi za sam product zapis, a ne za sve udaljene domene.

`QrService.regenerate` ponovo generiše fajl sa postojećim tokenom. Komentar koji govori o novom tokenu nije u skladu sa kodom. GET slike i QR-a zahteva JWT i istu objektnu autorizaciju kao proizvod; direktni /uploads je uklonjen.

## 5. Orders — porudžbine i prodajna analitika

**Paket:** `services/orders`; **baza:** `orders_db`; **port:** 3005.

`OrderService` validira stavke, pribavlja product snapshot-e, grupiše ih po gazdinstvu, računa iznose, upisuje orders/items, menja statuse i sastavlja odgovore. `product_service.rs` sadrži REST funkciju za čitanje proizvoda. Profile za kontakt pri kreiranju čita handler direktno iz auth servisa.

`OrderRepository` čuva zaglavlje i agregatne upite; `OrderItemRepository` čuva stavke i upit top proizvoda. Liste porudžbina dohvaćaju stavke dodatnim upitom za svaku porudžbinu, pa postoji N+1 obrazac upita. Izračunati `subtotal` generiše baza, a `total_price` sastavlja aplikacija pre upisa.

Revenue i top-products uključuju samo `DELIVERED`, dok total-orders i raspodela statusa uključuju neobrisane porudžbine svih statusa. Datum filtera je datum kreiranja, ne datum isporuke. Mesečni prihod je grupisan po mesecu kreiranja isporučene porudžbine. Baza nema posebno polje valute; UI prikazuje evro.

Objavljuje orders i order_items snapshot-e. Kontakt ime, email i napomene porudžbine izostavljeni su iz outbox payload-a. Detalj proverava kupca ili firmu, a logovi ne ispisuju ceo Order objekat. checkout.rs koordinira trajnu nameru, idempotentnu rezervaciju, upis porudžbina i release.

## 6. Read Models — query API i projekcije

**Paket:** `services/read-models`; **baza:** `read_models_db`; **port:** 3006, bez direktnog host HTTP port mapping-a u osnovnom Compose-u.

`projector.rs` validira event envelope i transakciono upisuje receipt i projekciju. Primena se odbija ako je event stariji od postojećeg stanja; `deleted` ostaje sačuvan kao tombstone. Nevalidan JSON ili nepodržana envelope kombinacija ide u dead-letter red. Greška baze vraća poruku u red i prekida vezu pre ponovnog pokušaja.

`queries.rs` daje tri poslovne putanje:

- `/dashboard`: kupcu najviše pet njegovih poslednjih porudžbina; vlasniku i radniku njegovu firmu; administratoru globalni pregled. Radniku su finansijska i orders statistika vraćene kao nule.
- `/producers`: lista ID-a i naziva gazdinstava za autentifikovane korisnike.
- `/internal/provenance/{id}`: projekcija proizvoda, proizvođača, serije, koraka i sirovina; tehnički token mora imati isti subject i business scope.

`/health` prijavljuje `status` i `consumer_connected`. To ne dokazuje praznu zaostalu kolonu poruka ili potpunu ažurnost podataka.

## 7. Common — zajednički infrastrukturni ugovori

`libs/common` nije zaseban mrežni servis. Ugrađuje se u backend binarne fajlove i sadrži:

| Modul | Uloga |
|---|---|
| `errors` | AppError/AppResult i HTTP mapiranje grešaka |
| `response` | 200/201 JSON envelope i 204 odgovor |
| `paginated_response` | `data`, `total`, `page`, `limit` |
| `jwt` | Claims i encode/decode helper-i |
| `middleware` | AuthClaims extractor, require_role i require_business |
| `models` | Role enum i prevod naziva |
| `events` | Envelope, AMQP topologija i outbox relay |
| `service_auth` | Kratkotrajni tehnički tokeni i provera operacije |

`require_business` samo zahteva da claims ima `business_id`: nema administratorski bypass niti dodatni upit baze. Poslovni kod zato mora posebno odlučiti kako se administratoru bira opseg.
