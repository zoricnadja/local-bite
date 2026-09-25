from pathlib import Path
from zipfile import ZipFile
from copy import deepcopy
from lxml import etree as E
from xml.sax.saxutils import escape
import re, hashlib

path = Path('LocalBite_diplomski_rad.odt')
original_bytes = path.read_bytes()
with ZipFile(path) as z:
    entries = [(i, z.read(i.filename)) for i in z.infolist()]
source = dict((i.filename, data) for i, data in entries)['content.xml'].decode('utf-8')
ns = {'text': 'urn:oasis:names:tc:opendocument:xmlns:text:1.0', 'table': 'urn:oasis:names:tc:opendocument:xmlns:table:1.0'}
root = E.fromstring(source.encode())

roles = [
('Uloga', 'Ključne dozvole', 'Ograničenje'),
('Posetilac', 'Javni prikaz porekla i PDF', 'Bez pristupa internoj evidenciji'),
('CUSTOMER', 'Ponuda, sopstvene porudžbine i profil', 'Bez uvida u interne podatke gazdinstva'),
('FARM_OWNER', 'Gazdinstvo, radnici, proizvodnja, prodaja i analitika', 'Poslovne operacije u okviru sopstvenog gazdinstva'),
('WORKER', 'Sirovine, proizvodnja, proizvodi i obrada porudžbina', 'Bez dodavanja radnika i otkazivanja porudžbina'),
('SYSTEM_ADMIN', 'Dostupni administrativni pregledi', 'Ovlašćenja zavise od operacije; nema javne registracije'),
]
requirements = [
('ID', 'Zahtev', 'Očekivano ponašanje i uslovi'),
('F-01', 'Registracija', 'Kreiranje naloga kupca ili vlasnika; zaštita lozinke i izdavanje tokena.'),
('F-02', 'Prijava i sesija', 'Provera pristupnih podataka i identifikacija korisnika i njegove uloge.'),
('F-03', 'Korisnički profil', 'Pregled i izmena sopstvenih podataka uz ograničenja dodeljenih ovlašćenja.'),
('F-04', 'Gazdinstvo', 'Kreiranje jednog gazdinstva za vlasnika i upravljanje njegovim podacima.'),
('F-05', 'Radnici', 'Vlasnik kreira i pregleda naloge radnika svog gazdinstva.'),
('F-06', 'Evidencija sirovina', 'Unos, pregled, pretraga, filtriranje, izmena i logičko brisanje sirovina.'),
('F-07', 'Praćenje zaliha sirovina', 'Korekcija količina i pregled zaliha prema zadatom pragu.'),
('F-08', 'Proizvodne serije', 'Kreiranje i pregled serija sa datumima, napomenama, sirovinama i izlazima.'),
('F-09', 'Utrošak sirovina', 'Provera jedinice i zalihe, umanjenje količine i čuvanje istorijskih podataka.'),
('F-10', 'Proizvodni koraci', 'Dodavanje, uređivanje i brisanje koraka sa jedinstvenim rednim brojem, dok serija nije zatvorena.'),
('F-11', 'Završetak proizvodnje', 'Svi koraci su završeni; postoji najmanje jedan korak i najmanje jedan validan izlaz.'),
('F-12', 'Skladište proizvoda', 'Potvrđeni izlazi završene serije prelaze u skladište, bez automatskog aktiviranja prodaje.'),
('F-13', 'Prodajna ponuda', 'Aktiviranje zahteva pozitivnu cenu, završenu seriju i važeći rok, ako je naveden.'),
('F-14', 'Slike proizvoda', 'Dodavanje slike proizvoda, obrada i prikaz u aplikaciji.'),
('F-15', 'QR kod', 'Generisanje i preuzimanje koda za pristup javnoj stranici porekla.'),
('F-16', 'Prikaz porekla', 'Povezivanje proizvoda, gazdinstva, serije, koraka i istorijskih podataka sirovina.'),
('F-17', 'Dokument o poreklu', 'Generisanje PDF dokumenta na osnovu podataka dostupnih javnom prikazu.'),
('F-18', 'Poručivanje', 'Rezervacija zaliha i formiranje odvojenih porudžbina za gazdinstva iz iste korpe.'),
('F-19', 'Obrada porudžbine', 'Kontrolisani prelazi statusa i oslobađanje rezervisane zalihe pri otkazivanju.'),
('F-20', 'Analitika', 'Pregled broja i statusa porudžbina, prihoda i najprodavanijih proizvoda.'),
('F-21', 'Upravljački pregled', 'Pregled podataka prilagođen ulozi korisnika.'),
('F-22', 'Izbor proizvođača', 'Autentifikovani pregled proizvođača radi filtriranja ponude.'),
('F-23', 'Prenos poslovnih promena', 'Pouzdano objavljivanje događaja i zaštita od duplirane obrade.'),
('F-24', 'Obnova projekcija', 'Ponovno slanje sačuvanih događaja pomoću operativne skripte.'),
('F-25', 'Katalog tipova sirovina', 'Zajednički tipovi i dopuna za gazdinstvo, bez dupliranja istog naziva.'),
('F-26', 'Parametri koraka', 'Lista parova naziva i tekstualnih vrednosti; nazivi su jedinstveni unutar koraka.'),
('F-27', 'Više proizvodnih izlaza', 'Planiranje i potvrđivanje više proizvoda i njihovih količina u jednoj seriji.'),
('F-28', 'Statusi koraka', 'Planiranje, pokretanje i završetak koraka; pokretanje koraka aktivira seriju.'),
]

def table(name, data):
    src = root.find(f'.//table:table[@table:name="{name}"]', ns)
    dst = deepcopy(src)
    rows = src.findall('.//table:table-row', ns)
    for child in list(dst):
        if E.QName(child).localname in ('table-row', 'table-header-rows', 'table-rows'):
            dst.remove(child)
    for i, values in enumerate(data):
        row = deepcopy(rows[0 if i == 0 else 1])
        cells = row.findall('table:table-cell', ns)
        assert len(cells) == len(values)
        for cell, value in zip(cells, values):
            para = deepcopy(cell.find('text:p', ns))
            for child in list(para):
                para.remove(child)
            # Preserve the existing table's run style, including header weight.
            oldspan = cell.find('text:p/text:span', ns)
            if oldspan is not None:
                span = E.SubElement(para, '{'+ns['text']+'}span', attrib=dict(oldspan.attrib))
                span.text = value
                para.text = None
            else:
                para.text = value
            for child in list(cell):
                cell.remove(child)
            cell.append(para)
        if i == 0:
            header = E.SubElement(dst, '{'+ns['table']+'}table-header-rows')
            header.append(row)
        else:
            dst.append(row)
    return E.tostring(dst, encoding='unicode', with_tail=False)

tables = {'[[ROLES]]': table('Table3', roles), '[[REQUIREMENTS]]': table('Table4', requirements)}
blocks = []
for block in Path('tmp/chapter3_expanded.md').read_text(encoding='utf-8').strip().split('\n\n'):
    if block in tables:
        blocks.append(tables[block])
    elif block.startswith('#'):
        prefix, text = block.split(' ', 1)
        level = len(prefix)
        style = 'P3' if level == 1 else f'Heading_20_{level}'
        blocks.append(f'<text:h text:style-name="{style}" text:outline-level="{level}">{escape(text)}</text:h>')
    else:
        style = 'Caption' if block.startswith('Tabela ') else 'Standard'
        blocks.append(f'<text:p text:style-name="{style}">{escape(block)}</text:p>')
blocks.append('<text:p text:style-name="Standard"/>')
headings = list(re.finditer(r'<text:h\b[^>]*>.*?</text:h>', source, re.S))
start = next(m.start() for m in headings if '3 Analiza domena i funkcionalni zahtevi' in m.group())
end = next(m.start() for m in headings if '4 Teorijske osnove i izbor tehnologija' in m.group())
new = source[:start] + ''.join(blocks) + source[end:]
parsed = E.fromstring(new.encode('utf-8'))
assert new[:start] == source[:start]
assert new.endswith(source[end:])
assert len(parsed.findall('.//table:table[@table:name="Table3"]', ns)) == 1
assert len(parsed.findall('.//table:table[@table:name="Table4"]', ns)) == 1
assert len(parsed.findall('.//table:table[@table:name="Table4"]//table:table-row', ns)) == 29
for index in range(1, 29):
    assert f'F-{index:02}' in new
assert '[[' not in ''.join(parsed.itertext())

backup = Path('tmp/LocalBite_diplomski_rad_pre_prosirenja_glave3.odt')
if backup.exists():
    raise RuntimeError('Backup already exists; inspect before rerunning.')
assert path.read_bytes() == original_bytes, 'Document changed during editing.'
backup.write_bytes(original_bytes)
temp = path.with_suffix('.chapter3.odt')
with ZipFile(temp, 'w') as z:
    for info, data in entries:
        z.writestr(info, new.encode('utf-8') if info.filename == 'content.xml' else data)
with ZipFile(temp) as z:
    assert z.testzip() is None
    for info, data in entries:
        if info.filename != 'content.xml':
            assert z.read(info.filename) == data
temp.replace(path)
old_text = ''.join(E.fromstring(('<root '+ ' '.join(f'xmlns:{k}="{v}"' for k,v in root.nsmap.items() if k) +'>'+source[start:end]+'</root>').encode()).itertext())
report = {'old_chapter_words':len(old_text.split()), 'new_chapter_words':len(' '.join(E.fromstring(('<root '+ ' '.join(f'xmlns:{k}="{v}"' for k,v in root.nsmap.items() if k) +'>'+''.join(blocks)+'</root>').encode()).itertext()).split()), 'requirements':28, 'use_cases':7, 'prefix_unchanged':True, 'chapter4_onward_unchanged':True, 'other_zip_entries_unchanged':True, 'backup_sha256':hashlib.sha256(original_bytes).hexdigest()}
Path('tmp/chapter3_verification.json').write_text(__import__('json').dumps(report, indent=2), encoding='utf-8')
print(report)
