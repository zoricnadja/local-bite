from pathlib import Path
from zipfile import ZipFile
from copy import deepcopy
from lxml import etree as E
from xml.sax.saxutils import escape, quoteattr
import re, json, hashlib

path=Path('LocalBite_diplomski_rad.odt')
original=path.read_bytes()
with ZipFile(path) as z:
    entries=[(i,z.read(i.filename)) for i in z.infolist()]
source=dict((i.filename,b) for i,b in entries)['content.xml'].decode('utf-8')
root=E.fromstring(source.encode())
ns={'text':'urn:oasis:names:tc:opendocument:xmlns:text:1.0','table':'urn:oasis:names:tc:opendocument:xmlns:table:1.0'}

def boundary(s,title):
    return next(m.start() for m in re.finditer(r'<text:h\b[^>]*>.*?</text:h>',s,re.S) if title in m.group())

def inline(s):
    parts=re.split(r'(https://[^\s]+)',s)
    return ''.join(f'<text:a xlink:type="simple" xlink:href={quoteattr(t)}>{escape(t)}</text:a>' if t.startswith('https://') else escape(t) for t in parts)

def paragraph(s,style='Standard'):
    return f'<text:p text:style-name="{style}">{inline(s)}</text:p>'

tech=[
('Tehnologija','Primena','Značaj za rešenje'),
('Rust i Cargo','Serverski servisi i zajednička biblioteka','Tipizovani modeli i objedinjeno upravljanje zavisnostima'),
('Axum i Tokio','HTTP obrada i asinhrono izvršavanje','Tipizovani zahtevi i konkurentne ulazno-izlazne operacije'),
('PostgreSQL 15','Zasebne servisne baze','Transakcije, integritet i kontrola konkurentnih izmena'),
('SQLx','SQL pristup i migracije','Eksplicitni upiti i kontrola granica transakcije'),
('RabbitMQ i Lapin','Razmena integracionih događaja','Nezavisna obrada promena uz potvrde i ponavljanje'),
('Reqwest i Serde','HTTP pozivi i prenos podataka','Komunikacija kroz strukturisane JSON ugovore'),
('JWT i Argon2','Identitet i zaštita lozinki','Provera pristupa i čuvanje heširanih lozinki'),
('Angular i TypeScript','Komponente, rute i obrasci','Organizacija i tipizacija korisničkog interfejsa'),
('RxJS','Tokovi odgovora i događaja','Transformacija rezultata i kontrolisano osvežavanje'),
('image i qrcode','Fotografije i QR kodovi','Obrada medija i pristup poreklu proizvoda'),
('Python i ReportLab','Dokument o poreklu','Programsko sastavljanje PDF prikaza'),
('Docker Compose i Nginx','Pokretanje i prosleđivanje zahteva','Ponovljivo okruženje i zajednička ulazna tačka'),
('Vitest i pomoćne skripte','Provera komponenti i poslovnih tokova','Izdvojene i integracione provere'),
]
oldtable=root.find('.//table:table[@table:name="Table5"]',ns)
newtable=deepcopy(oldtable)
rows=oldtable.findall('.//table:table-row',ns)
for c in list(newtable):
    if E.QName(c).localname in ('table-row','table-header-rows','table-rows'): newtable.remove(c)
for i,values in enumerate(tech):
    row=deepcopy(rows[0 if i==0 else 1])
    for cell,value in zip(row.findall('table:table-cell',ns),values):
        p=deepcopy(cell.find('text:p',ns))
        span=cell.find('text:p/text:span',ns)
        for child in list(p): p.remove(child)
        p.text=None
        if span is not None:
            E.SubElement(p,'{'+ns['text']+'}span',attrib=dict(span.attrib)).text=value
        else: p.text=value
        for child in list(cell): cell.remove(child)
        cell.append(p)
    if i==0: E.SubElement(newtable,'{'+ns['table']+'}table-header-rows').append(row)
    else: newtable.append(row)

def convert(file):
    result=[]
    for block in Path(file).read_text(encoding='utf-8').strip().split('\n\n'):
        if block=='[[TECHNOLOGIES]]':
            result.append(E.tostring(newtable,encoding='unicode',with_tail=False))
        elif block.startswith('#'):
            prefix,text=block.split(' ',1)
            level=len(prefix)
            style='P3' if level==1 else f'Heading_20_{level}'
            result.append(f'<text:h text:style-name="{style}" text:outline-level="{level}">{escape(text)}</text:h>')
        else: result.append(paragraph(block,'Caption' if block.startswith('Tabela ') else 'Standard'))
    return ''.join(result)+'<text:p text:style-name="Standard"/>'

chapter4=convert('tmp/chapter4_technologies.md')
patterns=convert('tmp/chapter5_patterns.md')
a=boundary(source,'4 Teorijske osnove i izbor tehnologija')
b=boundary(source,'5 Arhitektura sistema')
c=boundary(source,'6 Model podataka i servisne baze')
new=source[:a]+chapter4+source[b:c]+patterns+source[c:]
new=new.replace('5 Arhitektura sistema','5 Arhitektura sistema i primenjeni softverski obrasci')
old_rule='Uklanjanje veze sirovine iz serije ne predstavlja automatski povraćaj ranije utrošene količine; ovo ograničenje treba razlikovati od kompenzacije neuspešno izvršene distribuirane operacije.'
new_rule='Dozvoljeno uklanjanje veze sirovine iz serije upisuje trajni zadatak za povraćaj ranije utrošene količine. Takvi zadaci nastaju i pri otkazivanju ili logičkom brisanju serije, a pozadinski proces ih izvršava preko servisa sirovina. Povraćaj je zato asinhron i treba ga razlikovati od trenutne promene lokalne evidencije.'
assert new.count(old_rule)==1
new=new.replace(old_rule,new_rule)
new=new.replace('Teorijske osnove, tehnologije i arhitektonske odluke','Teorijske osnove, tehnologije, arhitektonske odluke i softverski obrasci')

refs={
5:('J. Lewis i M. Fowler, „Microservices“, 2014.','https://martinfowler.com/articles/microservices.html'),
8:('PostgreSQL Global Development Group, „PostgreSQL 15 Documentation — What Is PostgreSQL?“.','https://www.postgresql.org/docs/15/intro-whatis.html'),
9:('RabbitMQ, „Consumer Acknowledgements and Publisher Confirms“, dokumentacija.','https://www.rabbitmq.com/docs/confirms'),
10:('Rust Project Developers, „The Rust Programming Language — Understanding Ownership“.','https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html'),
11:('Angular Team, „What is Angular?“, zvanična dokumentacija.','https://angular.dev/overview'),
13:('Axum contributors, „axum 0.8.8“, dokumentacija biblioteke.','https://docs.rs/axum/0.8.8/axum/'),
14:('Tokio contributors, „Tutorial“, zvanična dokumentacija.','https://tokio.rs/tokio/tutorial'),
15:('SQLx contributors, „sqlx 0.8.6“, dokumentacija biblioteke.','https://docs.rs/sqlx/0.8.6/sqlx/'),
16:('Serde contributors, „Overview“, zvanična dokumentacija.','https://serde.rs/'),
17:('A. Biryukov, D. Dinu, D. Khovratovich i S. Josefsson, „Argon2 Memory-Hard Function for Password Hashing and Proof-of-Work Applications“, RFC 9106, 2021.','https://www.rfc-editor.org/rfc/rfc9106.html'),
18:('Microsoft, „The TypeScript Handbook“, zvanična dokumentacija.','https://www.typescriptlang.org/docs/handbook/intro.html'),
19:('ReactiveX, „RxJS Introduction“, zvanična dokumentacija.','https://rxjs.dev/guide/overview'),
20:('ReportLab, „Chapter 5: Platypus“, ReportLab User Guide.','https://docs.reportlab.com/reportlab/userguide/ch5_platypus/'),
21:('Docker Inc., „Docker Compose“, zvanična dokumentacija.','https://docs.docker.com/compose/'),
22:('NGINX, „Module ngx_http_proxy_module“, zvanična dokumentacija.','https://nginx.org/en/docs/http/ngx_http_proxy_module.html'),
23:('E. Gamma, R. Helm, R. Johnson i J. Vlissides, Design Patterns: Elements of Reusable Object-Oriented Software, Addison-Wesley, 1994, ISBN 978-0-201-63361-0.','https://www.informit.com/store/design-patterns-elements-of-reusable-object-oriented-9780201633610'),
24:('E. Hieatt i R. Mee, „Repository“, katalog Patterns of Enterprise Application Architecture, 2003.','https://martinfowler.com/eaaCatalog/repository.html'),
25:('R. Stafford, „Service Layer“, katalog Patterns of Enterprise Application Architecture, 2003.','https://martinfowler.com/eaaCatalog/serviceLayer.html'),
26:('M. Fowler, „Inversion of Control Containers and the Dependency Injection pattern“, 2004.','https://martinfowler.com/articles/injection.html'),
27:('C. Richardson, „Pattern: Transactional outbox“, Microservices.io.','https://microservices.io/patterns/data/transactional-outbox.html'),
28:('M. Fowler, „CQRS“, 2011.','https://martinfowler.com/bliki/CQRS.html'),
29:('C. Richardson, „Pattern: Saga“, Microservices.io.','https://microservices.io/patterns/data/saga.html'),
}
def reference(i):
    label,url=refs[i]
    return paragraph(f'[{i}] {label} Dostupno na: {url} (pristupljeno 24. 9. 2026).')

for i in [5,8,9,10,11]:
    count=0
    def replace_reference(m):
        global count
        text=''.join(E.fromstring(('<root '+ ' '.join(f'xmlns:{k}="{v}"' for k,v in root.nsmap.items() if k)+'>'+m.group()+'</root>').encode()).itertext())
        if text.startswith(f'[{i}] '):
            count+=1
            return reference(i)
        return m.group()
    new=re.sub(r'<text:p\b(?:[^>]*?/>|[^>]*>.*?</text:p>)',replace_reference,new,flags=re.S)
    assert count==1,(i,count)
appendix=boundary(new,'Prilog A API primeri')
new=new[:appendix]+''.join(reference(i) for i in range(13,30))+'<text:p text:style-name="Standard"/>'+new[appendix:]
parsed=E.fromstring(new.encode())
assert new[boundary(new,'6 Model podataka'):boundary(new,'Literatura')]==source[c:boundary(source,'Literatura')]
assert new[boundary(new,'Prilog A API primeri'):]==source[boundary(source,'Prilog A API primeri'):]
assert len(parsed.findall('.//table:table[@table:name="Table5"]',ns))==1
assert len(parsed.findall('.//table:table[@table:name="Table5"]//table:table-row',ns))==len(tech)
assert '[[TECHNOLOGIES]]' not in new
assert len(re.findall(r'text:outline-level="3">5\.6\.',new))==13
assert set(map(int,re.findall(r'\[(\d+)\]',chapter4+patterns))) <= set(range(1,30))
for i in range(13,30): assert f'[{i}]' in new
backup=Path('tmp/LocalBite_diplomski_rad_pre_tehnologija_obrazaca.odt')
assert not backup.exists()
assert path.read_bytes()==original,'Document modified during editing'
backup.write_bytes(original)
temp=path.with_suffix('.expanded.odt')
with ZipFile(temp,'w') as z:
    for info,data in entries: z.writestr(info,new.encode() if info.filename=='content.xml' else data)
with ZipFile(temp) as z:
    assert z.testzip() is None
    for info,data in entries:
        if info.filename!='content.xml': assert z.read(info.filename)==data
temp.replace(path)
report={'technology_sections':13,'pattern_subsections':13,'new_references':17,'chapters_6_to_14_unchanged':True,'appendices_unchanged':True,'chapter3_material_return_corrected':True,'backup_sha256':hashlib.sha256(original).hexdigest()}
Path('tmp/technologies_patterns_verification.json').write_text(json.dumps(report,indent=2),encoding='utf-8')
print(report)
