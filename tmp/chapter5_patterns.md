## 5.6 Primenjeni softverski obrasci

Softverski obrasci predstavljaju ponovljiva rešenja za karakteristične probleme projektovanja. U LocalBite-u njihova primena obuhvata organizaciju koda, izbor ponašanja, izgradnju dokumenata i saradnju distribuiranih servisa. Klasični objektni obrasci, kao što su Strategy i Builder [23], razlikuju se od arhitektonskih i integracionih obrazaca. Zato se svaki obrazac u nastavku povezuje sa konkretnim problemom, učesnicima u implementaciji i granicama ostvarene primene.

### 5.6.1 Repository i Service Layer

Repository izdvaja pristup trajno sačuvanim podacima iza operacija koje su razumljive poslovnom sloju [24]. U projektu su primeri BatchRepository, ProductRepository i OrderRepository. Metoda find_by_id_and_farm izražava potrebu za pronalaženjem objekta u okviru gazdinstva, dok njen SQL izraz ostaje u repozitorijumu. Pozivalac tako ne mora u svakom poslovnom toku ponovo da sastavlja uslove filtriranja i mapiranje rezultata.

Service Layer objedinjuje poslovne operacije i koordinira njihov tok [25]. BatchService, na primer, proverava da li je serija završena, da li svi koraci dozvoljavaju završetak proizvodnje i da li su izlazni proizvodi validni. Tek nakon provera prosleđuje izmenu repozitorijumu. HTTP funkcija ostaje odgovorna za prijem zahteva i ovlašćenja, dok je poslovna odluka izdvojena iz mrežnog interfejsa.

Ova kombinacija smanjuje ponavljanje i olakšava pronalaženje pravila pri održavanju. Ipak, repozitorijumi su pretežno konkretne Rust strukture, a deo integracionih tokova izvršava SQL neposredno. Primena je stoga slojevita organizacija sa poznatim izuzecima, bez tvrdnje da je ostvarena potpuna nezavisnost domena od infrastrukture ili stroga heksagonalna arhitektura.

### 5.6.2 Ubrizgavanje zavisnosti

Ubrizgavanje zavisnosti razdvaja stvaranje saradničkih objekata od njihove upotrebe [26]. U Rust servisima početna funkcija konstruiše skup konekcija, repozitorijume i poslovne servise, a zatim ih povezuje kroz konstruktore. Na primer, BatchService dobija repozitorijume serija, koraka i sirovina, kao i klijent za operacije nad sirovinama. Axum prosleđuje pripremljene instance funkcijama za obradu zahteva preko Extension mehanizma.

Na ovaj način zavisnosti su vidljive na mestu konstrukcije, umesto da ih svaki objekat samostalno stvara tokom obrade. Angular koristi ugrađeni sistem ubrizgavanja kroz inject, deklaracije servisa i pružaoce zavisnosti. Ograničenje serverske realizacije jeste vezanost većine konstruktora za konkretne tipove, pa zamena repozitorijuma testnom implementacijom nije univerzalno omogućena samo prisustvom ovog obrasca. Deljena instanca preko Arc-a ili Angular servisa ne predstavlja sama po sebi implementaciju obrasca Singleton.

### 5.6.3 DTO i mapiranje podataka

Objekti za prenos podataka, odnosno DTO strukture, definišu podatke koji prelaze granicu interfejsa. U projektu se zahtevi za kreiranje i izmenu razlikuju od modela baze, a složeni odgovori sastavljaju se namenskim funkcijama. ProductionBatchResponse objedinjuje seriju, proizvodne korake i sirovine, dok map_order_response priprema odgovor porudžbine i njenih stavki.

PublicProvenance izdvaja skup informacija dozvoljenih za javni prikaz i generisanje dokumenta. Takvo mapiranje sprečava da se interni model neposredno proglasi javnim ugovorom. Prednost je kontrola sadržaja i mogućnost razvoja šeme baze bez automatskog izlaganja svih promena klijentu. Potrebno je, međutim, održavati doslednost između modela, mapiranja i korisničkog interfejsa, jer tipizovane strukture ne uklanjaju mogućnost semantičke neusaglašenosti.

### 5.6.4 Strategy i Simple Factory

Strategy izdvaja promenljivo ponašanje iza zajedničkog ugovora. U servisu proizvoda ugovor ProductPolicy definiše metodu validate, dok FoodProductPolicy i GenericProductPolicy predstavljaju konkretne strategije. Zajedničke provere obuhvataju naziv, tip, cenu i količinu. Strategija za prehrambene proizvode dodatno zahteva jedinicu mere.

ProductPolicyFactory::for_type bira strategiju prema tipu proizvoda i vraća objekat koji implementira zajednički ugovor. ProductService primenjuje izabranu strategiju pri izmeni proizvoda, a obrada proizvodnih događaja koristi je pri validaciji izlaza pre upisa. HTTP sloj zato ne sadrži zasebne grane za sve porodice proizvoda. Dodavanje nove politike može se ostvariti uvođenjem strategije i dopunom mesta izbora.

Realizacija izbora odgovara jednostavnoj fabrici, odnosno Simple Factory pristupu, jer jedna funkcija odlučuje koji objekat da konstruiše. Nije uspostavljena hijerarhija kreatora sa predefinisanjem metode stvaranja karakteristična za GoF Factory Method. Takođe, sva pravila prodaje nisu preneta u strategije: pozitivna prodajna cena, završena serija i zabrana promene potvrđene količine ostaju odgovornost poslovnog servisa. Ovakva podela odgovara stvarnom obimu primene obrasca.

### 5.6.5 Builder i Director za dokument o poreklu

Builder razdvaja postupak sastavljanja složenog rezultata od njegove konkretne reprezentacije. U Python komponenti za PDF, CertificateBuilder definiše korake product, production, materials i disclaimer, kao i operaciju result. ReportLabCertificateBuilder implementira ove korake kroz paragrafe, tabele, stilove i listu elemenata dokumenta. CertificateDirector određuje redosled izgradnje.

Tok generisanja počinje pribavljanjem javnih podataka o poreklu. Rust servis ih prosleđuje Python procesu, a funkcija render povezuje direktora sa konkretnim graditeljem. Direktor dodaje podatke o proizvodu, proizvodnji, sirovinama i napomenu o izvoru podataka, nakon čega preuzima gotov PDF. Time je pravilo organizacije sadržaja odvojeno od ReportLab detalja.

U projektu postoji RecordingBuilder namenjen proveri redosleda poziva direktora. Takav test pokazuje da se saradnja učesnika može proveriti bez oslanjanja na vizuelni izgled PDF-a. Drugi graditelj mogao bi podržati drugačiju reprezentaciju uz očuvan redosled koraka, ali takva dodatna reprezentacija nije deo aktuelne implementacije. Obrazac ovde ima jasne učesnike i stvarnu upotrebu u poslovnom toku, nezavisno od bibliotečkih klasa sličnog naziva.

### 5.6.6 Publish–Subscribe i reaktivne pretplate

Publish–Subscribe razdvaja objavljivanje događaja od pojedinačnih reakcija na njega. Servis proizvodnje objavljuje promenu serije, a RabbitMQ je prosleđuje odgovarajućim redovima. Potrošač projekcija osvežava podatke za čitanje, dok potrošač izlaza proizvodnje ažurira proizvode. Proizvođač događaja ne poziva svaki od ovih tokova zasebno, a potrošači obrađuju poruke nezavisno.

Na korisničkoj strani, RxJS Observable i pretplate omogućavaju reakciju na odgovore i vremenske događaje [19]. Primer je periodično osvežavanje upravljačkog pregleda uz prekid pretplate pri zatvaranju komponente. Ovakva primena ima vezu sa principom Observer obrasca, ali distribuirana komunikacija preko RabbitMQ-a nije klasična implementacija objekta koji u istoj memoriji održava listu posmatrača. Razlikovanje ova dva nivoa važno je zbog različitih uslova otkaza i trajnosti poruka.

### 5.6.7 Transactional Outbox

Transactional Outbox rešava problem nepouzdanog odvojenog upisa poslovne promene i objavljivanja događaja [27]. Ukoliko se baza uspešno izmeni, a broker u tom trenutku nije dostupan, događaj ne sme trajno nestati. U LocalBite-u okidači upisuju zapis u integration_outbox u istoj lokalnoj transakciji sa promenom poslovnog objekta. Pozadinski proces zatim čita neposlate zapise, objavljuje ih i označava kao poslate nakon potvrde brokera.

Primer je završavanje serije: status i događaj ostaju sačuvani zajedno čak i kada RabbitMQ privremeno nije dostupan. Nakon oporavka, događaj može stići do servisa proizvoda i projekcija. Ako dođe do prekida nakon objavljivanja, a pre označavanja zapisa, poruka može biti poslata ponovo. Obrazac zato zahteva idempotentne potrošače, kao i održavanje evidencije događaja i praćenje zaostale obrade. Njegova garancija odnosi se na vezu lokalne promene i zapisa događaja, a ne na jednu transakciju koja obuhvata sve servise.

### 5.6.8 Idempotent Consumer i zaštita od zastarelih događaja

Potrošač projekcija beleži obrađene poruke u projection_receipts prema izvoru i sekvenci događaja. Zapis o prijemu i promena projekcije izvršavaju se u istoj transakciji. Ako ista poruka ponovo stigne, postojeći zapis sprečava ponovnu primenu. Dodatno, sekvenca na projektovanom entitetu obezbeđuje da stariji događaj ne prepiše novije stanje.

Logička oznaka brisanja zadržava informaciju da je entitet uklonjen, kako ranije poslata poruka ne bi ponovo uspostavila njegov prikaz. U servisu proizvoda odvojeni mehanizam koristi stanje proizvodnje, identitete izlaza i evidenciju production_outputs. Time se ponavljanje promene serije ne tumači kao nalog za stvaranje nove zalihe. Ovakve zaštite odnose se na konkretne operacije; ne znače da su svi pozivi aplikacije automatski idempotentni.

### 5.6.9 CQRS i projekcije

CQRS razdvaja odgovornosti modela za menjanje podataka i modela za njihovo čitanje [28]. Domenski servisi LocalBite-a proveravaju poslovna pravila i čuvaju merodavno stanje. Servis Read Models iz događaja formira projekcije pogodne za upravljački pregled i poreklo proizvoda. Čitanje porekla zato ne zahteva da svaki javni zahtev posebno pribavlja sve podatke iz servisa korisnika, proizvodnje i sirovina.

Prednost je mogućnost oblikovanja prikaza prema potrebama čitanja, uz očuvanje vlasništva nad izvornim podacima. Ograničenje je vremenski razmak između promene izvora i osvežavanja projekcije. Primena je selektivna, jer pojedini pregledi i analitika ostaju u domenskim servisima. Projekcije su aplikacione tabele i nisu PostgreSQL materijalizovani pogledi.

Sistem ne primenjuje potpuni Event Sourcing: trenutno poslovno stanje čuva se u redovnim tabelama, a integracioni događaji služe prenosu promena i obnovi projekcija. Mogućnost ponovnog slanja outbox događaja nije dovoljna da bi se celo rešenje označilo kao sistem zasnovan na rekonstrukciji stanja isključivo iz dnevnika događaja.

### 5.6.10 Trajna orkestracija i kompenzacioni tokovi

Saga pristup koordinira više lokalnih transakcija i predviđa kompenzacije kada se poslovni tok ne može završiti [29]. LocalBite koristi trajno evidentirane korake i kompenzacione operacije koji slede ovaj princip, bez posebnog opšteg izvršnog okvira za sage. Servis porudžbina čuva nameru u checkout_jobs, zahteva rezervaciju proizvoda i zatim upisuje porudžbine i sačuvani odgovor. Pozadinski proces nastavlja obradu nezavršenih zahteva istim identifikatorom.

Ako je rezervacija već izvršena pre prekida procesa, njeno ponavljanje ne sme ponovo umanjiti zalihu. Kada vlasnik otkaže porudžbinu, zajedno sa promenom statusa upisuje se stock_release_jobs zapis. Oslobađanje rezervacije zatim se ponavlja do uspešnog izvršavanja, uz idempotentnu obradu u servisu proizvoda. Zbog toga povraćaj ne mora biti vidljiv u istom trenutku kada je prihvaćeno otkazivanje.

Sličan mehanizam postoji za sirovine: material_intents čuva nameru potrošnje, dok material_release_jobs evidentira potreban povraćaj. Okidači kreiraju poslove povraćaja pri dozvoljenom uklanjanju veze sirovine, otkazivanju ili logičkom brisanju serije. Oporavak razlikuje uspešno povezanu sirovinu od prekinute operacije i šalje odgovarajući zahtev servisu zaliha. Kompenzacija predstavlja novu poslovnu operaciju kojom se ispravlja posledica prethodnog koraka; ne predstavlja vraćanje jedne distribuirane SQL transakcije.

### 5.6.11 Konačne mašine stanja

Životni ciklusi porudžbine, serije i proizvodnog koraka predstavljeni su eksplicitnim skupovima stanja i dozvoljenih prelaza. OrderStatus definiše dozvoljene naredne statuse, dok BatchService i StepService proveravaju promene stanja proizvodnje. Okidači baze dodatno štite vezu između statusa koraka i zatvaranja serije, uključujući konkurentne zahteve.

Ovakav pristup čini poslovne prelaze preglednim: porudžbina ne može neposredno preći iz čekanja u isporučeno stanje, a serija se ne može završiti dok postoje nezavršeni koraci. Međutim, u kodu ne postoji poseban objekat sa ponašanjem za svako stanje, što bi karakterisalo objektnu realizaciju GoF State obrasca. Preciznije je govoriti o konačnim mašinama stanja, realizovanim enumeracijama, uslovima i ograničenjima baze.

### 5.6.12 Istorijski snimci i logičko brisanje

Istorijski snimci izdvajaju podatke relevantne za prethodno izvršenu operaciju. Veza sirovine i serije čuva podatke zabeležene pri utrošku, a stavka porudžbine naziv, cenu i jedinicu proizvoda u trenutku kupovine. Kasnija promena dobavljača ili prodajne cene zato ne menja automatski značenje ranije evidencije. Kod izlaza proizvodnje zadržava se i prethodno sačuvan plan, kada postoji, uz potvrđene vrednosti.

Logičko brisanje koristi oznaku is_deleted za izdvajanje zapisa iz redovnih pregleda uz očuvanje podataka potrebnih za istoriju i integraciju. Ovaj pristup zahteva dosledne filtere u svim relevantnim upitima i nije podjednako primenjen na sve tabele. Istorijski snimci i logičko brisanje predstavljaju obrasce rada sa podacima, a ne samostalnu realizaciju nepromenljivog revizionog dnevnika.

### 5.6.13 Zajednički doprinos obrazaca

Primenjeni obrasci rešavaju različite delove istog poslovnog problema. Repository, Service Layer i ubrizgavanje zavisnosti određuju organizaciju koda; Strategy izdvaja pravila validacije, a Builder postupak sastavljanja dokumenta. Outbox, nezavisni potrošači, projekcije i kompenzacije povezuju servise tako da privremeni prekid komunikacije ne mora dovesti do trajnog gubitka poslovnog toka.

Njihova vrednost procenjuje se kroz konkretno ponašanje: završena serija mora sačuvati potvrđene izlaze, ponovljena poruka ne sme duplirati zalihu, a otkazana porudžbina mora pokrenuti oslobađanje rezervacije. Primena obrazaca ujedno povećava broj komponenti i evidencija koje treba održavati. Zbog toga je svaka apstrakcija u radu vezana za postojeći problem implementacije i njena ograničenja, umesto da se broj imenovanih obrazaca posmatra kao samostalan pokazatelj kvaliteta.
