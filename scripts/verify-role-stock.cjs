// Local integration check. Uses a unique test business and removes only its fixtures.
// Run after `docker compose up -d`: node scripts/verify-role-stock.cjs
const fs = require('node:fs');
const crypto = require('node:crypto');
const { execFileSync } = require('node:child_process');
const assert = require('node:assert/strict');
const settings = Object.fromEntries(fs.readFileSync('.env', 'utf8').split(/\r?\n/).filter(s => /^[A-Z_]+=/.test(s)).map(s => { const i=s.indexOf('='); return [s.slice(0,i),s.slice(i+1).replace(/^['"]|['"]$/g,'')]; }));
const business = crypto.randomUUID();
function token(role) {
  const now = Math.floor(Date.now()/1000);
  const enc = value => Buffer.from(JSON.stringify(value)).toString('base64url');
  const data = enc({ alg:'HS256',typ:'JWT' })+'.'+enc({ sub:crypto.randomUUID(),email:'integration@example.invalid',role,business_id:business,iat:now,exp:now+600 });
  return data+'.'+crypto.createHmac('sha256',settings.JWT_SECRET).update(data).digest('base64url');
}
const owner=token('BUSINESS_OWNER'), worker=token('WORKER'), customer=token('CUSTOMER');
async function request(url, method='GET', body, auth=owner) {
  const response = await fetch('http://localhost/api'+url, { method, headers: { Authorization:'Bearer '+auth, 'Content-Type':'application/json' }, body:body===undefined?undefined:JSON.stringify(body), signal:AbortSignal.timeout(30000) });
  const text=await response.text(); let json; try {json=JSON.parse(text);}catch{}
  return {status:response.status,json,text};
}
function success(result,status=200) {assert.equal(result.status,status, result.text);return result.json.data;}
async function stock(id) {return Number(success(await request('/raw-materials/'+id)).quantity);}
function cleanup() {
  // business is generated above, never accepted from external input.
  const user=settings.POSTGRES_USER || 'postgres';
  for(const [container,db,sql] of [
    ['productions-db','productions_db',`DELETE FROM production_batches WHERE business_id='${business}';`],
    ['raw-materials-db','raw_materials_db',`DELETE FROM production_consumption WHERE business_id='${business}'; DELETE FROM raw_materials WHERE business_id='${business}';`],
  ]) execFileSync('docker',['exec',container,'psql','-U',user,'-d',db,'-v','ON_ERROR_STOP=1','-c',sql],{stdio:['ignore','pipe','pipe']});
}
(async()=>{
 try {
  const a=success(await request('/raw-materials','POST',{name:'Integration milk',material_type:'dairy',quantity:10,unit:'kg'}),201);
  const b=success(await request('/raw-materials','POST',{name:'Integration salt',material_type:'spice',quantity:4,unit:'kg'}),201);
  const batch=success(await request('/productions/batches','POST',{name:'Integration batch',process_type:'fermentation',raw_materials:[{raw_material_id:a.id,quantity_used:3,unit:'kg'}]}),201);
  assert.equal(await stock(a.id),7);
  success(await request(`/productions/batches/${batch.id}/materials`,'POST',{raw_material_id:b.id,quantity_used:2,unit:'kg'},worker),201);
  assert.equal(await stock(b.id),2);
  assert.ok((await request(`/productions/batches/${batch.id}/materials`,'POST',{raw_material_id:b.id,quantity_used:2,unit:'kg'},worker)).status>=400);
  assert.equal(await stock(b.id),2);
  const before=success(await request('/productions/batches')).total;
  const rejected=await request('/productions/batches','POST',{name:'Must roll back',process_type:'fermentation',raw_materials:[{raw_material_id:a.id,quantity_used:1,unit:'kg'},{raw_material_id:b.id,quantity_used:99,unit:'kg'}]});
  assert.equal(rejected.status,400,rejected.text);
  assert.equal(await stock(a.id),7);assert.equal(await stock(b.id),2);
  assert.equal(success(await request('/productions/batches')).total,before);
  assert.equal((await request('/products','POST',{name:'Forbidden',product_type:'dairy',price:1,quantity:1,unit:'kg'},customer)).status,403);
  assert.equal((await request('/orders','POST',{items:[]},owner)).status,403);
  assert.equal((await request('/orders/analytics','GET',undefined,worker)).status,403);
  console.log('PASS: creation deducts stock, adding materials deducts stock, duplicates and insufficient stock are safe, role restrictions enforced.');
 } finally { cleanup(); console.log('Removed integration fixtures.'); }
})().catch(error=>{ console.error(error.message); process.exitCode=1; });
