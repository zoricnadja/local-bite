# 4 Teorijske osnove i izbor tehnologija

Tehnološka osnova platforme LocalBite oblikovana je prema zahtevima za razdvajanjem poslovnih odgovornosti, kontrolisanim pristupom podacima, praćenjem proizvodnje i pouzdanom razmenom promena između servisa. Serverski deo razvijen je u programskom jeziku Rust, korisnički interfejs u Angular okruženju, a trajno čuvanje podataka i asinhrona komunikacija ostvareni su pomoću PostgreSQL baza i RabbitMQ brokera. U nastavku su predstavljene uloge ovih tehnologija i način njihove primene u projektu.

Navedene verzije odnose se na deklaracije u projektnim manifestima i oznake kontejnerskih slika. Konačne verzije zavisnosti određuju datoteke Cargo.lock i package-lock.json. Oznaka Rust 2021 predstavlja izdanje jezika, a ne verziju kompajlera. Razlikovanje ovih podataka važno je za ponovljivost izgradnje i pravilno tumačenje razvojnog okruženja.

## 4.1 Mikroservisna arhitektura

Mikroservisna arhitektura zasniva se na podeli aplikacije na zasebne servise sa jasno određenim odgovornostima i komunikacijom preko definisanih interfejsa [5]. LocalBite obuhvata pet poslovnih servisa za korisnike i gazdinstva, sirovine, proizvodnju, proizvode i porudžbine, kao i servis namenjen projekcijama podataka za čitanje. Svaki servis ima sopstveni izvršni proces i bazu podataka.

Ovakva organizacija omogućava da se pravila proizvodnje razvijaju odvojeno od pravila poručivanja ili prikaza porekla. Istovremeno, operacija koja obuhvata više servisa ne može se osloniti na jednu lokalnu SQL transakciju. Na primer, rezervacija proizvoda i upis porudžbine zahtevaju koordinaciju dve baze. Zbog toga su pouzdano objavljivanje događaja, idempotentnost i oporavak sastavni delovi rešenja. Zajednički repozitorijum i biblioteka common olakšavaju razvoj, ali uvode i određenu povezanost servisa pri izgradnji i promeni zajedničkih ugovora.

## 4.2 Rust i upravljanje projektom pomoću Cargo alata

Rust je statički tipizovan jezik u kojem se upravljanje memorijom zasniva na vlasništvu nad vrednostima i pravilima pozajmljivanja referenci. Ova pravila omogućavaju otkrivanje određenih grešaka pri prevođenju programa, bez oslanjanja na sakupljač otpada [10]. U projektu tipovi Option i Result eksplicitno predstavljaju odsustvo vrednosti i ishod operacije, što je značajno za obradu opcionih datuma, rezultata upita i poslovnih grešaka.

Serverski projekat organizovan je kao Cargo workspace sa šest servisnih paketa i zajedničkom bibliotekom. Zavisnosti koje koristi više servisa deklarisane su u korenskom manifestu, dok svaki servis zadržava svoj ulazni program, modele, migracije i konfiguraciju. Biblioteka common objedinjuje, između ostalog, obradu grešaka, proveru ovlašćenja i infrastrukturu integracionih događaja. Time se smanjuje ponavljanje koda koji mora imati isto značenje u različitim servisima.

Deljene instance poslovnih servisa i repozitorijuma prosleđuju se pomoću Arc tipa. Takvo deljenje objekata unutar procesa ne zamenjuje kontrolu konkurentnosti u bazi: istovremene izmene zaliha i dalje zahtevaju transakcije, uslovne upite ili zaključavanje. Izgradnja u kontejneru koristi postojeću datoteku Cargo.lock i optimizovani režim prevođenja, čime se olakšava ponavljanje izgradnje sa istim skupom zavisnosti.

## 4.3 Axum i Tokio

Axum je okvir za razvoj HTTP aplikacija koji povezuje putanje i metode zahteva sa funkcijama za njihovu obradu. Mehanizam izdvajanja podataka iz zahteva omogućava tipizovano čitanje putanje, parametara upita i JSON sadržaja [13]. U projektu je deklarisana verzija Axum 0.8.8. Identifikatori se preuzimaju kroz Path<Uuid>, filteri i paginacija kroz Query, a podaci obrazaca kroz Json strukture. Projektni AuthClaims mehanizam izdvaja i proverava podatke iz pristupnog tokena.

Funkcija za obradu zahteva povezuje HTTP interfejs sa poslovnim slojem. Na primer, zahtev za dodavanje sirovine dobija identitet gazdinstva iz proverene sesije, umesto da prihvati proizvoljan identifikator poslat iz obrasca. Zajednički tip AppError prevodi poslovne i infrastrukturne greške u odgovarajuće HTTP odgovore. Ovakvo razdvajanje olakšava ujednačeno prijavljivanje neispravnog zahteva, zabrane pristupa, nepostojećeg resursa ili konflikta.

Tokio obezbeđuje izvršno okruženje za asinhrone operacije u Rust-u [14]. Koristi se za prihvatanje mrežnih zahteva, pristup bazi, međuservisne pozive i pozadinske zadatke. Dok operacija čeka mrežni odgovor ili rezultat upita, izvršno okruženje može obrađivati druge spremne zadatke. U LocalBite-u pozadinski zadaci objavljuju događaje, primaju poruke i ponavljaju nedovršene operacije oporavka. Asinhrono izvršavanje samo po sebi ne obezbeđuje poslovnu ispravnost, pa su provere stanja i transakcione granice definisane zasebno.

## 4.4 HTTP interfejsi i međuservisna komunikacija

REST predstavlja arhitektonski stil za distribuirane hipermedijske sisteme [6]. U projektu se resursno organizovani HTTP interfejsi koriste za operacije nad korisnicima, gazdinstvima, sirovinama, serijama, proizvodima i porudžbinama. Metoda i putanja određuju traženu operaciju, dok ulazne i izlazne strukture definišu ugovor između klijenta i servisa. HTTP komunikacija naročito je značajna kada je pre nastavka poslovnog toka potreban neposredan rezultat provere.

Biblioteka Reqwest koristi se za pozive između servisa. Servis proizvodnje preko nje pribavlja podatke o sirovinama i evidentira njihovu potrošnju, servis porudžbina zahteva rezervaciju proizvoda, a servis proizvoda pribavlja projekciju porekla. Adrese odredišta podešavaju se kroz promenljive okruženja. Interni pozivi koriste odgovarajuća tehnička ovlašćenja, dok vremenska ograničenja sprečavaju neograničeno čekanje na nedostupan servis.

Prekid HTTP komunikacije ne određuje pouzdano da li je udaljena operacija izvršena. Zbog toga zahtev za kreiranje porudžbine koristi ključ idempotentnosti, a rezervacija se povezuje sa trajno sačuvanim identifikatorom obrade. Ponovljeni poziv nastavlja istu operaciju ili vraća njen rezultat. Ovaj mehanizam je deo poslovnog protokola i ne proizlazi automatski iz upotrebe HTTP-a.

## 4.5 PostgreSQL i SQLx

PostgreSQL je relacioni sistem za upravljanje bazama podataka sa podrškom za transakcije, integritet podataka i proširive tipove [8]. Konfiguracija projekta koristi PostgreSQL 15 i zasebnu bazu za svaki servis. Takav izbor omogućava da se ograničenja i migracije šeme vezuju za servis koji poseduje podatke. Veze između različitih servisnih baza proveravaju se kroz aplikacionu komunikaciju, jer ne postoje zajednički međubazni strani ključevi.

U modelu podataka koriste se UUID identifikatori, decimalne količine i novčani iznosi, datumi i JSONB strukture. Ograničenja CHECK i UNIQUE sprečavaju pojedine nedozvoljene vrednosti i duplikate. Transakcije, zaključavanje redova i uslovne izmene štite konkurentne operacije nad zalihama. Okidači baze dodatno obezbeđuju upis integracionih događaja i zabranu izmena zatvorenih proizvodnih serija. Time se deo važnih pravila sprovodi i na nivou trajnog čuvanja podataka.

SQLx, deklarisan u verziji 0.8.6, omogućava asinhroni pristup PostgreSQL-u uz eksplicitne SQL upite [15]. U projektu se koriste skupovi konekcija, parametrizovani upiti i migracije koje se izvršavaju pri pokretanju servisa. Pojedini upiti koriste makroe za proveru pri prevođenju i sačuvane metapodatke iz direktorijuma .sqlx, dok se dinamički upiti proveravaju pri izvršavanju. Stoga se provera pri prevođenju ne može pripisati svakom upitu u aplikaciji.

Kombinacija PostgreSQL-a i SQLx-a omogućava neposrednu kontrolu nad transakcijama i složenijim upitima potrebnim za rezervacije i projekcije. Cena takvog pristupa je obaveza ručnog održavanja SQL izraza, mapiranja rezultata i usklađenosti sa migracijama. SQLx se u ovom radu koristi kao biblioteka za pristup bazi, a ne kao objektno-relacioni okvir koji automatski upravlja celim životnim ciklusom poslovnih objekata.

## 4.6 RabbitMQ i Lapin

RabbitMQ je posrednik za razmenu poruka kojim su u projektu povezani proizvođači i potrošači integracionih događaja. Biblioteka Lapin, sa eksplicitno navedenom verzijom 2.5.5, omogućava Rust servisima komunikaciju preko AMQP protokola. Topologija sadrži razmenjivač poruka tipa topic, poseban red za projekcije i poseban red za obradu izlaza proizvodnje. Na taj način jedna promena serije može biti nezavisno obrađena radi ažuriranja porekla i radi formiranja proizvoda.

Pouzdanost se oslanja na trajne redove, poruke označene za trajno čuvanje, potvrde objavljivanja i potvrde uspešne obrade. Potvrda brokera odnosi se na objavljenu poruku, dok potvrda potrošača označava završetak obrade na njegovoj strani; to su različite odgovornosti [9]. U projektu se događaj označava kao objavljen tek nakon potvrde, a potrošač projekcije potvrđuje prijem tek nakon potvrđivanja lokalne transakcije.

Ponovna isporuka poruke ostaje moguća, na primer ako je obrada završena neposredno pre prekida veze. Zato RabbitMQ nije dovoljan bez aplikacionog prepoznavanja duplikata. Ovaj odnos povezuje izbor brokera sa obrascima Transactional Outbox i Idempotent Consumer, opisanim u odeljku 5.6. Asinhrona komunikacija smanjuje vremensku zavisnost servisa, ali zahteva obradu privremeno neusaglašenih prikaza i praćenje neuspešnih poruka.

## 4.7 Serijalizacija i pomoćni tipovi podataka

Serde obezbeđuje serijalizaciju i deserijalizaciju struktura, dok serde_json podržava JSON format [16]. U LocalBite-u ovi alati povezuju HTTP zahteve, odgovore i integracione događaje sa Rust tipovima. Namenske strukture za prenos podataka odvojene su od modela baze. Na primer, odgovor za detalje serije objedinjuje podatke o seriji, koracima i sirovinama koji se trajno čuvaju odvojeno.

Biblioteke uuid, chrono i bigdecimal koriste se za identifikatore, vremenske vrednosti i decimalne brojeve. UUID omogućava stvaranje identiteta objekata i operacija unutar različitih servisa. Chrono razlikuje datume i vremenske zapise, dok BigDecimal podržava rad sa decimalnim vrednostima iz baze. Pošto pojedini HTTP ugovori koriste brojeve tipa f64, ograničenja preciznosti i pretvaranja vrednosti moraju se proveravati na granici sistema; sam izbor decimalnog tipa u bazi ne uklanja sve rizike zaokruživanja.

## 4.8 Autentifikacija i zaštita pristupa

Identitet korisnika prenosi se pomoću JSON Web Token formata [7]. Token u projektu sadrži identifikator korisnika, adresu elektronske pošte, ulogu, pripadnost gazdinstvu i podatke o vremenu važenja. Klijent ga dostavlja u zaglavlju Authorization, a server proverava token pre izvršavanja zaštićene operacije. Poslovna autorizacija zatim proverava ulogu i pripadnost traženog objekta gazdinstvu.

Lozinke se obrađuju algoritmom Argon2, uz nasumičnu so. Argon2 je namenjen heširanju lozinki sa troškom rada i memorije koji otežava masovno isprobavanje kandidata [17]. U bazi se čuva rezultat heširanja, a pri prijavi proverava se uneta lozinka. To je različito od reverzibilnog šifrovanja. Biblioteka jsonwebtoken obrađuje tokene, dok argon2 obezbeđuje odvojenu odgovornost zaštite lozinki.

Servisi koriste zajedničku simetričnu tajnu za tokene, što pojednostavljuje konfiguraciju, ali proširuje granicu poverenja između njih. Tehnički tokeni služe ograničenim internim operacijama, kao što je čitanje porekla. Provere u Angular interfejsu upravljaju navigacijom i vidljivošću akcija, ali ne zamenjuju obavezne serverske provere.

## 4.9 Angular i TypeScript

Angular omogućava organizaciju korisničkog interfejsa kroz komponente, rutiranje, obrasce i ubrizgavanje zavisnosti [11]. Projekat koristi Angular 21, dok je za TypeScript deklarisan opseg verzije 5.9. TypeScript dopunjuje JavaScript statičkim opisima tipova [18], što olakšava definisanje modela korisnika, proizvodnih serija, proizvoda i odgovora servisa.

Interfejs je organizovan kao aplikacija sa jednom stranicom i samostalnim komponentama. Funkcionalne celine učitavaju se kroz rute, a odloženo učitavanje omogućava da se određeni delovi aplikacije pribave tek pri navigaciji. Reaktivni obrasci povezuju unos podataka sa validacijom, dok zajednički prikazi grešaka ujednačavaju povratne informacije korisniku. Angular Material je prisutan među zavisnostima, ali značajan deo izgleda ostvaren je sopstvenim HTML i CSS rešenjima.

Stanje komponenata prati se pomoću signala i izvedenih vrednosti. Promena izabranog filtera, statusa učitavanja ili podataka serije tako se povezuje sa osvežavanjem prikaza. API servisi izdvajaju komunikaciju sa serverskim delom, a HTTP interceptor dodaje pristupni token. Ovakva organizacija smanjuje ponavljanje mrežnog i autentifikacionog koda u pojedinačnim ekranima.

## 4.10 RxJS i reaktivna obrada

RxJS omogućava predstavljanje tokova vrednosti i događaja kroz Observable objekte i njihovu obradu pomoću operatora [19]. U projektu Angular HttpClient vraća ovakve tokove, pa se odgovori mogu transformisati, povezati sa prikazom greške i obraditi nakon prijema. Reaktivna pretplata koristi se i za periodično osvežavanje podataka.

Upravljački pregled se osvežava u intervalima od pet sekundi, uz prekid pretplate kada komponenta prestane da se koristi. Javni prikaz porekla može ograničeno ponavljati zahtev kada dobije odgovor da se projekcija još usaglašava. Ovi primeri pokazuju kako je asinhrona priroda servera predstavljena u interfejsu. Ponavljanje operacije koja menja podatke, međutim, zahteva zasebnu zaštitu idempotentnosti i ne može se bezuslovno primeniti na svaki zahtev.

## 4.11 Obrada slika, QR kodova i PDF dokumenata

Biblioteka image koristi se za obradu fotografija proizvoda, dok qrcode generiše kod koji sadrži adresu javnog prikaza porekla. Obrada fotografije obuhvata prilagođavanje dimenzija i čuvanje u JPEG formatu. Slike i QR kodovi smeštaju se u direktorijum servisa proizvoda povezan sa trajnim volumenom, a baza čuva njihove putanje. QR kod je sredstvo pristupa podacima i ne predstavlja digitalni potpis niti nezavisnu potvrdu njihovog sadržaja.

Dokument o poreklu generiše se u Python-u pomoću ReportLab biblioteke. Njen Platypus sloj omogućava sastavljanje dokumenta od paragrafa, tabela i drugih elemenata [20]. Servis proizvoda priprema javni skup podataka, prosleđuje JSON pomoćnom procesu preko standardnog ulaza i preuzima PDF preko standardnog izlaza. Python proces je pomoćna komponenta istog servisa, a ne poseban poslovni mikroservis.

Generisanje dokumenta organizovano je primenom obrasca Builder, tako da su redosled sastavljanja sadržaja i konkretan način PDF prikaza razdvojeni. Tekst koji korisnik unosi obrađuje se pre prosleđivanja elementima dokumenta, a trajanje procesa je ograničeno. Ovakva kombinacija omogućava da osnovna poslovna aplikacija ostane u Rust-u, dok se za raspored sadržaja dokumenta koristi namenska biblioteka.

## 4.12 Docker Compose i Nginx

Docker Compose opisuje više povezanih kontejnera, njihove mreže, volumene i konfiguraciju u zajedničkoj YAML datoteci [21]. U projektu se koristi za pokretanje servisa, baza, brokera, korisničkog interfejsa i ulaznog posrednika. Višefazna izgradnja odvaja alate za prevođenje od izvršne slike. Frontend se gradi pomoću Node.js okruženja i npm alata, nakon čega se dobijene statičke datoteke poslužuju preko Nginx-a.

Nginx ima ulogu obrnutog posrednika koji prosleđuje HTTP zahteve odgovarajućem odredištu [22]. Korisnički interfejs koristi zajednički prefiks /api, dok konfiguracija određuje nadležni servis. Time se unutrašnje adrese servisa izdvajaju iz klijentskih komponenata. Javni QR pristup koristi ograničen skup ruta; pomoćna skripta može obezbediti privremeni javni pristup radi demonstracije.

Kontejnerizacija pojednostavljuje uspostavljanje razvojnog okruženja, ali postojeća konfiguracija sama po sebi ne dokazuje visoku raspoloživost. Više replika, upravljanje tajnama, deljeno skladište datoteka i oporavak infrastrukture zahtevaju dodatno planiranje. U okviru rada Compose prvenstveno predstavlja ponovljiv način pokretanja i integracione provere sistema.

## 4.13 Alati za proveru i dijagnostiku

Provera sistema obuhvata Rust testove, frontend testove zasnovane na Vitest i jsdom zavisnostima, kao i integracione skripte u Node.js, Python i PowerShell okruženjima. Skripte proveravaju poslovne tokove poput rezervacije zaliha, nastanka proizvoda, obnove projekcija i generisanja dokumenta. Jedinični test i provera celog distribuiranog toka imaju različite ciljeve: prvi izdvaja lokalno pravilo, dok drugi proverava saradnju procesa i baza.

Biblioteke tracing i tracing-subscriber koriste se za strukturisano beleženje događaja i grešaka, uz podešavanje nivoa zapisa kroz okruženje. Takvi zapisi pomažu dijagnostici, ali ne predstavljaju potpunu infrastrukturu za distribuirano praćenje. Pregled osnovnih tehnologija i njihovih odgovornosti dat je u tabeli 5, dok je detaljnija analiza provere sistema izložena u poglavlju 11.

[[TECHNOLOGIES]]

Tabela 5: Tehnologije i njihova primena
