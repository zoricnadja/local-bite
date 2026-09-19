# 08 — Frontend i korisnički interfejs

[Sadržaj specifikacije](README.md)

## 1. Struktura

Frontend se nalazi u `frontend/local-bite-frontend`. `src/main.ts` pokreće Angular, `app.config.ts` registruje router, HttpClient/interceptor i animacije. `app.ts/html/css` predstavljaju korensku školjku sa navigacijom i router outlet-om.

| Sloj | Sadržaj |
|---|---|
| `core/auth` | Sesija, guards, permission matrica, appCan direktiva |
| `core/interceptors` | Dodavanje JWT zaglavlja |
| `core/services` | API adapteri za korisnike, farmu, sirovine, proizvodnju, proizvode, porudžbine i proizvođače |
| `features` | Ekrani grupisani po poslovnoj oblasti |
| `shared/models` | TypeScript ugovori i statusne konstante |
| `shared/form-validators.ts` | Zajednička validacija |
| `shared/field-errors.directive.ts` | Prikaz grešaka formulara |
| `infrastructure/material` | Zajednički Material importi |
| `styles.css` | Globalni stilovi, vizuelne promenljive i klase |

Nema zasebnog frontend mikroservisa po domenu: radi se o jednoj SPA sa lazy feature granicama.

## 2. Ekrani i rute

| Ruta | Komponenta / svrha | Glavne akcije |
|---|---|---|
| `/auth/login` | LoginComponent | Prijava, token, učitavanje profila |
| `/auth/register` | RegisterComponent | Kreiranje naloga uz profilna polja |
| `/profile` | ProfileComponent | Profil, podaci farme, uređivanje, potvrda brisanja |
| `/dashboard` | DashboardComponent | Sažetak prema ulozi, niska zaliha, aktivne/planirane serije ili kupčeve porudžbine |
| `/farm/create` | CreateFarmComponent | Kreiranje gazdinstva i usklađivanje tokena |
| `/farm/workers` | WorkersListComponent | Pregled zaposlenih |
| `/farm/workers/add` | AddWorkerComponent | Registracija radnika za farmu |
| `/raw-materials` | RawMaterialsListComponent | Lista, pretraga, filtriranje, paginacija, korekcija zalihe i brisanje |
| `/raw-materials/new` | RawMaterialFormComponent | Unos sirovine |
| `/raw-materials/:id/edit` | Ista forma | Izmena postojeće sirovine |
| `/production` | ProductionListComponent | Lista i filteri serija |
| `/production/new` | ProductionFormComponent | Planiranje serije sa sirovinama |
| `/production/:id/edit` | Ista forma | Izmena serije |
| `/production/:id` | ProductionDetailComponent | Status, koraci, sirovine i forma konačnog izlaza |
| `/products` | ProductsListComponent | Ponuda, tip, pretraga, filter proizvođača za kupca |
| `/products/new` | Redirect | Preusmerava na `/production/new` |
| `/products/:id/edit` | ProductFormComponent | Cena, opis, aktivnost, rok i ostali dozvoljeni podaci |
| `/products/:id` | ProductDetailComponent | Podaci proizvoda, slika, QR i poreklo |
| `/orders` | OrdersListComponent | Farm lista ili kupčeve porudžbine, filteri |
| `/orders/new` | OrderFormComponent | Izbor proizvoda, korpa i kreiranje porudžbine |
| `/orders/:id` | OrderDetailComponent | Stavke, ukupni iznos i dozvoljene promene statusa |
| `/orders/analytics` | OrdersAnalyticsComponent | Prihod, statusi, top proizvodi i mesečna tabela |
| `/trace/:qrToken` | PublicTraceComponent | Javni prikaz porekla i PDF bez prijave |

Prazna/nepoznata ruta preusmerava na dashboard, koji zatim zahteva autentifikaciju. Javni QR je odvojen od tog guard-a.

## 3. Sesija

AuthService čuva `lb_token` i `lb_user` u localStorage i inicijalizuje signale iz tog skladišta. `currentUser` i `token` su read-only signali za potrošače, dok `role`, `farmId` i `isLoggedIn` predstavljaju izvedene vrednosti.

`isLoggedIn` proverava prisustvo tokena, ne njegov kriptografski potpis ili istek. `authGuard` na navigaciji poziva `refreshUser`, pa backend proverava token i vraća aktuelni profil. Novi token iz `/me` ili kreiranja farme se čuva pre naredne operacije. Logout briše lokalne vrednosti i navigira na login, bez serverske revocation liste.

Interceptor dodaje Bearer token svakom zahtevu kroz ovaj HttpClient ako token postoji. Trenutni API adapteri koriste relativne putanje. Ako se ubuduće dodaju pozivi na druge domene, interceptor treba ograničiti na dozvoljeni API origin da ne prenosi token proizvoljnom odredištu.

## 4. Dozvole UI-ja

| Permission | Uloge iz frontend modela |
|---|---|
| shop | Customer |
| manageProducts, manageProduction, manageMaterials, manageOrders | FarmOwner, Worker |
| deleteFarmData, manageFarm | FarmOwner |
| analytics | FarmOwner, SystemAdmin |
| viewFarm | FarmOwner, Worker |
| viewMaterials | FarmOwner, Worker, SystemAdmin |

`permissionGuard` sprečava navigaciju, a `CanDirective` uz effect dodaje ili uklanja deo template-a. Korenska navigacija ima dodatnu sopstvenu listu uloga, pa postoje dva mesta koja treba održavati usklađeno. Backend pravila nisu automatski izvedena iz ove matrice.

## 5. Stanje, učitavanje i povratne informacije

Komponente uglavnom koriste signal-e za podatke, loading, grešku i status slanja. FormsModule služi filterima, a ReactiveFormsModule unosu. API servisi grade HttpParams iz definisanih vrednosti i preskaču prazne parametre. Nema globalnog query cache-a; komponente učitavaju sopstvene podatke.

Dashboard osvežava na pet sekundi, sa `requesting` zaštitom od preklapanja i `takeUntilDestroyed` prekidom intervala. Storage lista takođe koristi petosekundni interval. Pretraga proizvoda odlaže poziv 300 ms ručnim timer-om. Javna trace komponenta ponavlja samo 409 odgovore i ima poseban opis 404.

Pojedini ekrani pri grešci samo isključe loading, bez jasne poruke. Nisu svi subscribe tokovi ujednačeni niti svuda postoji zaštita od zastarelog odgovora. To su ograničenja UX-a i održavanja, ne nepostojanje ekrana.

## 6. Forme i poslovna pravila

`requiredText` odbija praznine bez sadržaja, `websiteUrl` dozvoljava HTTP/HTTPS URL, `dateOrder` proverava redosled ISO datuma. `FieldErrorsDirective` uz touched/dirty prikazuje poruku i dodaje `aria-describedby`, `aria-live` i `aria-invalid`, uz čišćenje elementa pri uništenju.

Forma serije bira postojeće sirovine i njihove jedinice. Forma dodavanja sirovine ograničava količinu trenutno poznatim stanjem; backend ponavlja proveru jer stanje može da se promeni. Završetak serije ima odvojenu formu za naziv, tip, prinos, jedinicu, datum kraja i rok trajanja.

Product forma pri izmeni onemogućava polja količine i jedinice, kao i batch kada je već vezan. Učitava serije po stranicama za izbor. Neaktivni proizvod se posle izmene vraća na Storage, aktivni na Products. Frontend blokada polja nije zamena za serversku proveru measured proizvoda.

Korpa je signal unutar OrderFormComponent, nije trajno sačuvana korpa korisnika na serveru. Količina se menja dugmadima, a ukupna vrednost je informativni klijentski proračun. Orders backend ponovo čita cene i pravi konačne iznose. Picker trenutno učitava do 100 proizvoda po izabranom filteru, bez punog višestraničnog izbora celog kataloga.

## 7. Javna stranica i analitika

Javni trace prikazuje naziv, tip, opis, proizvođača, rok, seriju i poreklo sirovina. PublicProvenanceResponse izostavlja interne ID-eve, dobavljača, cene i zalihe. PDF se gradi iz istog javnog ugovora.

Orders analitika koristi CSS trake za raspodelu statusa, listu top proizvoda i mesečnu tabelu. Ne postoji Chart.js import ili dependency u pregledanom manifestu. UI top listu skraćuje na osam, iako backend računa do deset stavki. Valuta je vizuelno označena kao evro, bez viševalutnog domena u bazi.

## 8. Tipizacija i pristupačnost

Product i Production liste koriste ApiResponse<PaginatedResponse<T>>, low-stock ApiResponse<T[]>, a listWorkers niz WorkerOut. Uklonjeni su as unknown as workaround-i. decimal-wire.ts pretvara NUMERIC stringove u prikazne brojeve; server ostaje izvor finansijskog obračuna. OrdersService čuva ključ nerešene kupovine u sessionStorage, vezan za kupca i telo zahteva. Posle neizvesnog odgovora isti zahtev zadržava ključ i nakon osvežavanja stranice. AuthenticatedMediaDirective preuzima privatne slike/QR preko HttpClient-a i oslobađa Blob URL pri promeni i uništavanju.

Postoje responzivni CSS breakpoint-i, forme sa labelama i direktiva grešaka sa ARIA podrškom. Nije izvršen potpun accessibility audit, test tastaturom i screen-reader-om; ne tvrditi usaglašenost sa standardom samo na osnovu ovih elemenata. UI tekst je pretežno na engleskom, bez uvedenog i18n sistema.
