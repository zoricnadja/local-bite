# AgroTrace – Mikroservisna platforma za praćenje proizvodnje i porekla domaćih proizvoda

## Tip teme

**Samostalno definisana tema** sa jasnim osloncem na mikroservisnu arhitekturu. Tema je pogodna i kao **predefinisana mikroservisna** jer ispunjava sve zahteve iz sekcije 1.1.2, ali se prijavljuje kao samostalno definisana kako bi se ostavio prostor za proširenja i diplomski rad.

---

## 1. Uvod i motivacija

Poljoprivredna gazdinstva koja proizvode domaće proizvode (mesne prerađevine, ajvar, sokovi, pekmezi) često nemaju digitalni sistem za praćenje porekla sirovina, procesa proizvodnje i prodaje. Potrošači sve češće žele transparentnost: odakle proizvod dolazi, kako je proizveden i da li je zaista domaći.

Cilj projekta **AgroTrace** je da omogući **digitalno praćenje celokupnog životnog ciklusa proizvoda**, od sirovine, preko procesa proizvodnje, do finalnog proizvoda i prodaje – uz mogućnost da krajnji korisnik skeniranjem QR koda vidi kompletnu istoriju proizvoda.

---

## 2. Generalizacija sistema

AgroTrace je od samog početka **projektovan kao generički sistem**, sposoban da podrži:

* **Više gazdinstava (multi-tenant)** – svaki gazdinstvo ima izolovane podatke i sopstvene korisnike.  
* **Različite tipove sirovina** – meso, voće, povrće, mlečni proizvodi, začini, itd.  
* **Različite procese proizvodnje** – sušenje, dimljenje, fermentacija, kuvanje, pečenje, pasterizacija.  
* **Različite tipove finalnih proizvoda** – kulen, kobasica, pečenica, slanina, ajvar, sokovi, pekmezi, sirupi.  
* **Fleksibilne domenske modele** – entiteti `RawMaterial`, `ProcessStep` i `Product` su generički i mogu se proširivati dodavanjem novih polja bez promene osnovne arhitekture.  
* **Event-driven ažuriranja** – promene u jednom servisu automatski se propagiraju u povezane servise, omogućavajući jednostavno dodavanje novih servisa ili funkcionalnosti.  

Ova generalizacija znači da **isti sistem može da se primeni na različita gazdinstva** sa različitim vrstama proizvoda bez potrebe za pisanjem novog koda za svaki slučaj.

---

## 3. Opis problema

Trenutni problemi u radu manjih gazdinstava:

* Evidencija se vodi ručno (sveske, Excel), bez povezivanja podataka.
* Ne postoji sistemsko povezivanje sirovina sa finalnim proizvodima.
* Kupci nemaju uvid u poreklo i proces proizvodnje.
* Teško je analizirati proizvodnju i prodaju kroz vreme.

AgroTrace rešava ove probleme kroz **mikroservisnu arhitekturu** gde svaki deo sistema ima jasno definisanu odgovornost i **generalizovane entitete** koji se mogu primeniti na bilo koje gazdinstvo ili proizvod.

---

## 4. Ciljevi projekta

U okviru projektnog zadatka biće implementirano:

* Mikroservisna arhitektura sa najmanje 4 servisa (biće implementirano 5).  
* Autentifikacija i autorizacija korisnika.  
* CRUD operacije nad svim glavnim entitetima (`RawMaterial`, `ProcessStep`, `Product`, `Order`).  
* Generisanje QR koda za finalne proizvode.  
* Rad sa slikama proizvoda.  
* Osnovna analitika u formi grafikona.  

Sve funkcionalnosti biće demonstrirane kroz jednostavan frontend interfejs.

---

## 5. Arhitektura sistema

### 5.1 Pregled

Sistem je baziran na mikroservisnoj arhitekturi. Svaki servis ima svoju bazu podataka i jasno definisanu ulogu. Servisi međusobno komuniciraju putem REST API-ja.

Glavne komponente:

* Auth Service  
* Raw Materials Service  
* Production Service  
* Product Service  
* Orders Service  

> **Napomena:** Napredni design patterni (Strategy, Factory, Builder, Observer, CQRS) biće implementirani u okviru diplomskog rada za dodatnu fleksibilnost i skalabilnost sistema.

---

### 5.2 Servisi

#### 5.2.1 Auth Service
**Odgovornost:** autentifikacija i autorizacija korisnika.

Funkcionalnosti:

* Registracija korisnika  
* Prijava (login)  
* JWT tokeni  
* Uloge: admin (gazdinstvo), radnik, kupac  

Baza: PostgreSQL

---

#### 5.2.2 Raw Materials Service (Sirovine)
**Odgovornost:** evidencija svih sirovina koje se koriste u proizvodnji.

Polja i entiteti su **generički**, što omogućava lako dodavanje novih tipova sirovina bez menjanja koda.

Baza: PostgreSQL

---

#### 5.2.3 Production Service (Proces proizvodnje)
**Odgovornost:** praćenje procesa obrade sirovina.  
Svaki proces je **generički entitet**, sa poljima za tip procesa, datum, korišćene sirovine i opis.

Baza: PostgreSQL

---

#### 5.2.4 Product Service (Finalni proizvodi)
**Odgovornost:** upravljanje finalnim proizvodima.  
Polja i entiteti su **generički**, omogućavajući dodavanje novih tipova proizvoda (meso, sokovi, pekmezi, sirupi, itd.) bez izmene osnovne arhitekture.

Baza: PostgreSQL

---

#### 5.2.5 Orders Service (Porudžbine)
**Odgovornost:** evidencija porudžbina i kupaca.  
Omogućava multi-gazdinstvenu podršku i povezivanje sa Product Service-om.

Baza: PostgreSQL

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
* Framework: Actix-web ili Axum

### Baze podataka
* PostgreSQL

### Frontend
* React ili jednostavan HTML/CSS/JS interfejs

### Dodatno
* Biblioteka za generisanje QR koda  
* Biblioteka za rad sa slikama  
* Biblioteka za grafikone (Chart.js)  

---

## 8. Plan realizacije (1 mesec)

*(isto kao ranije)*

---

## 9. Vizija za diplomski rad (proširenja u narednih 6 meseci)

U okviru diplomskog rada sistem će biti proširen sledećim funkcionalnostima:

* Design patterni i arhitektura - Strategy, Factory, Builder – fleksibilnost i generalizacija proizvoda i procesa, Observer / Event pattern – event-driven komunikacija između servisa, CQRS – razdvajanje upisa i čitanja podataka za skalabilnost
* AI modul (Python) za predikciju potražnje i optimizaciju proizvodnje,
* asinhrona komunikacija između servisa (message broker),
* mobilna aplikacija za skeniranje QR koda,
* generisanje PDF sertifikata o poreklu


