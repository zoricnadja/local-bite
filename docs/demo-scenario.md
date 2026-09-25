# Local Bite — scenario za snimanje demonstracije

Scenario je usklađen sa pregledanim izvornim kodom 25. 9. 2026. Ovo je priprema snimka, ne izveštaj o izvršenom testiranju aplikacije. Tekst u navodnicima možeš direktno da govoriš. Putanje služe kao redosled otvaranja fajlova u editoru.

## 1. Kratka lista funkcionalnosti koje treba pokazati

1. **Prijava, profil i poslovni nalog** — vlasnik, podaci poslovanja i dodavanje radnika.
2. **Sirovine** — unos, poreklo, datumi, količina, pretraga i korekcija zalihe.
3. **Proizvodna serija** — izbor sirovina i automatsko umanjenje njihovih zaliha.
4. **Koraci proizvodnje** — redosled, proizvoljni parametri i prelazi Planned → In progress → Completed.
5. **Plan i rezultat proizvodnje** — više izlaznih proizvoda iz jedne serije; planirana i stvarna količina.
6. **Proizvodi** — stanja Production → Storage → On sale, cena i slika.
7. **Poreklo** — QR kod, javna stranica bez prijave i PDF dokument.
8. **Kupovina** — kupčev pregled ponude, filter proizvođača, korpa i porudžbina.
9. **Obrada porudžbine** — potvrda, slanje i isporuka; na posebnoj porudžbini otkazivanje i povrat zalihe.
10. **Dashboard, analitika i uloge** — prihod isporučenih porudžbina, najprodavaniji proizvodi i razlika između vlasnika i radnika.

## 2. Priprema i redosled snimanja

Predlog trajanja je 10–15 minuta demonstracije i 20–25 minuta prolaska kroz kod. Ako profesor traži kraći snimak, skrati unos podataka i broj fajlova koje otvaraš, a zadrži objašnjenja celog poslovnog toka.

Pripremi vlasnički i kupčev nalog u dva odvojena profila pregledača, nalog radnika, jednu sliku proizvoda i nekoliko ranije isporučenih porudžbina za analitiku. Registraciju i dodavanje radnika dovoljno je pokazati jednom. Za prikaz izolacije poslovanja potreban je i nalog vlasnika drugog poslovanja.

Koristi jedan dosledan primer: jagode i šećer kao sirovine, seriju „Džem od jagoda — demo“, korake „Priprema“, „Kuvanje“ i „Pakovanje“. Parametri koraka mogu biti „Temperatura: 100 °C“ i „Trajanje: 30 min“ — primeri unetih podataka, a ne preporučeni tehnološki postupak. Planiraj 20 komada proizvoda, a pri završetku evidentiraj 18. Drugi izlaz može biti drugo pakovanje istog proizvoda. Za izlaze izaberi podržan tip iz ponuđenog izbora i jedinicu pcs.

Probaj ceo tok jednom pre snimanja. Asinhrone promene mogu postati vidljive posle kratkog čekanja ili osvežavanja. Za javni prikaz dovoljan je privatni prozor sa QR adresom. Ako skeniraš telefonom, adresa u QR kodu mora biti dostupna telefonu; localhost na telefonu nije tvoj računar.

## 3. Priča tokom demonstracije aplikacije

### 3.1. Uvod, prijava i poslovanje

**Na ekranu:** prijava, dashboard, profil poslovanja, zaposleni.

„Local Bite je aplikacija za praćenje sirovina, proizvodnje, gotovih proizvoda i porudžbina lokalnih proizvođača. Glavni cilj je da proizvod povežem sa serijom u kojoj je nastao i sirovinama koje su korišćene, tako da njegov put može da se prati do kupca.

Prvo se prijavljujem kao vlasnik. Vlasnik upravlja svojim poslovanjem i dodaje radnike. Različita poslovanja koriste istu aplikaciju, ali poslovni podaci imaju pripadnost određenu identifikatorom business_id. Kupac ima drugačiji prikaz, namenjen ponudi i sopstvenim porudžbinama.“

### 3.2. Sirovine i zalihe

**Na ekranu:** napravi sirovinu, pokaži poreklo, datume i raspoloživu količinu; zatim listu i filtere.

„Ovde evidentiram sirovine: naziv, tip, količinu, jedinicu mere i podatke o poreklu i datumima. Mogu da pretražujem evidenciju i korigujem zalihu. Količine se proveravaju i na serveru, tako da provera ne zavisi samo od formulara.

Ovi podaci kasnije ulaze u sledljivost proizvoda. Kada sirovinu povežem sa proizvodnjom, čuva se snimak njenih podataka u trenutku korišćenja.“

### 3.3. Serija i koraci

**Na ekranu:** kreiraj seriju sa sirovinama, pokaži promenu zalihe, dodaj korake i parametre, pokreni prvi korak.

„Sada kreiram proizvodnu seriju i biram količine sirovina koje koristi. Te količine se skidaju sa raspoloživog stanja već pri povezivanju sa serijom.

Proces opisujem uređenim koracima. Svaki korak ima naziv, opis i listu parametara u obliku naziva i vrednosti. Zato različite vrste proizvodnje mogu da beleže različite informacije, bez posebnog modela za svaki postupak.

Korak prolazi kroz planirano stanje, rad i završetak. Pokretanje koraka automatski pokreće seriju. Završavanje cele proizvodnje postaje dostupno kada postoji najmanje jedan korak i svi koraci budu završeni.“

### 3.4. Plan, završetak i proizvodi

**Na ekranu:** sačuvaj plan dva izlaza, prikaži Products sa stanjem Production; završi korake, koriguj konačnu količinu i završi seriju.

„Jedna serija može da proizvede više proizvoda. Najpre unosim planirane izlaze, sa količinom, jedinicom i ostalim podacima. U pregledu proizvoda oni se pojavljuju u stanju Production.

Kada završim sve korake, potvrđujem stvarne rezultate. Ovde menjam planiranih 20 komada na stvarno dobijenih 18. Sistem čuva prethodni plan, pa se vidi razlika između planiranog i ostvarenog.

Završetak serije šalje događaj koji servis proizvoda obrađuje i prevodi izlaze u Storage. Pošto se ovo odvija asinhrono, prikaz može kratko da sačeka obradu događaja. Završena serija je zaštićena od naknadnih izmena.“

### 3.5. Prodaja i poreklo

**Na ekranu:** proizvodi, filter stanja, cena, slika, Move to On sale, detalj, QR, javna stranica i PDF.

„Gotov proizvod najpre ostaje u skladištu. Za prodaju određujem pozitivnu cenu i prebacujem ga u On sale. Mogu da dopunim opis i sliku, dok se izmerena proizvedena količina ne menja običnim uređivanjem proizvoda.

Na detalju mogu da prikažem QR kod. Njegova adresa vodi na javnu stranicu porekla, koju kupac može da otvori bez naloga. Ovde se vide proizvod, proizvođač, proizvodna serija, koraci i poreklo korišćenih sirovina.

Iz istih javnih podataka generiše se PDF. Dokument predstavlja evidenciju podataka koje je proizvođač uneo; ne predstavlja nezavisnu potvrdu kvaliteta proizvoda.“

### 3.6. Kupovina i obrada

**Na ekranu:** kupčev nalog, proizvodi, korpa, kreirana porudžbina; zatim vlasnički nalog i statusi porudžbine.

„Sada prelazim na kupca. Kupac pregleda dostupnu ponudu, filtrira proizvođača i bira proizvode i količine. Server proverava proizvode, cene i raspoloživost, a zatim rezerviše količine i kreira porudžbine. Ako su proizvodi iz više poslovanja, zahtev se razdvaja po poslovanju.

Vlasnik obrađuje svoju porudžbinu kroz statuse Pending, Confirmed, Shipped i Delivered. Prelazi su ograničeni poslovnim pravilima. Na posebnom primeru mogu da pokažem otkazivanje i vraćanje rezervisane količine, koje se izvršava kroz pouzdan pozadinski posao.“

### 3.7. Analitika i dozvole

**Na ekranu:** analitika i dashboard; kratko radnički nalog.

„Analitika prikazuje broj porudžbina, raspodelu statusa, najprodavanije proizvode i prihod od isporučenih porudžbina. Dashboard daje pregled prilagođen ulozi.

Radnik ima operativne mogućnosti za proizvodnju i proizvode, dok upravljanje poslovanjem i finansijska analitika nisu deo njegovih frontend dozvola. Time završavam primer od prijema sirovine do prodaje i prikaza porekla.“

## 4. Priča tokom prolaska kroz kod

### 4.1. Struktura i arhitektura

**Otvori:** `Cargo.toml`, `docker-compose.yml`, `nginx/nginx.conf`.

„Projekat ima Angular frontend i šest backend mikroservisa: auth, raw-materials, productions, products, orders i read-models. Rust workspace okuplja njihove pakete i zajedničku biblioteku common.

Backend koristi Rust, Axum za HTTP sloj, Tokio za asinhrono izvršavanje i SQLx za rad sa PostgreSQL bazama. Svaki servis ima svoju bazu i migracije. Podaci se između servisa razmenjuju preko API poziva i događaja, bez direktnog pristupa tuđim tabelama.

Docker Compose opisuje pokretanje servisa, baza, RabbitMQ-a i frontenda. Nginx je ulazna tačka koja API putanje prosleđuje odgovarajućim servisima. RabbitMQ prenosi događaje; pgAdmin je pomoćni alat za pregled baza.“

### 4.2. Frontend: organizacija i navigacija

**Otvori redom, unutar `frontend/local-bite-frontend/src`:**

- `main.ts`, `app/app.config.ts`, `app/app.routes.ts`.
- `app/app.ts` i `app/app.html`.
- Direktorijume `app/features`, `app/core` i `app/shared`.

„Frontend je jedna Angular aplikacija sa ekranima organizovanim po poslovnim oblastima. Main pokreće aplikaciju, konfiguracija registruje router i HTTP klijent, a glavna komponenta sadrži navigaciju i prostor za trenutno aktivnu stranicu.

Rute koriste odloženo učitavanje komponenti i grupa ekrana. Features sadrži poslovne ekrane, core sesiju i API servise, a shared modele, validatore i zajedničke komponente. Javna trace ruta je odvojena od ruta koje zahtevaju prijavu. Stara storage ruta preusmerava na Products, gde je skladište jedno od stanja proizvoda.“

### 4.3. Frontend: sesija, dozvole i HTTP

**Otvori u `src/app`:** `core/auth/auth.service.ts`, `auth.guard.ts`, `permissions.ts`, `can.directive.ts`, `core/interceptors/auth.interceptor.ts` i `core/services/production.service.ts`.

„AuthService čuva token i korisnika i izlaže reaktivno stanje pomoću signala. Iz korisnika se izvode uloga i pripadnost poslovanju. Guard pri navigaciji osvežava profil preko servera, a interceptor dodaje Bearer token HTTP zahtevima.

Permission matrica određuje koje ekrane i akcije korisnik vidi. Guard kontroliše navigaciju, a appCan direktiva prikaz delova interfejsa. Backend zasebno proverava dozvole, jer sakrivanje dugmeta samo po sebi ne štiti podatke.

Angular API servisi kriju URL-ove i HTTP pozive od komponenti. Oni nisu dodatni backend mikroservisi: izvršavaju se u pregledaču. Pored ProductionService, postoje adapteri za sirovine, proizvode, porudžbine, korisnike, poslovanje i proizvođače.“

### 4.4. Frontend: jedan tok detaljno

**Otvori u `src/app`:** `features/production/detail/production-detail.component.ts`, `shared/models/production.models.ts`, `shared/field-errors.directive.ts`, `core/services/orders.service.ts` i `features/public-trace/public-trace.component.ts`.

„Na detalju proizvodnje vidi se tipična organizacija ekrana. Signali čuvaju učitane podatke, stanje slanja i greške. Forma za korake koristi niz kontrola za promenljive parametre. Funkcija canComplete proverava da li postoje koraci i da li su svi završeni. Izmene se šalju ProductionService adapteru, a server ponovo proverava pravila.

Zajednički modeli opisuju očekivane podatke, a direktiva za greške ujednačava prikaz validacije. Isti princip koriste ostali poslovni ekrani.

Kod poručivanja frontend čuva ključ idempotentnosti nerešenog zahteva, tako da ponavljanje posle neizvesnog odgovora može da koristi isti ključ. Javna stranica porekla posebno obrađuje odgovor da se podaci još sinhronizuju i kratko ponavlja učitavanje.“

### 4.5. Zajednička biblioteka i slojevi backenda

**Otvori:** `libs/common/src/middleware.rs`, `jwt.rs`, `errors.rs`, `response.rs`, `service_auth.rs`; zatim `services/raw-materials/src/main.rs` i `routes.rs`.

„Common je deljena biblioteka, ne zaseban servis. Ona sadrži JWT ugovore, proveru uloge i poslovnog opsega, standardne odgovore i mapiranje aplikativnih grešaka na HTTP odgovore. Za interne operacije postoje kratkotrajni tehnički tokeni ograničeni na konkretnu namenu.

Uobičajen tok zahteva je ruta, handler, poslovni servis i repository. Handler obrađuje HTTP podatke i autorizaciju, servis poslovna pravila, a repository SQL. DTO definiše ulazni ili izlazni ugovor, dok model predstavlja podatke domena ili baze.

Main konstruiše pool, repozitorijume i servise, pa ih prosleđuje handlerima preko Axum Extension i deljenih Arc referenci. To je eksplicitno povezivanje zavisnosti. Postoje i specijalizovani moduli sa direktnim SQL-om, naročito za rezervacije, potrošnju i projekcije.“

### 4.6. Auth — identitet i poslovanje

**Otvori:** `services/auth/src/main.rs`, `service/service.rs`, `service/user_service.rs`, `service/business_service.rs`, `repository/business_repository.rs`, `handlers/users.rs`.

„Auth mikroservis upravlja identitetom, profilima, poslovanjima i radnicima. AuthService registruje korisnika, obrađuje lozinku pomoću Argon2 i pri prijavi izdaje JWT. UserService upravlja korisničkim podacima, a BusinessService kreiranjem poslovanja i dodavanjem radnika.

Kod kreiranja poslovanja upis i povezivanje vlasnika izvršavaju se u jednoj transakciji. Korisnik zatim dobija ažuriran token sa business_id. Ruta me čita aktuelne podatke i može da vrati osvežen token.

Radnika kreira vlasnik za svoje poslovanje. Lozinka se čuva kao hash i ne serijalizuje se u korisnički odgovor. Promene relevantne drugim servisima objavljuju se kroz outbox, uz ograničen skup podataka.“

### 4.7. Raw materials — sirovine i pouzdana potrošnja

**Otvori:** `services/raw-materials/src/routes.rs`, `service/service.rs`, `repository/repository.rs`, `handlers/consumption.rs`; po potrebi `libs/common/src/type_catalog.rs`.

„RawMaterialService upravlja unosom, izmenama, datumima, pregledom zaliha i korekcijama količine. Repository ograničava poslovne upite odgovarajućim business_id. Korekcija predstavlja razliku koja se dodaje ili oduzima, a SQL štiti od negativnog stanja.

Potrošnja za proizvodnju ima poseban transakcioni tok. Svaka operacija dobija jedinstveni identifikator. Ponovljen zahtev sa istim identifikatorom i podacima ne skida količinu ponovo. Tako mrežno ponavljanje ne dovodi do duple potrošnje.

Release operacija vraća količinu najviše jednom. Ne postoji automatska konverzija jedinica: potrošnja mora da odgovara jedinici evidentirane sirovine. Katalog tipova kombinuje zajedničke vrednosti i dodatne vrednosti konkretnog poslovanja tamo gde ga servis koristi.“

### 4.8. Productions — serije, koraci, izlazi i oporavak

**Otvori:** `services/productions/src/services/batch_service.rs`, `step_service.rs`, `raw_materials_service.rs`, `models/process_step.rs`, `material_recovery.rs` i `services/productions/migrations/20260924000002_step_status.sql`.

„BatchService koordinira seriju, ulazne sirovine i izlazne proizvode. Pri dodavanju sirovina preuzima njihove podatke i čuva istorijski snimak, pa kasnija izmena izvornog zapisa ne prepisuje prethodnu proizvodnju.

RawMaterialsService u ovom mikroservisu predstavlja vezu sa udaljenim servisom sirovina i koordinaciju potrošnje. Ako deo procesa ne uspe, potrebna je kompenzacija. Material recovery čuva namere i obrađuje poslove povrata kako oporavak ne bi zavisio samo od memorije procesa.

StepService proverava naziv, redosled, parametre i dozvoljene statuse koraka. Parametri su parovi name i value, uz proveru praznih i ponovljenih naziva.

Migracija pokazuje dodatnu zaštitu u bazi: pokretanje koraka pokreće seriju, a završetak serije zahteva završene korake. Zaključavanje roditeljske serije štiti od konkurentnih promena.

Izlazi su lista, tako da jedna serija daje više proizvoda. Pri završetku se potvrđuju stvarne vrednosti i čuva prethodni plan. Završene i otkazane serije ne prihvataju dalje poslovne izmene.“

### 4.9. Products — sva interna zaduženja

**Otvori:** `services/products/src/services/product_service.rs`, `output_consumer.rs`, `reservations.rs`, a zatim module u `services/products/src/services`.

„ProductService upravlja pregledom i izmenama proizvoda i proverava uslove prodaje. Planirani proizvod uređuje se preko serije, a gotov proizvod može da prelazi između skladišta i prodaje. Za aktiviranje se proveravaju cena i završena proizvodnja. Raspoloživost zavisi i od roka trajanja.

Output consumer sluša događaje proizvodnih serija. Na osnovu planiranih izlaza održava proizvode u stanju Production, a posle završetka prevodi ih u Storage. Veza između serije, izlaza i proizvoda, zaključavanje i sekvenca događaja sprečavaju dupliranje pri ponovljenoj obradi.

Reservations je interna funkcionalnost koju koristi Orders. Ona atomarno proverava i rezerviše količine i podržava idempotentno oslobađanje rezervacije.“

**Pokaži redom:** `image_service.rs`, `qr_service.rs`, `provenance_service.rs`, `certificate_service.rs`, `product_policy.rs`, `services/products/assets/certificate.py`.

„ImageService validira i obrađuje sliku i povezuje fajl sa proizvodom. QrService generiše PNG sa javnom adresom porekla. Obnova QR fajla zadržava postojeći token.

ProvenanceService povezuje aktuelni proizvod sa projekcijom porekla iz Read Models servisa. Javni DTO bira dozvoljena polja, bez internih cena, zaliha i podataka o dobavljaču.

CertificateService prosleđuje javne podatke Python rendereru koji pomoću ReportLab-a pravi PDF. Python je ovde pomoćni proces unutar funkcionalnosti Products servisa, nije sedmi HTTP mikroservis.

ProductPolicy je Strategy ugovor za validaciju, a ProductPolicyFactory bira odgovarajuću strategiju. U PDF rendereru Builder sastavlja delove dokumenta, dok Director određuje redosled njihove izgradnje.“

### 4.10. Orders — kupovina, statusi i analitika

**Otvori:** `services/orders/src/handlers/orders.rs`, `services/order_service.rs`, `services/product_service.rs`, `checkout.rs`, `models/order_status.rs` i repozitorijume porudžbina i stavki.

„OrderService upravlja porudžbinama, stavkama i statusima, a product_service modul dohvatom proizvoda iz udaljenog servisa. Handler uzima identitet kupca iz autentifikovanog zahteva i kontaktne podatke iz profila. Cene se proveravaju na serveru.

Checkout modul čuva trajnu nameru pre rezervacije. Isti ključ idempotentnosti, isti kupac i isti zahtev omogućavaju povrat prethodnog rezultata bez pravljenja novih porudžbina. Zatim se rezerviše zaliha, stavke grupišu po poslovanju i porudžbine sačuvaju u lokalnoj transakciji.

Stavke čuvaju naziv i cenu iz trenutka kupovine. Kasnija promena cene proizvoda zato ne menja staru porudžbinu.

OrderStatus eksplicitno definiše dozvoljene prelaze. Otkazivanje pokreće trajni posao oslobađanja zalihe. Ovo je koordinacija sa kompenzacijom između servisa; nije jedna zajednička ACID transakcija preko više baza.

Analitika se računa u Orders servisu. Prihod i najprodavaniji proizvodi koriste isporučene porudžbine, a pregled statusa obuhvata i ostale neobrisane porudžbine. Vremensko grupisanje koristi datum kreiranja porudžbine.“

### 4.11. Read Models — šesti servis i CQRS

**Otvori:** `services/read-models/src/main.rs`, `projector.rs` i `queries.rs`.

„Read Models ima posebnu bazu namenjenu objedinjavanju podataka za čitanje. Projector prima događaje i održava projekcije. Query API iz njih daje dashboard, listu proizvođača i podatke za poreklo.

Ovo je selektivna primena CQRS-a: upis ostaje u domenskim servisima, dok su određeni objedinjeni pregledi izdvojeni. Obični CRUD pregledi i analitika porudžbina i dalje ostaju u svojim servisima.

Projector pamti obrađene događaje i proverava njihove sekvence. Ponovljena poruka ne treba da se primeni dvaput, a starija ne sme da pregazi novije stanje. Obrisani zapis ostavlja oznaku brisanja da ga stari događaj ne bi ponovo oživeo.

Projekcije se ažuriraju asinhrono, pa postoji kratko kašnjenje između upisa i objedinjenog prikaza. To je eventualna konzistentnost. Sistem nije potpuni Event Sourcing, jer izvorno stanje čuvaju poslovne tabele.“

### 4.12. RabbitMQ, outbox i ceo tok

**Otvori:** `libs/common/src/events.rs` i jednu migraciju `20260917000000_transactional_outbox.sql` iz domenskog servisa.

„Promena poslovnog podatka i upis događaja u outbox nastaju u istoj lokalnoj transakciji. Pozadinski relay objavljuje događaj u RabbitMQ i označava ga poslatim tek kada broker potvrdi objavu.

Ako broker privremeno nije dostupan, događaj ostaje u bazi za sledeći pokušaj. Poruka može biti ponovljena, zbog čega potrošači moraju da podrže idempotentnu obradu. Posebni redovi omogućavaju da isti događaj nezavisno obrade Products i Read Models. Neispravne poruke izdvajaju se u dead-letter redove.

Na našem primeru: završetak serije menja Productions bazu i upisuje outbox; RabbitMQ prenosi događaj; Products ažurira gotove proizvode; Read Models ažurira prikaz porekla. Pri skeniranju QR-a Products proverava javnu dostupnost proizvoda i uzima objedinjene podatke iz Read Models servisa.“

### 4.13. Provere i završna rečenica

**Otvori:** direktorijum `scripts`, primere `verify-product-workflow.cjs`, `verify-role-stock.cjs`, `verify-cqrs.cjs`, `test-step-status.ps1`, `test-certificate.py`, kao i jedan frontend `.spec.ts` fajl.

„Projekat sadrži jedinične testove i skripte koje proveravaju poslovne tokove: statuse koraka, nastanak proizvoda, dozvole, zalihu, projekcije i PDF. Posebno su značajne provere ponavljanja zahteva i prelaza između servisa, jer se tu proverava doslednost celog procesa.

Time sam pokazala kako se korisnički tok povezuje sa implementacijom: od Angular forme, preko poslovnih pravila i baza, do događaja i javnog prikaza porekla. Glavna vrednost sistema je povezana evidencija proizvodnje i prodaje, uz jasne odgovornosti servisa i podršku za oporavak distribuiranih operacija.“

Rezultate testova predstavljaj kao uspešne samo ako ih prethodno stvarno pokreneš i proveriš izlaz.

## 5. Precizni odgovori ako profesor pita

- **Zašto šest servisa?** Pet servisa pokriva poslovne oblasti; šesti objedinjuje podatke za određene upite i poreklo.
- **Zašto REST i RabbitMQ zajedno?** REST služi kada je potreban neposredan odgovor, poput rezervacije; događaji služe nezavisnom obaveštavanju i ažuriranju projekcija.
- **Šta ako se poruka ponovi?** Potrošači koriste evidenciju obrade, sekvence i jedinstvene veze. Ne tvrditi da transport garantuje exactly-once isporuku.
- **Šta ako zahtev prekine posle rezervacije?** Trajna namera, isti ključ operacije i recovery tok omogućavaju nastavak ili kompenzaciju kada servisi ponovo postanu dostupni.
- **Da li svi pregledi koriste CQRS?** Ne. CQRS je primenjen na izdvojene objedinjene preglede; domenski servisi zadržavaju svoje upite.
- **Da li QR dokazuje istinitost porekla?** Prikazuje evidentirani lanac porekla. Nema nezavisne provere ni digitalnog potpisa sertifikata.
- **Da li postoje plaćanje i AI?** U aktuelnom kodu nema elektronskog plaćanja, kurirske integracije ni AI prognoziranja.
- **Kako su prikazani grafikoni?** Postoje CSS trake, liste i tabele; ne navoditi Chart.js kao korišćenu biblioteku.
- **Da li su svi obrasci GoF?** Ne. Repository, CQRS i Outbox su drugačije kategorije obrazaca. Ovde su Strategy i Builder konkretni primeri, Factory je Simple Factory, a statusni prelazi su eksplicitne mašine stanja.

Ne oslanjaj se na stari opis u korenskom README-u za broj servisa i noviji proizvodni tok. Aktuelni kod podržava više izlaza po seriji, parametre koraka i objedinjenu Products stranicu sa tri stanja.
