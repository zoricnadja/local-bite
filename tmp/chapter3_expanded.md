# 3 Analiza domena i funkcionalni zahtevi

Analiza domena obuhvata učesnike u radu platforme, poslovne objekte i pravila koja određuju njihove međusobne odnose. U sistemu LocalBite osnovni poslovni tok započinje evidentiranjem sirovina, nastavlja se kroz proizvodnu seriju i njene korake, a završava nastankom proizvoda i obradom porudžbina. Podaci nastali u ovom toku koriste se i za javni prikaz porekla proizvoda. U ovom poglavlju funkcionalni zahtevi razmatraju se u skladu sa realizovanim servisima, proverama ulaznih podataka i podržanim promenama stanja.

## 3.1 Akteri i korisničke uloge

Platforma razlikuje pet kategorija krajnjih korisnika: posetioca, kupca, vlasnika gazdinstva, radnika i administratora sistema. Njihova ovlašćenja određuju se prema korisničkoj ulozi i, kada je to primenljivo, pripadnosti gazdinstvu. Pripadnost gazdinstvu predstavlja osnov za razdvajanje internih poslovnih podataka više proizvođača koji koriste istu platformu.

Posetilac pristupa javnom prikazu porekla preko QR koda, bez registracije i prijave. Dostupni su mu podaci namenjeni javnom prikazu i dokument o poreklu u PDF formatu. Ovakav pristup ne podrazumeva ovlašćenje za pregled interne evidencije gazdinstva, podataka o zaposlenima ili porudžbina drugih korisnika.

Kupac je registrovani korisnik sa ulogom CUSTOMER. Pregleda prodajnu ponudu, koristi dostupne filtere, dodaje proizvode u korpu i kreira porudžbine. Može da pregleda sopstvene porudžbine i njihove statuse, kao i da uređuje svoj profil. Kreiranje porudžbine zahteva autentifikaciju, dok javni prikaz porekla ostaje dostupan i bez korisničkog naloga.

Vlasnik gazdinstva, označen ulogom FARM_OWNER, uspostavlja gazdinstvo i upravlja njegovim poslovnim podacima. Pored evidencije sirovina, proizvodnih serija i proizvoda, vlasnik dodaje radnike, pregleda njihove naloge, obrađuje porudžbine i pristupa poslovnoj analitici. U postojećem modelu vlasnik može da kreira jedno gazdinstvo. Prilikom njegovog kreiranja korisnički nalog povezuje se sa gazdinstvom, a podaci sesije dopunjuju odgovarajućim identifikatorom.

Radnik, sa ulogom WORKER, obavlja operativne poslove u okviru gazdinstva kojem pripada. Njegova ovlašćenja obuhvataju rad sa sirovinama, proizvodnim serijama, koracima proizvodnje i proizvodima, kao i dozvoljene promene statusa porudžbina. Radnik ne dodaje druge zaposlene i nema ovlašćenje za otkazivanje porudžbina, koje je dodeljeno vlasniku. Nalog radnika kreira vlasnik gazdinstva; ova uloga nije ponuđena kroz javnu registraciju.

Administrator sistema ima ulogu SYSTEM_ADMIN i pristup određenim administrativnim pregledima. Obim njegovih ovlašćenja zavisi od konkretne operacije, pa se ova uloga ne tumači kao neograničeno pravo izmene svih poslovnih podataka. Javna registracija dozvoljava izbor uloge kupca ili vlasnika, čime se sprečava samostalno dodeljivanje radničke ili administratorske uloge.

Pored korisničkih uloga, implementacija koristi tehnička ovlašćenja za međuservisnu komunikaciju. Na primer, TRACEABILITY omogućava ograničeno čitanje podataka o poreklu, dok se posebna ovlašćenja koriste za rezervaciju proizvoda i promenu stanja sirovina. Takva ovlašćenja nisu zasebni poslovni akteri niti nalozi za prijavu kroz korisnički interfejs. Pregled korisničkih uloga prikazan je u tabeli 3.

[[ROLES]]

Tabela 3: Uloge i dozvole

## 3.2 Osnovni pojmovi i odnosi u domenu

Gazdinstvo predstavlja organizacionu celinu kojoj pripadaju radnici, sirovine, proizvodne serije i proizvodi. Korisnička uloga određuje vrstu dozvoljenih operacija, dok pripadnost gazdinstvu ograničava skup poslovnih objekata nad kojima se te operacije mogu izvršavati. Kupac se sa gazdinstvom povezuje kroz porudžbinu, bez sticanja pristupa njegovoj internoj evidenciji.

Sirovina predstavlja evidentiranu ulaznu zalihu namenjenu proizvodnji. Osnovni podaci obuhvataju naziv, tip, količinu i jedinicu mere, uz mogućnost navođenja porekla, dobavljača, datuma berbe ili proizvodnje, prijema i isteka roka trajanja. Može se definisati i prag niske zalihe. Katalog tipova sirovina sadrži zajedničke vrednosti i vrednosti koje korisnici dodaju za sopstveno gazdinstvo. Ponovljeni unos istog naziva, bez obzira na razliku između velikih i malih slova, vraća postojeći tip.

Proizvodna serija objedinjuje evidenciju jednog proizvodnog toka. Sadrži naziv, datume, napomene, status, korišćene sirovine, proizvodne korake i izlazne proizvode. U aktuelnom modelu seriji se ne dodeljuje poseban tip proizvodnje. Priroda postupka opisuje se nazivom serije, njenim koracima i parametrima tih koraka, što omogućava primenu istog modela na različite proizvodne postupke.

Veza sirovine i serije sadrži utrošenu količinu i istorijski snimak podataka o sirovini. Snimak obuhvata naziv, tip, jedinicu mere, poreklo, dobavljača i evidentirane datume. Time se podaci relevantni za konkretnu proizvodnju čuvaju nezavisno od naknadnog uređivanja izvornog zapisa o sirovini. Ista sirovina ne može se više puta dodati kao zasebna veza unutar iste serije.

Proizvodni korak opisuje pojedinačnu fazu, poput soljenja, dimljenja ili sušenja. Svaki korak ima naziv, redni broj, status, opcioni opis i listu parametara. Parametar se predstavlja parom naziva i tekstualne vrednosti, na primer „Temperatura — 18 °C”, „Trajanje — 48 h” ili „Vlažnost — 75 %”. Nazivi parametara moraju biti jedinstveni unutar koraka, a lista može biti prazna. Ovaj model omogućava evidentiranje različitih uslova procesa, ali ne podrazumeva automatsko tumačenje jedinica mere, proračun parametara ili merenje pomoću senzora.

Jedna proizvodna serija može imati više izlaznih proizvoda. Za svaki izlaz evidentiraju se identitet, naziv, tip proizvoda, količina, jedinica mere i cena, uz mogućnost navođenja dodatnih podataka, poput roka trajanja. Tokom proizvodnje izlazi predstavljaju planirane proizvode. Pri završavanju serije potvrđuju se ostvarene količine, a za izlaze koji su prethodno sačuvani zadržava se i ranije evidentirani plan. Na taj način razlikuju se planirani rezultat proizvodnje i potvrđeni izlaz.

Proizvod predstavlja pojedinačni izlaz serije koji se prati kroz proizvodnju, skladište i prodajnu ponudu. Porudžbina povezuje kupca sa jednim gazdinstvom i sadrži stavke sa količinama i cenama. Jedan zahtev za kupovinu može obuhvatiti više gazdinstava, ali se iz njega formiraju odvojene porudžbine za svakog proizvođača. Naziv proizvoda, jedinica mere i cena čuvaju se u stavci porudžbine kao vrednosti zabeležene pri poručivanju.

## 3.3 Funkcionalni zahtevi

Funkcionalni zahtevi grupisani su prema poslovnim celinama: upravljanje korisnicima i gazdinstvima, evidencija sirovina, praćenje proizvodnje, upravljanje proizvodima, poručivanje i prikaz porekla. Zahtev obuhvata očekivano ponašanje sistema i uslove pod kojima se operacija prihvata. Zbog distribuirane organizacije aplikacije, katalog uključuje i funkcije pouzdanog prenosa promena i oporavka podataka za čitanje. Pregled zahteva dat je u tabeli 4.

[[REQUIREMENTS]]

Tabela 4: Katalog funkcionalnih zahteva

Zahtevi F-25 do F-28 posebno izdvajaju proširivost kataloga sirovina, prilagodljive parametre koraka, podršku za više izlaznih proizvoda i upravljanje statusima koraka. Proširivost nije jednaka za sve poslovne objekte: tip sirovine može se dopuniti u okviru gazdinstva, dok se tipovi izlaznih proizvoda pri završavanju serije proveravaju prema unapred definisanom skupu. Podržani tipovi izlaza su meat, dairy, vegetable, fruit, cheese, sausage, honey i other, a jedinice mere kg, g, l, ml i pcs.

## 3.4 Poslovna pravila i životni ciklusi

Proizvodna serija nastaje u statusu PLANNED. Pokretanje bilo kog njenog koraka, prelaskom koraka u IN_PROGRESS, automatski postavlja seriju u IN_PROGRESS i beleži datum početka ukoliko ranije nije naveden. Korak zatim može preći u COMPLETED. Preskakanje početnog stanja ili vraćanje koraka u prethodni status nije podržano. Redni brojevi koraka moraju biti pozitivni i jedinstveni unutar serije; oni određuju redosled prikaza, ali implementacija ne zahteva da se svi koraci pokreću strogo prema tim brojevima.

Završetak svih koraka ne završava automatski proizvodnu seriju. Korisnik zasebno potvrđuje završetak proizvodnje i konačne izlaze. Prelazak serije u COMPLETED zahteva najmanje jedan korak, završene sve postojeće korake i najmanje jedan izlazni proizvod. Za svaki izlaz proveravaju se naziv, podržani tip i jedinica mere, pozitivna količina i nenegativna cena. Serija može biti otkazana iz planiranog ili aktivnog stanja, prelaskom u CANCELLED. Završene i otkazane serije ne prihvataju izmene kroz operaciju ažuriranja serije, a njihovi koraci ne mogu se dodavati, menjati ili brisati.

Životni ciklus proizvoda razlikuje stanja PRODUCTION, STORAGE i ON_SALE. Planirani izlazi prikazuju se kao proizvodi u proizvodnji i uređuju kroz odgovarajuću seriju. Nakon potvrđenog završetka serije prelaze u skladište, pri čemu ne postaju automatski dostupni za kupovinu. Stavljanje u prodaju zahteva pozitivnu cenu, završenu pripadajuću seriju i važeći rok trajanja, kada je naveden. Proizvod se ne kreira nezavisno od proizvodnje kroz uobičajenu operaciju dodavanja proizvoda. Nakon prenosa u skladište, količina potvrđenog proizvodnog izlaza, jedinica mere i pripadnost seriji ne mogu se proizvoljno menjati kroz uređivanje proizvoda.

Pri evidentiranju utroška sirovine zahteva se pozitivna količina, najmanje 0,001, sa najviše tri decimale, usklađena jedinica mere i dovoljno raspoložive zalihe. Nije podržana automatska konverzija između jedinica, pa količina navedena u gramima ne može neposredno umanjiti zalihu evidentiranu u kilogramima. Umanjenje stanja štiti se od konkurentnih zahteva i ponavljanja iste operacije. Uklanjanje veze sirovine iz serije ne predstavlja automatski povraćaj ranije utrošene količine; ovo ograničenje treba razlikovati od kompenzacije neuspešno izvršene distribuirane operacije.

Porudžbina nastaje u stanju PENDING, nakon uspešne rezervacije potrebnih količina. Podržani redovni tok je PENDING, CONFIRMED, SHIPPED i DELIVERED. Otkazivanje je dozvoljeno iz stanja PENDING ili CONFIRMED i rezervisano je za vlasnika gazdinstva. Otkazivanje upisuje trajni zadatak za oslobađanje rezervisane zalihe, čije izvršavanje može biti ponovljeno ako servis proizvoda privremeno nije dostupan. Logičko brisanje dozvoljeno je samo za porudžbine koje čekaju obradu ili su otkazane, uz odgovarajuće oslobađanje zalihe.

Pri poručivanju se proveravaju dostupnost proizvoda, tražena količina i cena korišćena za rezervaciju. Stavke koje se odnose na isti proizvod objedinjuju se, a količine moraju biti pozitivne i imati najviše tri decimale. Ponovljeni zahtev sa istim ključem idempotentnosti i istim sadržajem nastavlja postojeću obradu ili vraća njen sačuvani rezultat. Upotreba istog ključa za drugog korisnika ili drugačiji zahtev odbija se kao konflikt. Time se smanjuje mogućnost da ponavljanje zahteva nakon prekida komunikacije proizvede duplu porudžbinu ili višestruko umanjenje zalihe.

Promene proizvoda nastale iz proizvodnih događaja i objedinjeni prikaz porekla ažuriraju se asinhrono. Zbog toga uspešan završetak operacije u jednom servisu ne podrazumeva da su svi povezani prikazi istog trenutka osveženi. Sistem mora razlikovati privremeno neusaglašene podatke od nepostojećeg ili nedostupnog proizvoda. Trenutno stanje prodajne dostupnosti proverava se u servisu proizvoda, dok se povezani podaci o poreklu preuzimaju iz modela za čitanje.

## 3.5 Glavni slučajevi korišćenja

### 3.5.1 Uspostavljanje gazdinstva i dodavanje radnika

Preduslov je prijavljen korisnik sa ulogom vlasnika koji još nema gazdinstvo. Vlasnik unosi naziv i adresu, uz opcione kontaktne i opisne podatke. Sistem proverava postojeću povezanost vlasnika sa gazdinstvom, kreira zapis i izdaje osveženi token sa identifikatorom gazdinstva. Nakon toga vlasnik može da doda radnika unosom podataka potrebnih za njegov nalog. Sistem proverava pripadnost gazdinstva vlasniku i jedinstvenost adrese elektronske pošte. Ishod je gazdinstvo sa povezanim korisnicima i ovlašćenjima potrebnim za vođenje evidencije.

### 3.5.2 Evidentiranje sirovina i planiranje proizvodnje

Vlasnik ili radnik evidentira novu sirovinu unosom osnovnih podataka o zalihi i, po potrebi, podataka o poreklu i datumima. Tip bira iz dostupnog kataloga ili dodaje novu vrednost za svoje gazdinstvo. Zatim kreira proizvodnu seriju i bira sirovine koje će se koristiti, sa odgovarajućim količinama i jedinicama mere. Sistem proverava pripadnost sirovina gazdinstvu i raspoloživost zalihe, evidentira utrošak i čuva istorijske podatke u seriji. Utrošak se evidentira već pri povezivanju sirovine sa serijom, a ne tek pri njenom završavanju. Nedovoljna količina ili neusaglašena jedinica mere dovode do odbijanja operacije.

Planiranje se nastavlja dodavanjem proizvodnih koraka i izlaznih proizvoda. Korisnik određuje nazive i redne brojeve koraka, unosi opis postupka i parametre koje želi da prati. Za izlaze definiše očekivane proizvode i njihove količine. Ishod je serija u planiranom stanju sa povezanim ulazima, opisanim fazama rada i evidentiranim očekivanim rezultatima.

### 3.5.3 Izvršavanje koraka i potvrđivanje proizvodnog izlaza

Korisnik pokreće proizvodni korak, čime se aktivira i serija. Tokom rada evidentira relevantne parametre i završava pojedinačne korake promenom njihovog statusa. Kada su svi koraci završeni, potvrđuje konačne proizvode i ostvarene količine. Sistem odbija završetak ako nema koraka, ako neki korak nije završen ili ako izlazni podaci ne zadovoljavaju propisana pravila. Nakon prihvatanja, serija dobija status COMPLETED, a događaj o promeni omogućava prenos izlaznih proizvoda u skladište.

Na primer, jedna serija može obuhvatiti više gotovih proizvoda nastalih iz istog proizvodnog postupka. Svaki izlaz zadržava sopstveni identifikator i količinu, dok se svi povezuju sa zajedničkom serijom i njenim sirovinama i koracima. Zaštita od ponovne obrade istog ili starijeg događaja sprečava da višestruka isporuka poruke proizvoljno uveća evidentiranu zalihu. Zbog asinhrone obrade, prikaz skladišta može postati ažuran naknadno.

### 3.5.4 Priprema proizvoda za prodaju

Preduslov je proizvod prenet iz završene serije u skladište. Vlasnik ili radnik uređuje prodajne podatke, postavlja pozitivnu cenu i po potrebi dodaje opis i sliku. Zahtev za prelazak u prodajnu ponudu pokreće proveru završetka proizvodnje, pripadnosti gazdinstvu i roka trajanja. Ako su uslovi ispunjeni, proizvod dobija stanje ON_SALE. U suprotnom, sistem odbija aktiviranje i proizvod ostaje van prodajne ponude. Planirani proizvodi ne mogu se aktivirati kao gotova roba pre završetka serije.

### 3.5.5 Kreiranje i obrada porudžbine

Prijavljeni kupac bira proizvode i količine i potvrđuje sadržaj korpe. Sistem proverava stavke, čuva podatke potrebne za nastavak obrade i zahteva rezervaciju zalihe. Nakon uspešne rezervacije stavke grupiše prema gazdinstvu i formira odgovarajuće porudžbine, sa zabeleženim cenama. Ako je proizvod postao nedostupan, zaliha nije dovoljna ili se cena promenila, zahtev se ne prihvata kao uspešno završena kupovina. Pri privremenom prekidu komunikacije obrada može biti nastavljena istim identifikatorom zahteva.

Vlasnik ili radnik zatim menja status porudžbine u skladu sa dozvoljenim prelazima. Kupac prati status sopstvenih porudžbina, dok vlasnik može da otkaže porudžbinu pre slanja. Otkazivanje pokreće oslobađanje prethodno rezervisanih količina. Oznake slanja i isporuke predstavljaju ručno evidentirane poslovne statuse; u aktuelnom rešenju one nisu potvrde dobijene od integrisane kurirske službe niti dokaz izvršenog plaćanja.

### 3.5.6 Pregled porekla proizvoda

Posetilac skenira QR kod i otvara javnu stranicu odgovarajućeg proizvoda. Sistem proverava token i trenutnu dostupnost proizvoda, a zatim objedinjuje podatke o gazdinstvu, proizvodnoj seriji, njenim koracima i korišćenim sirovinama. Posetilac može da pregleda podatke i preuzme dokument o poreklu u PDF formatu. Ako projekcija još nije usaglašena, prikazuje se odgovarajuće obaveštenje ili raspoloživi nepotpuni podaci, uz mogućnost ponovnog pokušaja. Prikaz se zasniva na evidentiranim informacijama i ne dopunjava nedostajuće podatke pretpostavkama.

### 3.5.7 Pregled poslovnih pokazatelja

Vlasnik pristupa analitičkom pregledu za svoje gazdinstvo i izabrani period. Sistem prikazuje broj porudžbina, raspodelu po statusima, mesečne iznose i pregled najprodavanijih proizvoda. Prihod se računa na osnovu porudžbina označenih kao isporučene, pa predstavlja pokazatelj izveden iz evidencije prodaje, a ne potvrdu finansijske naplate. Upravljački pregled dopunjuje operativnu evidenciju i omogućava sagledavanje rezultata bez pojedinačnog otvaranja svake porudžbine.

## 3.6 Ograničenja funkcionalnog obuhvata

Realizovani model omogućava prilagođavanje proizvodnje kroz nazive serija, uređene korake, tekstualne parametre i više izlaznih proizvoda. Međutim, ne obuhvata automatski proračun prinosa, obaveznu jednakost ulazne i izlazne mase, recepture sa normativima niti automatsku konverziju jedinica mere. Parametri procesa evidentiraju se ručno i ne predstavljaju automatski prikupljena merenja. Ova ograničenja određuju granicu između evidencije proizvodnje i sistema za automatizovano upravljanje tehnološkim procesom.

Poručivanje je podržano kao evidencija zahteva kupca, rezervacije zalihe i promena poslovnog statusa. Elektronsko plaćanje, fiskalizacija i integracija dostave nisu deo ovog toka. Takođe, podaci o poreklu predstavljaju sledljivost zapisa koje su uneli ovlašćeni korisnici, bez nezavisne sertifikacije njihove tačnosti. Ovakvo razgraničenje omogućava da se ostvarene funkcionalnosti procenjuju prema stvarnom obuhvatu implementacije.
