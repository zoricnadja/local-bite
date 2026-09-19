// Disposable end-to-end fixtures. Briefly stops RabbitMQ to prove outbox recovery.
const fs=require('node:fs'), crypto=require('node:crypto'), assert=require('node:assert/strict');
const {execFileSync}=require('node:child_process');
const run=crypto.randomUUID(), password=crypto.randomUUID();
const emails=[`cqrs-owner-${run}@example.invalid`,`cqrs-customer-${run}@example.invalid`];
const settings=Object.fromEntries(fs.readFileSync('.env','utf8').split(/\r?\n/).filter(s=>/^[A-Z_]+=/.test(s)).map(s=>{const i=s.indexOf('=');return [s.slice(0,i),s.slice(i+1).replace(/^['"]|['"]$/g,'')]}));
let farm, product, brokerStopped=false;
const sleep=ms=>new Promise(resolve=>setTimeout(resolve,ms));
function docker(args) {return execFileSync('docker',args,{encoding:'utf8',stdio:['ignore','pipe','pipe'],timeout:60000}).trim();}
function sql(service,query) {return docker(['exec',`${service}-db`,'psql','-U',settings.POSTGRES_USER||'postgres','-d',service.replaceAll('-','_')+'_db','-At','-v','ON_ERROR_STOP=1','-c',query]);}
async function request(path,method='GET',body,token) {
 const res=await fetch('http://localhost/api'+path,{method,headers:{'Content-Type':'application/json', 'Idempotency-Key':crypto.randomUUID(),...(token?{Authorization:'Bearer '+token}:{})},body:body===undefined?undefined:JSON.stringify(body),signal:AbortSignal.timeout(20000)});
 const text=await res.text(); let json;try{json=JSON.parse(text)}catch{}
 return {status:res.status,json,text};
}
function data(r,code=200){assert.equal(r.status,code,r.text);return r.json.data;}
async function until(name,fn){for(let i=0;i<80;i++){const v=await fn();if(v)return v;await sleep(500);}throw Error('Timed out: '+name);}
async function register(index,role){const r=await request('/auth/register','POST',{email:emails[index],password,role,first_name:'CQRS',last_name:'Test',address:'Test only'});assert.equal(r.status,201,r.text);return r.json.token;}
async function publish(event){
 const res=await fetch('http://localhost:15672/api/exchanges/%2F/local_bite.events.v1/publish',{method:'POST',headers:{Authorization:'Basic '+Buffer.from(settings.RABBITMQ_DEFAULT_USER+':'+settings.RABBITMQ_DEFAULT_PASS).toString('base64'),'Content-Type':'application/json'},body:JSON.stringify({routing_key:'raw-materials.raw_materials.update',payload:JSON.stringify(event),payload_encoding:'string',properties:{delivery_mode:2}}),signal:AbortSignal.timeout(10000)});
 assert.equal(res.status,200);assert.equal((await res.json()).routed,true);
}
async function main(){try{
 const oldOwner=await register(0,'FARM_OWNER'), customer=await register(1,'CUSTOMER');
 const created=data(await request('/auth/farms','POST',{name:'CQRS test farm',address:'Test'},oldOwner),201);farm=created.farm.id;const owner=created.token;
 const material=data(await request('/raw-materials','POST',{name:'CQRS milk',material_type:'dairy',quantity:20,unit:'l',low_stock_threshold:30,received_date:'2026-09-14',expiry_date:'2026-09-30'},owner),201);
 const batch=data(await request('/productions/batches','POST',{name:'CQRS batch',process_type:'fermentation',raw_materials:[{raw_material_id:material.id,quantity_used:5,unit:'l'}]},owner),201);
 data(await request('/productions/batches/'+batch.id,'PUT',{status:'IN_PROGRESS'},owner));
 data(await request('/productions/batches/'+batch.id,'PUT',{status:'COMPLETED',end_date:'2026-09-17',output_name:'CQRS cheese',output_type:'dairy',output_quantity:10,output_unit:'kg',output_expiry_date:'2026-10-01'},owner));
 product=await until('production output',async()=>data(await request('/products/farm','GET',undefined,owner)).data.find(p=>p.batch_id===batch.id));
 product=data(await request('/products/'+product.id,'PUT',{price:8,is_active:true},owner));
 const orders=data(await request('/orders','POST',{items:[{product_id:product.id,quantity:2}]},customer),201).orders;
 assert.equal(orders.length,1);
 for(const status of ['CONFIRMED','SHIPPED','DELIVERED']) data(await request(`/orders/${orders[0].id}/status`,'PUT',{status},owner));
 const dashboard=await until('dashboard aggregates',async()=>{const r=await request('/queries/dashboard','GET',undefined,owner);return r.status===200&&r.json.data.stats.revenue===16&&r.json.data.stats.totalProducts===1&&r.json.data.lowStockItems.length===1?r.json.data:false;});
 assert.equal(dashboard.stats.totalOrders,1);assert.equal(dashboard.plannedCount,0);
 const mine=data(await request('/queries/dashboard','GET',undefined,customer));assert.deepEqual(mine.recentOrders.map(o=>o.id),[orders[0].id]);
 assert.equal((await request('/queries/dashboard')).status,401);
 // Verify all services emitted events, and auth never leaked credentials.
 for(const source of ['auth','raw-materials','products','productions','orders']) assert.ok(Number(sql('read-models',`SELECT count(*) FROM projection_receipts WHERE source='${source}'`))>0,source);
 assert.equal(sql('auth',"SELECT count(*) FROM integration_outbox WHERE entity_type='users' AND (data ? 'password_hash' OR data ? 'email')"),'0');
 // A transaction rollback must leave neither a change nor its event.
 const before=sql('raw-materials',`SELECT count(*) FROM integration_outbox WHERE entity_id='${material.id}'`);
 sql('raw-materials',`BEGIN; UPDATE raw_materials SET quantity=99 WHERE id='${material.id}'; ROLLBACK;`);
 assert.equal(sql('raw-materials',`SELECT count(*) FROM integration_outbox WHERE entity_id='${material.id}'`),before);
 // Re-delivery is idempotent and an old event cannot overwrite newer state.
 const first=JSON.parse(sql('raw-materials',`SELECT json_build_object('schema_version',1,'source','raw-materials','sequence',sequence,'entity_type',entity_type,'entity_id',entity_id,'operation',operation,'data',data) FROM integration_outbox WHERE entity_id='${material.id}' ORDER BY sequence LIMIT 1`));
 await publish(first);await publish(first);
 await until('duplicate receipt',()=>sql('read-models',`SELECT count(*) FROM projection_receipts WHERE source='raw-materials' AND sequence=${first.sequence}`)==='1');
 sql('read-models',`DELETE FROM projection_receipts WHERE source='raw-materials' AND sequence=${first.sequence}`);
 await publish(first);
 await until('old event processed',()=>sql('read-models',`SELECT count(*) FROM projection_receipts WHERE source='raw-materials' AND sequence=${first.sequence}`)==='1');
 assert.equal(Number(sql('read-models',`SELECT data->>'quantity' FROM projection_entities WHERE entity_id='${material.id}' AND entity_type='raw_materials'`)),15);
 // Command writes succeed during broker downtime, then the backlog catches up.
 console.log('Checking broker outage recovery...');
 brokerStopped=true;docker(['compose','stop','rabbitmq']);
 data(await request(`/raw-materials/${material.id}`,'PUT',{quantity:12},owner));
 assert.ok(Number(sql('raw-materials',`SELECT count(*) FROM integration_outbox WHERE entity_id='${material.id}' AND published_at IS NULL`))>0);
 assert.equal(Number(sql('read-models',`SELECT data->>'quantity' FROM projection_entities WHERE entity_id='${material.id}' AND entity_type='raw_materials'`)),15);
 docker(['compose','start','rabbitmq']);brokerStopped=false;
 await until('outbox recovery',()=>Number(sql('read-models',`SELECT data->>'quantity' FROM projection_entities WHERE entity_id='${material.id}' AND entity_type='raw_materials'`))===12);
 // Query model remains sufficient while the producer services are unreachable.
 let paused=false;
 try {docker(['compose','stop','auth-service','productions-service','raw-materials-service']);paused=true;
  const trace=await request(`/products/public/${product.qr_token}`);assert.equal(trace.status,200,trace.text);assert.equal(trace.json.data.farm_name,'CQRS test farm');assert.equal(trace.json.data.batch.raw_materials[0].quantity_used,undefined);
  assert.equal(data(await request('/queries/dashboard','GET',undefined,owner)).stats.revenue,16);
 } finally {if(paused)docker(['compose','start','auth-service','productions-service','raw-materials-service']);}
 console.log('PASS: all five publishers, CQRS trace/dashboard, isolation, transactional rollback, duplicates, out-of-order delivery, broker outage recovery and independent queries.');
}catch(error){console.error(error.stack);throw error;}finally{
 if(brokerStopped)docker(['compose','start','rabbitmq']);
 if(product?.qr_path&&/^qr\/[a-zA-Z0-9_-]+\.png$/.test(product.qr_path))docker(['exec','products-service','python3','-c','import pathlib,sys;pathlib.Path("/app/uploads",sys.argv[1]).unlink(missing_ok=True)',product.qr_path]);
 if(farm){
  sql('orders',`DELETE FROM orders WHERE farm_id='${farm}'; DELETE FROM checkout_jobs WHERE payload->'lines' @> '[{"farm_id":"${farm}"}]'::jsonb;`);
  sql('products',`WITH removed AS (DELETE FROM stock_reservation_items WHERE farm_id='${farm}' RETURNING reservation_id) DELETE FROM stock_reservations WHERE id IN (SELECT reservation_id FROM removed); DELETE FROM production_outputs WHERE product_id IN (SELECT id FROM products WHERE farm_id='${farm}'); DELETE FROM products WHERE farm_id='${farm}'; DELETE FROM production_states WHERE farm_id='${farm}';`);
  sql('productions',`DELETE FROM production_batches WHERE farm_id='${farm}';`);
  sql('raw-materials',`DELETE FROM production_consumption WHERE farm_id='${farm}';DELETE FROM raw_materials WHERE farm_id='${farm}';`);
  sql('auth',`UPDATE users SET farm_id=NULL WHERE farm_id='${farm}';DELETE FROM farms WHERE id='${farm}';`);
 }
 sql('auth',`DELETE FROM users WHERE email IN ('${emails[0]}','${emails[1]}');`);
 console.log('Removed CQRS fixtures.');
}}
main().catch(e=>{console.error(e.message);process.exitCode=1;});
