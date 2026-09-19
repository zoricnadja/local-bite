// Integration regression using an isolated farm and only its own fixtures.
const assert = require('node:assert/strict');
const { randomUUID } = require('node:crypto');
const { execFileSync } = require('node:child_process');
const run = randomUUID();
let farm, user;
async function api(path, method='GET', body, token) {
  const response = await fetch('http://localhost/api'+path, {method, headers:{'Content-Type':'application/json', ...(token ? {Authorization:'Bearer '+token}: {})},body:body===undefined?undefined:JSON.stringify(body)});
  const json=await response.json();
  return {status:response.status,json};
}
function data(r,status=200){assert.equal(r.status,status,JSON.stringify(r.json));return r.json.data;}
function sql(db,query){return execFileSync('docker',['exec',db+'-db','psql','-U','postgres','-d',db.replaceAll('-','_')+'_db','-v','ON_ERROR_STOP=1','-At','-c',query],{encoding:'utf8',stdio:['ignore','pipe','pipe']}).trim();}
async function until(fn){for(let i=0;i<60;i++){const value=await fn();if(value)return value;await new Promise(r=>setTimeout(r,250));}throw Error('Projection did not settle');}
(async()=>{try{
  const registered=await api('/auth/register','POST',{email:`workflow-${run}@example.invalid`,password:randomUUID(),role:'FARM_OWNER',first_name:'Workflow',last_name:'Test',address:'Test'});
  assert.equal(registered.status,201);
  user=JSON.parse(Buffer.from(registered.json.token.split('.')[1],'base64url')).sub;
  const created=data(await api('/auth/farms','POST',{name:'Workflow regression',address:'Test'},registered.json.token),201);
  farm=created.farm.id;const token=created.token;
  const batch=data(await api('/productions/batches','POST',{name:'Multiple outputs',process_type:'cheese'},token),201);
  const path='/productions/batches/'+batch.id;
  assert.equal((await api(path,'PUT',{outputs:['invalid']},token)).status,400);
  let plan=data(await api(path,'PUT',{outputs:[{name:'Cheese',product_type:'cheese',quantity:10,unit:'kg',price:5},{name:'Whey',product_type:'dairy',quantity:20,unit:'l',price:1}]},token));
  assert.equal(plan.outputs.length,2);
  const list=async(status)=>data(await api('/products/farm?status='+status,'GET',undefined,token));
  const planned=await until(async()=>{const p=await list('PRODUCTION');return p.total===2&&p;});
  assert.equal((await api('/products/'+planned.data[0].id,'PUT',{status:'ON_SALE'},token)).status,400);
  assert.equal((await api('/products','POST',{name:'Direct',product_type:'cheese',quantity:1,unit:'kg',price:5},token)).status,400);
  data(await api(path,'PUT',{status:'IN_PROGRESS'},token));
  assert.equal((await api(path,'PUT',{status:'COMPLETED',outputs:[]},token)).status,400);
  const final=data(await api(path,'PUT',{status:'COMPLETED',end_date:new Date().toISOString().slice(0,10),outputs:plan.outputs.map((p,i)=>({...p,quantity:i?18:8,name:i?'Final whey':'Final cheese'}))},token));
  assert.equal(final.outputs[0].planned.quantity,10);
  assert.equal(final.outputs[0].quantity,8);
  const stored=await until(async()=>{const p=await list('STORAGE');return p.total===2&&p;});
  assert.deepEqual(stored.data.map(p=>p.id).sort(),planned.data.map(p=>p.id).sort());
  assert.equal((await list('PRODUCTION')).total,0);
  const product=stored.data[0];
  assert.equal(data(await api('/products/'+product.id,'PUT',{status:'ON_SALE'},token)).status,'ON_SALE');
  assert.equal((await list('ON_SALE')).total,1);
  assert.equal(data(await api('/products/'+product.id,'PUT',{status:'STORAGE'},token)).status,'STORAGE');
  assert.equal((await list('STORAGE')).total,2);
  assert.equal((await api(path,'PUT',{status:'COMPLETED',outputs:plan.outputs},token)).status,400);
  console.log('PASS: multiple planned outputs, malformed input, creation restriction, sale restriction, completion snapshots, stable product identities, three state filters and both move actions.');
}finally{
  if(farm){
    sql('products',`DELETE FROM production_outputs WHERE batch_id IN(SELECT batch_id FROM products WHERE farm_id='${farm}'); DELETE FROM products WHERE farm_id='${farm}'; DELETE FROM production_states WHERE farm_id='${farm}';`);
    sql('productions',`DELETE FROM production_batches WHERE farm_id='${farm}';`);
    sql('auth',`UPDATE users SET farm_id=NULL WHERE id='${user}'; DELETE FROM farms WHERE id='${farm}';`);
  }
  if(user)sql('auth',`DELETE FROM users WHERE id='${user}';`);
}})().catch(e=>{console.error(e.message);process.exitCode=1;});
