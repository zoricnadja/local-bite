# Proizvodna serija i koraci

Production batch predstavlja celu proizvodnu seriju, od sirovina do gotovih
proizvoda. Serija nema tip. Process steps predstavljaju faze (soljenje →
dimljenje → sušenje), sa statusima PLANNED → IN_PROGRESS → COMPLETED.

Novi korak je PLANNED. Pokretanje bilo kog koraka automatski postavlja seriju
na IN_PROGRESS i, ako nije unet, beleži datum početka. Serija ostaje IN_PROGRESS
i kada su svi koraci završeni. Korisnik označava seriju kao COMPLETED tek kada
postoji bar jedan korak i sve faze su
završene i potvrđuje konačne proizvode i količine. Postojeći mehanizam tada
prebacuje proizvode u Storage. Samo dodavanje koraka „curing” ne završava seriju
i ne pravi gotov proizvod.

Svaki korak ima `variables: [{ name, value }]`. Vrednost je tekst i može sadržati
jedinicu, npr. „18 °C”, „48 h”, „75 %” ili naziv kulture. Migracija prenosi
postojeće duration_hours i temperature u ovu listu, uključujući vrednost 0.
Prazna lista je dozvoljena; nazivi moraju biti jedinstveni unutar koraka.
Završene i otkazane serije ne dozvoljavaju izmene koraka.

Tipovi sirovina ostaju u tabeli `type_catalog`, a „New type” dodaje tip za
trenutno gazdinstvo. Tip proizvodnje je uklonjen iz API-ja, obrazaca, filtera
i porekla proizvoda. Istorijska kolona `process_type` i katalog proizvodnje
ostaju samo radi očuvanja starih podataka i više se ne koriste u aplikaciji.

GET/POST `/raw_materials/types` zahtevaju FARM_OWNER ili
WORKER i važeće gazdinstvo. POST prima `{ "name": "..." }`; ponovljeno dodavanje
istog naziva, bez obzira na velika/mala slova, vraća već postojeći tip.
Forme i filteri učitavaju ove liste preko API-ja.

Migracije se izvršavaju pri pokretanju ažuriranih servisa. Frontend, servis
proizvodnje i servis proizvoda treba ažurirati zajedno zbog novog formata
parametara koraka u evidenciji porekla proizvoda.

Koraci istorijskih završenih serija dobijaju COMPLETED; ostali postojeći
koraci dobijaju PLANNED, jer se njihova stvarna faza ne može pouzdano zaključiti
iz statusa serije. API i PostgreSQL triggeri sprovode pravila. Promene koraka
zaključavaju roditeljsku seriju da bi se serijalizovale sa njenim završavanjem.

`scripts/test-step-status.ps1` proverava migraciju i pravila u izolovanoj šemi
sa rollback-om, bez menjanja podataka aplikacije.
