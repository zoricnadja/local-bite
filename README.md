# Local Bite – Mikroservisna platforma za praćenje proizvodnje i porekla domaćih proizvoda

## Tip teme

**Samostalno definisana tema** sa jasnim osloncem na mikroservisnu arhitekturu. Tema je pogodna i kao **predefinisana mikroservisna**, ali se prijavljuje kao samostalno definisana kako bi se ostavio prostor za proširenja i diplomski rad.

---

## 1. Uvod i motivacija

Local Bite je koncipiran kao generička, skalabilna platforma za mala poljoprivredna gazdinstva i porodične proizvođače, sa ciljem da se lako prilagodi različitim tipovima proizvodnje (mesni proizvodi, prerađevine od voća i povrća, mlečni proizvodi, med i slično). Sistem nije vezan za jedno konkretno gazdinstvo, već je dizajniran da podrži veliki broj nezavisnih proizvođača u okviru jedne aplikacije.

Ideja za projekat proistekla je iz realne potrebe malih proizvođača koji nemaju digitalizovan sistem za praćenje porekla, sirovina i procesa proizvodnje. U praksi se većina evidencije vodi ručno, bez centralizovanog sistema koji povezuje sirovine, procese proizvodnje i finalne proizvode.

Local Bite ima za cilj da digitalizuje ovaj proces i omogući praćenje kompletnog puta proizvoda – od sirovine, preko prerade, do finalnog proizvoda i prodaje – uz jasan fokus na transparentnost, doslednost i poverenje potrošača.

---

## 2. Opis problema

Trenutni problemi u radu manjih gazdinstava:

* Evidencija se vodi ručno (sveske, Excel), bez povezivanja podataka.
* Ne postoji sistemsko povezivanje sirovina sa finalnim proizvodima.
* Kupci nemaju uvid u poreklo i proces proizvodnje.
* Teško je analizirati proizvodnju i prodaju kroz vreme.

Local Bite rešava ove probleme kroz **mikroservisnu arhitekturu** gde svaki deo sistema ima jasno definisanu odgovornost i **generalizovane entitete** koji se mogu primeniti na bilo koje gazdinstvo ili proizvod.

---

## 3. Ciljevi projekta

U okviru projektnog zadatka biće implementirano:

* Mikroservisna arhitektura sa 5 servisa (moguća dopuna po potrebi).  
* Autentifikacija i autorizacija korisnika.  
* CRUD operacije nad svim glavnim entitetima (`RawMaterial`, `ProcessStep`, `Product`, `Order`).  
* Generisanje QR koda za finalne proizvode.  
* Rad sa slikama proizvoda.  
* Osnovna analitika u formi grafikona.  

Sve funkcionalnosti biće demonstrirane kroz jednostavan frontend interfejs.

---
## 4. Generalizacija sistema i izolacija podataka

Local Bite je od samog početka projektovan kao generički i proširiv sistem, sposoban da podrži širok spektar malih gazdinstava i lokalnih proizvođača hrane, bez potrebe za prilagođavanjem koda za svaki pojedinačni slučaj.

Sistem podržava:

- Više gazdinstava (multi-tenant arhitektura) – svako gazdinstvo ima potpuno izolovane podatke i sopstvene korisnike.
- Različite tipove sirovina – meso, voće, povrće, mlečni proizvodi, začini, itd.
- Različite procese proizvodnje – sušenje, dimljenje, fermentacija, kuvanje, pečenje, pasterizacija.
- Različite tipove finalnih proizvoda – kulen, kobasica, pečenica, slanina, ajvar, sokovi, pekmezi, sirupi.
- Fleksibilne domenske modele – entiteti RawMaterial, ProcessStep i Product su generički i mogu se proširivati dodavanjem novih polja bez promene osnovne arhitekture.

Ova generalizacija omogućava da isti sistem može da koristi veliki broj različitih gazdinstava sa potpuno različitim proizvodnim programima, uz zadržavanje iste arhitekture i logike.

### Izolacija podataka i uloge

Sistem je dizajniran kao multi-tenant platforma sa striktnom izolacijom podataka. Svaki korisnik tipa FarmOwner (vlasnik gazdinstva) i WORKER (radnik) je vezan za tačno jedno gazdinstvo, dok svi entiteti vezani za proizvodnju (sirovine, procesi, proizvodi, porudžbine) sadrže farm_id.

Na osnovu identiteta korisnika i njegove uloge:
- FarmOwner i WORKER mogu pristupati isključivo podacima svog gazdinstva.
- CUSTOMER ima pristup javno dostupnim informacijama svih gazdinstava (npr. pregled proizvoda i porekla).
- SystemAdmin ima globalni uvid u podatke svih gazdinstava.

Filtriranje po farm_id se sprovodi na nivou svakog upita prema bazi, čime se garantuje potpuna izolacija podataka i sprečava bilo kakvo mešanje informacija između različitih proizvođača.

Ovakav pristup omogućava da Local Bite funkcioniše kao jedinstvena platforma za veliki broj nezavisnih proizvođača, uz visok nivo bezbednosti, privatnosti i skalabilnosti.

## 5. Arhitektura sistema

### 5.1 Pregled

Sistem je baziran na mikroservisnoj arhitekturi. Svaki servis ima svoju bazu podataka i jasno definisanu ulogu. Servisi međusobno komuniciraju putem REST API-ja.

Glavne komponente:

* Auth Service  
* Raw Materials Service  
* Production Service  
* Product Service  
* Orders Service  
---

### 5.2 Servisi

#### 5.2.1 Auth Service
**Odgovornost:** autentifikacija i autorizacija korisnika.

Funkcionalnosti:

* Registracija korisnika  
* Prijava (login)  
* JWT tokeni  
* Uloge:
  - SystemAdmin – administrator platforme
  - FarmOwner – vlasnik gazdinstva
  - WORKER – zaposleni
  - CUSTOMER – krajnji korisnik

---

#### 5.2.2 Raw Materials Service (Sirovine)
**Odgovornost:** evidencija svih sirovina koje se koriste u proizvodnji.

Polja i entiteti su **generički**, što omogućava lako dodavanje novih tipova sirovina bez menjanja koda.


---

#### 5.2.3 Production Service (Proces proizvodnje)
**Odgovornost:** praćenje procesa obrade sirovina.  
Svaki proces je **generički entitet**, sa poljima za tip procesa, datum, korišćene sirovine i opis.


---

#### 5.2.4 Product Service (Finalni proizvodi)
**Odgovornost:** upravljanje finalnim proizvodima.  
Polja i entiteti su **generički**, omogućavajući dodavanje novih tipova proizvoda (meso, sokovi, pekmezi, sirupi, itd.) bez izmene osnovne arhitekture.


---

#### 5.2.5 Orders Service (Porudžbine)
**Odgovornost:** evidencija porudžbina i kupaca.  
Omogućava podršku za više gazdinstava i povezivanje sa Product Service-om.


---

## 6. Komunikacija između servisa

* REST API  
* Product Service ↔ Production Service (informacije o procesima)  
* Production Service ↔ Raw Materials Service (informacije o sirovinama)  
* Orders Service ↔ Product Service (validacija proizvoda)  

---

## 7. Tehnologije

### Backend
* Rust (svi mikroservisi)  
* Framework: Axum

### Baze podataka
* PostgreSQL

### Frontend
* Angular

### Dodatno
* Biblioteka za generisanje QR koda  
* Biblioteka za rad sa slikama  
* Biblioteka za grafikone (Chart.js)  

---

## 8. Dodatno za diplomski rad

U okviru diplomskog rada sistem će biti proširen keim od sledećih funkcionalnosti:

* Design patterni i arhitektura - Strategy, Factory, Builder – fleksibilnost i generalizacija proizvoda i procesa, Observer / Event pattern – event-driven komunikacija između servisa, CQRS – razdvajanje upisa i čitanja podataka za skalabilnost
* AI modul (Python) za predikciju potražnje i optimizaciju proizvodnje,
* Asinhrona komunikacija između servisa (message broker),
* Mobilna aplikacija za skeniranje QR koda,
* Generisanje PDF sertifikata o poreklu

---
### 9. Zaključak

Local Bite predstavlja realan i praktičan sistem koji rešava konkretan problem iz domena poljoprivrede i proizvodnje hrane. Projekat je pažljivo definisan tako da bude izvodljiv u okviru projektnog zadatka, ali i dovoljno širok da se prirodno proširi u diplomski rad.
