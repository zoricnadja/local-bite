// Publish only to the output queue, avoiding synthetic events in the shared query model.
const fs=require('node:fs'), crypto=require('node:crypto'), assert=require('node:assert/strict');
const {execFileSync}=require('node:child_process');
const settings=Object.fromEntries(fs.readFileSync('.env','utf8').split(/\r?\n/).filter(s=>/^[A-Z_]+=/.test(s)).map(s=>{const i=s.indexOf('=');return [s.slice(0,i),s.slice(i+1).replace(/^['"]|['"]$/g,'')]}));
const farm=crypto.randomUUID(), id=crypto.randomUUID(), legacy=crypto.randomUUID();
const base='http://localhost:15672/api';
const headers={Authorization:'Basic '+Buffer.from(settings.RABBITMQ_DEFAULT_USER+':'+settings.RABBITMQ_DEFAULT_PASS).toString('base64'),'Content-Type':'application/json'};
function sql(query){return execFileSync('docker',['exec','products-db','psql','-U',settings.POSTGRES_USER||'postgres','-d','products_db','-At','-v','ON_ERROR_STOP=1','-c',query],{encoding:'utf8',stdio:['ignore','pipe','pipe']}).trim();}
async function until(fn){for(let i=0;i<60;i++){if(await fn())return;await new Promise(r=>setTimeout(r,500));}throw Error('Output event did not settle');}
async function publish(event){const r=await fetch(base+'/exchanges/%2F/amq.default/publish',{method:'POST',headers,body:JSON.stringify({routing_key:'local_bite.production_outputs.v1',payload:JSON.stringify(event),payload_encoding:'string',properties:{delivery_mode:2}})});assert.equal(r.status,200);assert.equal((await r.json()).routed,true);}
async function deadCount(){const r=await fetch(base+'/queues/%2F/local_bite.production_outputs.dead.v1',{headers});assert.equal(r.status,200);return (await r.json()).messages??0;}
(async()=>{try{
 const event={schema_version:1,source:'productions',sequence:2,entity_type:'production_batches',entity_id:id,operation:'DELETE',data:{id,farm_id:farm,status:'COMPLETED',output_name:'Test',output_type:'cheese',output_unit:'kg',output_quantity:2}};
 await publish(event);await until(()=>sql(`SELECT deleted FROM production_states WHERE batch_id='${id}'`)==='t');
 await publish({...event,sequence:1,operation:'UPDATE'});
 await publish({...event,entity_id:legacy,operation:'UPDATE',data:{...event.data,id:legacy,output_quantity:null}});
 await until(()=>sql(`SELECT count(*) FROM production_states WHERE batch_id='${legacy}'`)==='1');
 assert.equal(sql(`SELECT count(*) FROM products WHERE farm_id='${farm}'`),'0','Stale or legacy event created inventory');
 const before=await deadCount();
 await publish({...event,sequence:3,operation:'UPDATE',data:{...event.data,output_unit:''}});
 await until(async()=>await deadCount()>before);
 // A later valid event demonstrates that poison delivery did not block the queue.
 await publish({...event,sequence:4});
 await until(()=>sql(`SELECT sequence FROM production_states WHERE batch_id='${id}'`)==='4');
 console.log('PASS: stale completion cannot resurrect inventory; legacy output creates no stock; poison delivery goes to DLQ and subsequent messages continue.');
}finally{sql(`DELETE FROM production_states WHERE farm_id='${farm}';`);}})().catch(e=>{console.error(e.message);process.exitCode=1;});
