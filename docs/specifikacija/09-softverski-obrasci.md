# 09 — Softverski obrasci: gde i kako se koriste

[Sadržaj specifikacije](README.md)

Obrasci su razdvojeni na arhitektonske, integracione i obrasce organizacije koda. Nije svaki od njih GoF obrazac. Prisustvo naziva u komentaru nije dovoljan dokaz da je obrazac aktivan u poslovnom toku.

## 1. Pregled

| Obrazac / praksa | Status | Konkretno mesto |
|---|---|---|
| Microservices | Aktivno | Šest servera, šest baza, zasebni procesi |
| Database per Service | Aktivno | Docker Compose i servisne migracije |
| Layered Architecture | Aktivno, sa izuzecima | handlers → services → repositories |
| Repository | Aktivno | UserRepository, BatchRepository, ProductRepository, OrderRepository |
| Service Layer | Aktivno | BatchService, OrderService, BusinessService |
| Dependency Injection / composition root | Aktivno | main.rs, Arc/Extension; Angular providers/inject |
| DTO / Mapper | Aktivno | dtos, map_order_response, assemble_detail |
| API Gateway / Reverse Proxy | Aktivno | nginx/nginx.conf |
| Transactional Outbox | Aktivno | SQL trigger-i i common/events.rs |
| Publish–Subscribe | Aktivno | Topic exchange i odvojeni consumer redovi |
| CQRS | Selektivno | Read-models dashboard/poreklo, domenske write baze |
| Idempotent Consumer / Inbox-like receipt | Aktivno | projection_receipts i uslovni upsert |
| Snapshot / denormalizacija | Aktivno | batch_raw_materials i order_items |
| Trajan orkestrisani kompenzacioni tok | Aktivno | checkout_jobs, material_intents i recovery petlje |
| State machine | Aktivno | OrderStatus i validate_status_transition |
| Soft Delete | Aktivno | is_deleted u domenskim tabelama |
| Strategy + Simple Factory | Aktivno u izmeni proizvoda i obradi izlaza | products/services/product_policy.rs |
| GoF Builder + Director za PDF | Aktivno | CertificateBuilder, ReportLabCertificateBuilder, CertificateDirector |
| Observer / reaktivna pretplata | Aktivno u reaktivnom smislu | RxJS subscribe, Angular signals/effect |

## 2. Repository i Service Layer

Repository skriva SQL detalje iza metoda sa domenskim imenima, npr. `find_by_id_and_business`, `insert_in` i `soft_delete`. Service Layer povezuje više repository poziva i proverava poslovna pravila: OrderService grupiše proizvode po gazdinstvu, a BatchService upravlja statusnim prelazom.

Dobit je manje SQL-a u HTTP handlerima i jedno mesto za pravilo. Ograničenje: repozitorijumi su uglavnom konkretni struct-ovi bez trait apstrakcije, a service-i zavise od konkretne implementacije. Zbog toga nije automatski obezbeđeno jednostavno mock-ovanje svega niti strogo dependency inversion pravilo. Direktan SQL u consumption handleru, queries i output consumer-u predstavlja izuzetak od uobičajenih slojeva.

## 3. Dependency Injection

Rust `main.rs` konstruiše PgPool, repository i service objekte i deli ih preko Arc-a. Axum Extension je mehanizam kojim handler dobija te objekte. To je eksplicitna injekcija zavisnosti, bez posebnog DI kontejnera.

Angular koristi svoj DI sistem: `@Injectable({providedIn:'root'})`, `inject(...)` i providers. HttpTestingController/TestBed mogu zameniti zavisnosti u testu. „Jedna instanca root servisa“ je scope DI kontejnera, ne dokaz ručno implementiranog GoF Singleton-a.

## 4. DTO i mapiranje

DTO odvaja HTTP ugovor od tabele. `OrderItemRequest` sadrži samo proizvod i količinu; cena i naziv uzimaju se sa servera. `ProductionBatchResponse` okuplja tri tabele. `map_order_response` pretvara BigDecimal iz baze u numerički izlaz.

Identitet kupca se prepisuje iz JWT-a, a kontakt iz auth profila. Javni PublicProvenance je eksplicitna lista dozvoljenih polja. Frontend razlikuje API omotač, paginiranu listu i decimalne vrednosti na mreži.

## 5. Transactional Outbox

Problem koji obrazac rešava: servis ne sme uspešno promeniti bazu, a zauvek izgubiti događaj zato što je broker nedostupan. Trigger upisuje outbox unutar iste transakcije kao izmenu. Relay kasnije objavljuje i označava događaj tek posle AMQP potvrde.

Primer: promena statusa serije u COMPLETED kreira outbox zapis čak i ako RabbitMQ trenutno ne radi. Posle oporavka isti zapis stiže output consumer-u i projekciji. Cena je potreba za čišćenjem istorije, praćenjem backlog-a i idempotentnim potrošačima.

## 6. Publish–Subscribe i Observer

Publisher ne poziva svaki consumer direktno. RabbitMQ topic exchange prosleđuje poruku svakom odgovarajućem redu. Read-model red prima sve, products-output samo promene serija. To je distribuirani Publish–Subscribe obrazac, funkcionalno srodan Observer ideji obaveštavanja pretplatnika.

Ne postoji klasična in-process GoF Subject klasa sa listom Observer objekata za proizvode. Opis „Observer kroz RabbitMQ“ treba kvalifikovati kao event-driven analogiju. Na frontend-u RxJS Observable i subscribe, kao i Angular signal/effect, stvarno omogućavaju reaktivne pretplate unutar aplikacije.

## 7. CQRS i projekcije

Komandni model čuva validne domenske zapise, a query model nudi strukturu pogodnu za objedinjeni pregled. Read-models ne menja izvorne poslovne entitete, nego prima njihove događaje. Poreklo tada ne mora da zove auth, productions i raw-materials za svaki QR zahtev.

Obrazac je selektivan: obični CRUD pregledi i orders analitika i dalje žive u domenskim servisima. Nema potpunog Event Sourcing-a: poslovno stanje se ne dobija isključivo reprodukcijom nepromenljivog domenskog dnevnika, već je sačuvano u običnim tabelama. Replay outbox-a služi obnovi projekcija i integracije.

## 8. Idempotency, receipt i tombstone

`projection_receipts` sprečava duplu primenu `(source, sequence)`. `projection_entities.sequence` sprečava starije stanje istog entiteta. `deleted` red ostaje kako stari INSERT ne bi ponovo aktivirao obrisani entitet.

Sličan cilj drugim mehanizmom ostvaruje `production_outputs`: jedinstveni batch i advisory lock dopuštaju jedno kreiranje proizvoda. `production_consumption.operation_id` sprečava višestruko skidanje iste sirovine pri retry-u. Ovi mehanizmi imaju lokalni opseg; ne znače da je svaki API poziv idempotentan. Orders POST zahteva UUID Idempotency-Key; isti kupac i isti zahtev dobijaju sačuvan odgovor, a drugi sadržaj ili kupac daju 409.

## 9. Trajna kompenzacija i granica Saga obrasca

Orders trajno upisuje nameru u checkout_jobs pre rezervacije. Products atomarno rezerviše sve stavke i pamti isti ključ. Orders u lokalnoj transakciji kreira sve porudžbine, veze i odgovor. Pad posle udaljenog commit-a rešava recovery petlja ponavljanjem istog ključa. Otkazivanje u istoj transakciji kreira stock_release_jobs; vraćanje zalihe je idempotentno.

Productions nezavisno commit-uje material_intents pre udaljene potrošnje. Advisory lock razlikuje aktivan lokalni posao od prekinutog procesa. Recovery proverava postojanje veze i oslobađa neuspešne operacije; brisanje veze i otkazivanje kreiraju material_release_jobs. Raw-materials tombstone sprečava da kasni zahtev ponovo potroši oslobođenu operaciju.

To je trajna orkestracija sa eventualnim završetkom i kompenzacijom, ne distribuirana ACID transakcija. Oporavak zavisi od ponovne dostupnosti servisa i očuvanih evidencija.

## 10. State machine, ne GoF State objekti

`OrderStatus` definiše skup stanja i `allowed_transitions`. BatchService koristi match nad statusnim stringom. To su eksplicitne konačne mašine stanja. Nema posebne klase/struct-a sa ponašanjem za svako stanje, pa to nije puna objektna realizacija GoF State obrasca.

Prednost je lako proverljiv graf. Nedostatak je što zaštita mora važiti na svim putanjama koje menjaju zavisne podatke; SQL trigger-i zaključavaju roditeljsku seriju i odbijaju insert/update/delete koraka i materijala završene, otkazane ili obrisane serije.

## 11. Strategy i Simple Factory — aktivni pozivi

ProductPolicy definiše validate. ProductPolicyFactory::for_type bira FoodProductPolicy za prehrambene kategorije ili GenericProductPolicy za ostale. Zajednička pravila proveravaju naziv, tip, konačne nenegativne brojeve i preciznost količine; prehrambena strategija zahteva i jedinicu. ProductService.update validira efektivan zahtev, a output_consumer::validate proverava izlaz pre upisa. Posebna pravila prodaje (pozitivna cena, završena serija, nepromenljiv prinos) ostaju u servisu.

Ovo je Simple Factory sa strategijama, ne GoF Factory Method hijerarhija. Promena strategije ne menja HTTP handler.

## 12. GoF Builder za PDF

CertificateBuilder je apstraktan ugovor sa koracima product, production, materials, disclaimer i result. ReportLabCertificateBuilder čuva fontove, stilove i story, escape-uje tekst i proizvodi PDF. CertificateDirector određuje redosled poziva. render povezuje Director sa konkretnim Builder-om; Rust CertificateService šalje samo javni DTO Python procesu.

Test sa RecordingBuilder potvrđuje redosled, a drugi test renderuje dug sadržaj sa srpskim slovima. Razdvajanje omogućava drugi format bez promene redosleda izgradnje. Angular FormBuilder je zaseban bibliotečki API i nije dokaz ovog obrasca.

## 13. Dodatne prakse

- **Snapshot:** sirovinski podaci u proizvodnji i cena/naziv proizvoda u porudžbini čuvaju istorijski kontekst.
- **Soft Delete:** skrivanje domenskog zapisa uz očuvanje istorije; nije univerzalno primenjeno na sve tabele.
- **Middleware/Interceptor:** zajednička JWT obrada van pojedinačnih ekrana; server i klijent imaju različite odgovornosti.
- **Facade/Adapter u praktičnom smislu:** frontend API servisi i HTTP helper-i kriju detalje URL-a i payload-a; nisu formalna nezavisna ports-and-adapters arhitektura.
- **Optimistička kontrola konkurentnosti:** očekivani status pri izmeni batch-a; ne postoji univerzalna version kolona na svim modelima.

Za svako navođenje obrasca u diplomskom radu navesti konkretan tip/funkciju, problem koji rešava i postojeće ograničenje. Time opis ostaje proverljiv i nakon promena arhitekture.
