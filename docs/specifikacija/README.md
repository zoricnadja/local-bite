# Local Bite — detaljna specifikacija sistema

Datum pregleda: **18. septembar 2026.** Dokumentacija opisuje trenutno radno stablo projekta, uključujući lokalne izmene koje nisu commitovane. Migracije do prefiksa `20260919` postoje u ovom stablu i uključene su u opis, bez pretpostavke da su već izvršene na konkretnoj bazi.

Ovo je specifikacija izvedena iz implementacije: opisuje šta sistem radi, kako je organizovan i gde postoje preostala ograničenja. Dokument 12 navodi konkretno izvršene testove; to i dalje nije potvrda produkcione spremnosti. Kod i SQL migracije imaju prednost nad starijim README dokumentima i tekstom diplomskog rada.

## Kako čitati dokumentaciju

| Dokument | Sadržaj |
|---|---|
| [01 — Domen i funkcionalni zahtevi](01-domen-i-funkcionalnosti.md) | Svrha, akteri, obuhvat, slučajevi korišćenja, poslovna pravila i statusi |
| [02 — Tehnologije](02-tehnologije.md) | Detaljan tehnološki sloj i konkretna primena svake tehnologije |
| [03 — Arhitektura](03-arhitektura.md) | Granice servisa, slojevi, baze, deployment, arhitektonske odluke |
| [04 — Backend servisi](04-servisi.md) | Odgovornosti i unutrašnji delovi svih šest backend servisa i zajedničke biblioteke |
| [05 — Modeli i podaci](05-modeli-i-podaci.md) | Rečnik svih tabela, relacije, istorijski snimci, tipovi i ograničenja |
| [06 — API ugovori](06-api.md) | HTTP rute, pristup, odgovori, filteri i primeri zahteva |
| [07 — Povezanost i tokovi](07-integracije-i-tokovi.md) | REST zavisnosti, RabbitMQ, outbox, projekcije, kompenzacije i kvarovi |
| [08 — Frontend](08-frontend.md) | Ekrani, Angular slojevi, sesija, forme, dozvole i prikaz podataka |
| [09 — Softverski obrasci](09-softverski-obrasci.md) | Aktivni, delimični i neaktivni obrasci, sa dokazima iz koda |
| [10 — Bezbednost i kvalitet](10-bezbednost-i-kvalitet.md) | Autorizacija, privatnost, nefunkcionalni zahtevi i konkretne nedoslednosti |
| [11 — Pokretanje i održavanje](11-pokretanje-i-odrzavanje.md) | Konfiguracija, portovi, build, migracije, monitoring, replay i backup |
| [12 — Testiranje i razvoj](12-testiranje-i-razvoj.md) | Postojeći testovi, matrica verifikacije, prioriteti i usklađivanje dokumentacije |
| [13 — Katalog tipova](13-katalog-tipova.md) | Polja Rust modela i DTO struktura iz izvornog koda |
| [14 — Mapa izvornog koda](14-mapa-izvornog-koda.md) | Inventar programskih, konfiguracionih i pomoćnih fajlova sa ulogama |
| [15 — Oporavak i operacije](15-oporavak-i-operacije.md) | Backup/restore, retention, performance baseline i alarmi |

Preporučen redosled za razumevanje: 01 → 03 → 04 → 05 → 07 → 08 → 09. Dokument 02 je detaljna tehnološka referenca, 06 i 13 služe pri implementaciji, a 10–12 pri evaluaciji i održavanju.

## Kratak opis sistema

Local Bite prati put hrane kroz **sirovine → proizvodne serije → skladište gotovih proizvoda → prodaju i porudžbine → javni prikaz porekla preko QR koda**. Namenjen je radu više gazdinstava u jednoj aplikaciji. Gazdinstvo je poslovna granica podataka; vlasnik i radnici upravljaju njegovom proizvodnjom, a kupac pregleda ponudu i poručuje.

Implementirano je pet domenskih servisa (`auth`, `raw-materials`, `productions`, `products`, `orders`) i šesti servis za čitanje projekcija (`read-models`). Svaki ima zasebnu PostgreSQL bazu. Angular aplikacija pristupa backendu kroz Nginx. REST pozivi služe za neposrednu validaciju i komande koje zavise od drugog servisa; RabbitMQ prenosi integracione događaje i omogućava održavanje projekcija i automatsko kreiranje proizvodnog izlaza u skladištu.

## Oznake i nivo pouzdanosti

- **Implementirano**: tok ili struktura potvrđeni čitanjem izvornog koda.
- **Delimično / ograničenje**: postoji mehanizam, ali ne pokriva sve granične slučajeve ili sve ulazne putanje.
- **Predlog**: dopuna za narednu iteraciju; ne treba je predstavljati kao postojeću funkcionalnost.
- **Postoji u kodu, nije aktivno**: sačuvana deklaracija ili pomoćna funkcija bez poziva u aktuelnom toku.

Pregled obuhvata backend, frontend, SQL migracije, Docker/Nginx konfiguraciju, skripte, zajedničku biblioteku, testove i izvorne materijale diplomskog rada. Generisani PDF/DOCX, binarni `typst.exe`, biblioteke iz `node_modules` i build rezultati nisu izvor poslovnog ponašanja. Tajne iz `.env` nisu prepisane u ovu dokumentaciju.

## Bitne razlike u odnosu na početni opis

1. Postoji šest, a ne samo pet backend servisa.
2. Komunikacija je kombinacija REST-a i RabbitMQ događaja.
3. Novi proizvod nastaje iz završene proizvodne serije. `POST /products` je sačuvan, ali trenutno odbija kreiranje.
4. Skladište i ponuda koriste istu tabelu proizvoda, uz različitu vidljivost.
5. CQRS je primenjen na dashboard i poreklo; nije svako čitanje prebačeno u read-model servis.
6. Nema punog Event Sourcing-a: izvor istine ostaju domenske relacione baze.
7. Strategy/Simple Factory aktivno validiraju izmene i proizvodni izlaz. PDF koristi `CertificateBuilder`, `ReportLabCertificateBuilder` i `CertificateDirector`.
8. Grafikoni analitike su HTML/CSS prikazi, bez zavisnosti od Chart.js.
9. Python je korišćen za PDF, dok AI predikcija potražnje nije implementirana u pregledanom kodu.

Za tačne granice autorizacije i konzistentnosti obavezno pročitati [10 — Bezbednost i kvalitet](10-bezbednost-i-kvalitet.md). Izolacija podataka i dalje zavisi od dosledne primene objektne autorizacije na svakoj novoj ruti; prodajni tok sada koristi atomsku rezervaciju po stavci, trajni checkout intent i kompenzaciono vraćanje zalihe.
