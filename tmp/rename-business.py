from pathlib import Path
import json,hashlib,re
root=Path.cwd()
def rename(s):
 for a,b in [('FARMS','BUSINESSES'),('Farms','Businesses'),('farms','businesses'),('FARM','BUSINESS'),('Farm','Business'),('farm','business')]:s=s.replace(a,b)
 return s
paths=[]
for folder in ['services','libs','scripts','docs','nginx','frontend/local-bite-frontend/src']:
 paths.extend(p for p in (root/folder).rglob('*') if p.is_file() and 'migrations' not in p.parts and p.suffix in ['.rs','.sql','.ts','.html','.css','.json','.md','.cjs','.js','.py','.ps1','.toml','.conf','.yml'])
paths += [root/'README.md']
changed=0
for p in paths:
 s=p.read_bytes().decode('utf-8');n=rename(s)
 if n!=s:p.write_bytes(n.encode('utf-8'));changed+=1
# SQLx offline metadata is keyed by SHA-256 of the exact query string.
for p in (root/'.sqlx').glob('query-*.json'):
 data=json.loads(p.read_text(encoding='utf-8'));n=json.loads(rename(json.dumps(data,ensure_ascii=False)))
 n['hash']=hashlib.sha256(n['query'].encode()).hexdigest()
 dest=p.with_name('query-'+n['hash']+'.json')
 if n!=data:
  dest.write_text(json.dumps(n,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
  if dest!=p:p.unlink()
# Rename deepest files/directories first; all targets remain within this workspace.
for folder in ['services','scripts','frontend/local-bite-frontend/src']:
 for p in sorted((root/folder).rglob('*'),key=lambda p:len(p.parts),reverse=True):
  if 'migrations' in p.parts:continue
  name=rename(p.name)
  if name!=p.name:
   dest=p.with_name(name)
   assert dest.resolve().is_relative_to(root)
   p.rename(dest)
print('Renamed content in',changed,'files and refreshed SQLx metadata.')
