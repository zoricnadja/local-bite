from pathlib import Path
from zipfile import ZipFile
from lxml import etree
from xml.sax.saxutils import escape
import re, shutil

path = Path('LocalBite_diplomski_rad.odt')
backup = Path('tmp/LocalBite_diplomski_rad_pre_redakture.odt')
replacements = {
1: 'U ovom diplomskom radu predstavljeni su projektovanje, implementacija i verifikacija platforme LocalBite, namenjene praćenju proizvodnje, zaliha, prodaje i porekla domaćih proizvoda. Rad razmatra problem nepovezane evidencije sirovina i proizvodnje na malim poljoprivrednim gazdinstvima, kao i ograničenu dostupnost informacija o poreklu proizvoda u trenutku kupovine. Predloženo rešenje povezuje podatke o korisnicima, gazdinstvima, sirovinama, proizvodnim serijama, gotovim proizvodima i porudžbinama u jedinstven tok, čime omogućava sledljivost od ulaznih sirovina do javnog prikaza porekla dostupnog putem QR koda.',
2: 'Sistem je realizovan kao mikroservisna aplikacija koju čini pet poslovnih servisa i servis za formiranje i čitanje projekcija podataka, pri čemu svaki servis koristi zasebnu PostgreSQL bazu podataka. Komunikacija između servisa zasniva se na REST pozivima i asinhronoj razmeni događaja posredstvom sistema RabbitMQ. Posebna pažnja posvećena je upravljanju transakcijama, idempotentnosti operacija, izolaciji podataka po gazdinstvu, atomskoj rezervaciji zaliha i oporavku nedovršenih distribuiranih poslovnih tokova.',
3: 'Ključne reči: mikroservisi, poljoprivredna gazdinstva, sledljivost, REST, RabbitMQ, PostgreSQL, CQRS, transakcioni outbox, QR kod',
46: 'Mala poljoprivredna gazdinstva mogu voditi evidenciju o sirovinama, proizvodnim postupcima, zalihama i prodaji u sveskama, elektronskim tabelama i drugim međusobno nepovezanim izvorima. Ovakva organizacija podataka otežava utvrđivanje raspoloživih količina, planiranje proizvodnje i naknadno praćenje porekla gotovih proizvoda. Ograničenje takvog pristupa ne proizlazi isključivo iz ručnog unosa podataka, već i iz odsustva veza između zapisa o pojedinačnim fazama proizvodnje i prodaje.',
47: 'Značaj povezivanja ovih podataka može se ilustrovati hipotetičkim primerom proizvodnje ajvara. U prvoj proizvodnoj seriji, 1. septembra 2026. godine, proizvedeno je 30 kg ajvara, dok je 3. septembra proizvedeno dodatnih 50 kg od paprika iz druge isporuke. Ukoliko se naknadno utvrdi neispravnost paprika upotrebljenih u prvoj seriji, potrebno je identifikovati proizvode nastale njihovom preradom. Bez evidencije koja povezuje isporuke sirovina sa proizvodnim serijama, moglo bi biti neophodno povući svih 80 kg proizvoda zbog nemogućnosti pouzdanog razgraničenja serija. Povezivanjem ovih zapisa omogućava se identifikacija potencijalno zahvaćenih 30 kg, dok QR kod olakšava pristup podacima o poreklu odgovarajućeg proizvoda. Mogućnost ograničavanja obuhvata povlačenja zavisi od potpunosti i tačnosti evidentiranih podataka.',
48: 'Sa stanovišta kupca, dostupnost podataka o proizvođaču, proizvodnoj seriji, korišćenim sirovinama i redosledu proizvodnih koraka doprinosi transparentnosti proizvodnje i može podržati poverenje u domaći proizvod. QR kod predstavlja pristupnu tačku tim podacima. Platforma LocalBite stoga povezuje proizvod iz kataloga sa proizvodnom serijom i pripadajućim sirovinama, omogućavajući uvid u evidentirani tok njegovog nastanka.',
50: 'Predmet rada je razvoj mikroservisne platforme za praćenje proizvodnje i porekla domaćih proizvoda. Osnovni cilj rada je projektovanje i implementacija sistema koji podržava rad više poljoprivrednih gazdinstava u okviru jedne aplikacije, uz kontrolisan pristup podacima svakog gazdinstva. Sistem treba da omogući vlasnicima i radnicima vođenje operativne evidencije, kupcima pregled ponude i kreiranje porudžbina, a posetiocima pristup javnom prikazu porekla proizvoda skeniranjem QR koda, bez prethodne registracije.',
51: 'Dodatni cilj je oblikovanje proširivog sistema primenom softverskih obrazaca i mikroservisne arhitekture. Takav pristup treba da olakša uključivanje novih gazdinstava i podršku za različite tipove gazdinstava, sirovina, proizvoda i proizvodnih koraka. Poseban cilj odnosi se na razumljivost korisničkog interfejsa i jasnoću poslovnih postupaka, kako bi platformu mogli da koriste i vlasnici i radnici bez prethodnog iskustva sa sličnim informacionim sistemima.',
52: '1.3 Metod i struktura rada',
53: 'Rad obuhvata analizu problema i zahteva, projektovanje arhitekture i modela podataka, implementaciju servisa i proveru njihovog ponašanja. Opis rešenja zasniva se na projektnoj dokumentaciji, izvornom kodu, definisanim HTTP interfejsima, testovima i skriptama za pokretanje, održavanje i oporavak sistema. Pri razmatranju rezultata razlikuju se realizovane funkcionalnosti, funkcionalnosti sa ograničenom podrškom i predlozi za budući razvoj. Organizacija narednih poglavlja prikazana je u tabeli 1.',
57: 'Poslovni problem, učesnici, zahtevi i slučajevi korišćenja',
61: 'Model podataka, serverska aplikacija, integracija servisa i korisnički interfejs',
65: 'Diskusija rezultata, budući razvoj i zaključak',
70: 'Osnovni poslovni problem predstavlja nepovezanost podataka o ulaznim sirovinama, proizvodnim serijama, gotovim proizvodima i prodajnim transakcijama. Kada se prijem sirovina, njihova potrošnja i prodaja proizvoda evidentiraju u odvojenim sistemima, otežano je usaglašavanje evidencije sa stvarnim stanjem zaliha. Istovremeno, odsustvo veza između ovih zapisa otežava rekonstrukciju proizvodnog toka i utvrđivanje porekla pojedinačnog proizvoda.',
71: 'Dodatni problem odnosi se na upravljanje pristupom podacima u sistemu koji koristi više gazdinstava. Interni poslovni podaci jednog gazdinstva moraju biti zaštićeni od neovlašćenog pristupa korisnika drugih gazdinstava, uz jasno razdvajanje od podataka namenjenih javnom prikazu. Identifikator gazdinstva (farm_id) zato predstavlja jedan od elemenata kontrole pristupa. Pri čitanju ili izmeni zaštićenih poslovnih podataka potrebno je proveriti ovlašćenja korisnika i pripadnost podataka odgovarajućem gazdinstvu, u skladu sa pravilima za konkretnu korisničku ulogu.',
74: 'Potrebe i očekivanja',
75: 'Podrška sistema',
77: 'Upravljanje proizvodnjom, zaposlenima i ponudom',
78: 'Funkcionalnosti za upravljanje gazdinstvom, sirovinama, proizvodnjom i proizvodima',
80: 'Efikasno evidentiranje operativnih promena',
81: 'Unos i izmena podataka o sirovinama i proizvodnim serijama u okviru dodeljenih ovlašćenja',
83: 'Pregled ponude i praćenje porudžbina',
84: 'Katalog proizvoda, korpa i pregled porudžbina i njihovih statusa',
86: 'Pristup informacijama o poreklu bez korisničkog naloga',
87: 'Javni prikaz porekla putem QR koda i dokument o poreklu u PDF formatu',
89: 'Administrativni pregled i podrška korisnicima',
90: 'Pristup administrativnim pregledima u okviru definisanih ovlašćenja',
93: 'Obuhvat platforme LocalBite čine evidencija poslovnih podataka i podrška tokovima proizvodnje i prodaje unutar sistema. Platforma obuhvata registraciju i prijavu korisnika, upravljanje gazdinstvima i radnicima, evidenciju sirovina i proizvodnih serija, upravljanje proizvodima i njihovim slikama, generisanje QR kodova, prikaz porekla, izradu dokumenata o poreklu u PDF formatu, obradu porudžbina i analitičke preglede. Elektronsko plaćanje, fiskalizacija, integracija sa službama dostave, povezivanje sa hardverskim senzorima i nezavisna sertifikacija podataka nisu obuhvaćeni rešenjem.',
94: 'Pri tumačenju javnog prikaza porekla potrebno je razlikovati sledljivost evidentiranih podataka od nezavisne potvrde njihove tačnosti. Dokument o poreklu u PDF formatu zasniva se na podacima koje je proizvođač uneo u sistem. Povezanost zapisa omogućava praćenje evidentiranog proizvodnog toka, ali sama po sebi ne potvrđuje usklađenost unetih podataka sa stvarnim stanjem. Stoga ovaj dokument ne predstavlja nezavisni sertifikat kvaliteta ili ispravnosti proizvoda.',
96: 'Izolacija internih podataka po gazdinstvu i kontrola pristupa prema korisničkoj ulozi i dodeljenim ovlašćenjima;',
97: 'očuvanje istorijskih vrednosti podataka o korišćenim sirovinama i cenama;',
98: 'idempotentnost operacija potrošnje sirovina i rezervacije zaliha, tako da ponavljanje istog zahteva ne izazove višestruku promenu stanja;',
99: 'otpornost projekcija podataka na ponovljene događaje i događaje pristigle van redosleda;',
100: 'mogućnost ponovne obrade događaja radi obnove projekcija nakon oporavka sistema;',
101: 'jasno razdvajanje merodavnog stanja zaliha od javnog modela za čitanje, koji se ažurira asinhrono.',
}

with ZipFile(path) as z:
    entries = [(i, z.read(i.filename)) for i in z.infolist()]
source = dict((i.filename,b) for i,b in entries)['content.xml'].decode('utf-8')
pattern = re.compile(r'<text:(p|h)\b(?:[^>]*?/>|[^>]*>.*?</text:\1>)', re.S)
matches = list(pattern.finditer(source))
assert '3 Analiza domena' in matches[103].group()
assert '01.09.2026.' in matches[47].group()
assert max(replacements) < 103
counter = -1
def replace(m):
    global counter
    counter += 1
    if counter not in replacements:
        return m.group()
    opening = m.group()[:m.group().index('>')+1]
    return opening + escape(replacements[counter]) + '</text:' + m.group(1) + '>'
updated = pattern.sub(replace, source)
etree.fromstring(updated.encode())
newmatches = list(pattern.finditer(updated))
assert len(matches) == len(newmatches)
assert all(a.group() == b.group() for a,b in zip(matches[103:],newmatches[103:]))
assert source[matches[103].start():] == updated[newmatches[103].start():]
if not backup.exists():
    shutil.copy2(path, backup)
out = path.with_suffix('.revised.odt')
with ZipFile(out, 'w') as z:
    for info, data in entries:
        z.writestr(info, updated.encode('utf-8') if info.filename == 'content.xml' else data)
with ZipFile(out) as z:
    assert z.testzip() is None
    for info, data in entries:
        if info.filename != 'content.xml':
            assert z.read(info.filename) == data
out.replace(path)
print(f'Updated {len(replacements)} paragraphs. Chapter 3 onward and all other package entries unchanged. Backup: {backup}')
