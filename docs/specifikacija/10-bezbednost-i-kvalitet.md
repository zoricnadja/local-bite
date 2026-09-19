# 10 — Bezbednost, kvalitet i poznata ograničenja

[Sadržaj specifikacije](README.md)

## Implementirane zaštite

- Javna registracija dozvoljava samo `CUSTOMER` i `FARM_OWNER`; radnika kreira vlasnik, a administrator se ne registruje javno.
- Lozinke se čuvaju kao Argon2 hash. Korisnički JWT važi jedan sat. Tehnički `TRACEABILITY`, `ORDER_STOCK` i `MATERIAL_STOCK` tokeni važe 60 sekundi i vezani su za operaciju/resurs i farmu.
- Detalj porudžbine proverava kupca ili farmu; detalj proizvoda, poreklo, slika i QR proveravaju ulogu, vlasništvo i efektivnu aktivnost.
- Javni decrement je uklonjen. Zaliha se menja samo internom atomarnom rezervacijom sa idempotency ključem.
- Završene, otkazane i obrisane serije SQL trigger-i štite od izmene koraka i utrošenih sirovina.
- `PublicProvenance` je allowlist DTO bez internih ID-eva, cene, stanja, dobavljača, putanja i internih datuma.
- Outbox ne objavljuje lozinke ni kontakt porudžbine. Greške baze i PDF procesa loguju kategoriju, bez payload-a, tokena ili stderr sadržaja.
- Direktni `/uploads` je uklonjen. Frontend medije preuzima autorizovanim `HttpClient` zahtevom.

## Pouzdanost zalihe i distribuiranih tokova

`checkout_jobs` beleži nameru pre rezervacije. Products u jednoj transakciji zaključava operaciju, proverava sve proizvode, cenu, farmu, aktivnost, rok i količinu, pa umanjuje stanje. Orders zatim lokalno upisuje porudžbine, stavke, veze rezervacije i sačuvani odgovor. Isti `Idempotency-Key`, kupac i telo vraćaju isti rezultat; drugi sadržaj daje 409.

Otkazivanje i brisanje dozvoljene porudžbine transakciono upisuju `stock_release_jobs`. Recovery ponavlja vraćanje, a Products `released` stanje čini operaciju idempotentnom. Izmena cene ne prepisuje količinu automatski nastalog proizvoda.

Productions koristi `material_intents` pre udaljene potrošnje i `material_release_jobs` za uklanjanje/otkazivanje. Raw Materials ledger i tombstone sprečavaju duplo i kasno trošenje već kompenzovane operacije. Ovo je eventualno oporavljiv orkestrisani tok, ne distribuirana ACID transakcija.

## Događaji i nepromenljivost

Output consumer proverava poslednju sekvencu pod advisory lock-om. Stariji `COMPLETED` ne može da oživi zalihu posle novijeg delete/cancel događaja. Legacy završetak bez izmerene količine ne izmišlja stock. Nevažeća poruka objavljuje se trajno u `local_bite.production_outputs.dead.v1`, potvrđuje se publish, pa se original ACK-uje. Greška baze requeue-uje poruku.

Outbox, publisher confirms, projection receipts i entity sequence štite od gubitka, duplikata i promene redosleda u pokrivenim scenarijima. RabbitMQ u Compose-u je jedna instanca; nema HA quorum klastera.

## Granice poverenja koje ostaju

- JWT je stateless. Promena uloge ili članstva ne opoziva već izdat token pre isteka; nema refresh/revocation tabele.
- Token je u `localStorage`, pa XSS ostaje ključna browser pretnja. CSP i formalni penetration test nisu urađeni.
- Svi servisi dele simetrični `JWT_SECRET`; zasebni issuer/ključevi bi smanjili blast radius.
- CORS je permissive na domenskim servisima. Osnovni Compose mapira razvojne DB i servisne portove na host.
- Brisanje farme je transakciono unutar auth baze i odvaja članove, ali ne orkestrira brisanje podataka u drugim servisima.
- Nema plaćanja, fiskalizacije ni kurirske integracije.

## Preostali tehnički dug

| ID | Ograničenje | Sledeći korak |
|---|---|---|
| O-01 | Zamena slike briše staru datoteku pre završetka novog SQL toka | Write-new → DB swap → delete-old |
| O-02 | MIME allowlist navodi WebP, a build eksplicitno pokriva PNG/JPEG | Uključiti/testirati WebP ili ga ukloniti iz ugovora |
| O-03 | `null` u update DTO-u često znači „bez promene“ | Za brisanje polja uvesti tri-state DTO |
| O-04 | Orders liste učitavaju stavke po porudžbini | Uvesti batch query/JSON agregaciju |
| O-05 | Nema centralnih metrika, correlation ID-a i distributed tracing-a | Dodati metrike, alarme i korelaciju |
| O-06 | Upload volume nije horizontalno deljen storage | Koristiti objektni storage za više instanci |
| O-07 | Retention se ne izvršava automatski | Odobriti rokove, backup checkpoint i dry-run |

## Nefunkcionalni status

| Oblast | Potvrđeno | Granica dokaza |
|---|---|---|
| Build | Rust workspace release i Angular production build prolaze | Nema višearhitekturnog CI matriksa |
| Konzistentnost | Lokalne transakcije, rezervacije i trajne kompenzacije | Nema globalne ACID transakcije |
| Performanse | Lokalni baseline u `docs/verification/performance.json` | Nije produkcioni load/capacity test |
| Oporavak | Svih šest dump-ova vraćeno u izolovan PostgreSQL 15 | Cross-service point-in-time RPO traži maintenance checkpoint |
| Bezbednost | Role/objektna autorizacija, public DTO, privatni mediji | Nema formalnog pentesta |
| Pristupačnost | Responzivni CSS, labele i ARIA greške | Nema WCAG/screen-reader audita |

Merljive produkcione ciljeve treba postaviti prema realnoj infrastrukturi. Lokalni rezultat nije SLA.
