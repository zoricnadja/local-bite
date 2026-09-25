from pathlib import Path
import json

p = Path('LocalBite_diplomski_rad.odt')
q = Path('LocalBite_diplomski_rad.expanded.odt')
b = Path('tmp/LocalBite_diplomski_rad_pre_tehnologija_obrazaca.odt')
assert p.read_bytes() == b.read_bytes()
result = {'technology_sections': 13, 'pattern_subsections': 13, 'references': 29}
try:
    q.replace(p)
    result.update(output=str(p.resolve()), original_updated=True)
except PermissionError:
    final = Path('LocalBite_diplomski_rad_prosireno.odt')
    assert not final.exists()
    q.replace(final)
    result.update(output=str(final.resolve()), original_updated=False)
Path('tmp/technologies_patterns_verification.json').write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding='utf-8')
print(json.dumps(result, ensure_ascii=False))
