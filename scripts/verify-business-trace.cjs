// End-to-end regression: uses disposable accounts/business and cleans only its own fixtures.
const fs = require('node:fs');
const crypto = require('node:crypto');
const { execFileSync } = require('node:child_process');
const assert = require('node:assert/strict');
const run = crypto.randomUUID();
const password = crypto.randomUUID();
const ownerEmail = `trace-owner-${run}@example.invalid`;
const customerEmail = `storage-customer-${run}@example.invalid`;
const workerEmail = `trace-worker-${run}@example.invalid`;
let business, owner, worker, qrPath;
const otherEmail=`other-${run}@example.invalid`;
let otherCustomer; let ordersStopped=false;
async function request(path, method='GET', body, token, key=crypto.randomUUID()) {
  const response = await fetch('http://localhost/api'+path, {method,
    headers: {'Content-Type':'application/json', 'Idempotency-Key':key, ...(token ? {Authorization:'Bearer '+token}: {})},
    body:body === undefined ? undefined : JSON.stringify(body), signal:AbortSignal.timeout(30000)});
  const bytes = Buffer.from(await response.arrayBuffer());
  let json; try { json=JSON.parse(bytes.toString()); } catch {}
  return {status:response.status, headers:response.headers, json, bytes};
}
function data(r, status=200) { assert.equal(r.status,status,r.bytes.toString()); return r.json.data; }
async function projectedTrace(qr) {
  for(let i=0;i<40;i++) {
    const result=await request(`/products/public/${qr}`);
    if(result.status===200 && result.json.data.business_name && result.json.data.batch?.raw_materials.length) return result.json.data;
    if(![200,409,500].includes(result.status)) throw new Error(result.bytes.toString());
    await new Promise(resolve=>setTimeout(resolve,500));
  }
  throw new Error('Traceability projection did not catch up');
}
const claims = token => JSON.parse(Buffer.from(token.split('.')[1],'base64url'));
function sql(container, db, query) { return execFileSync('docker',['exec',container,'psql','-U','postgres','-d',db,'-v','ON_ERROR_STOP=1','-c',query],{encoding:'utf8',stdio:['ignore','pipe','pipe']}).trim(); }
async function main() {
  try {
    for(const role of ['SYSTEM_ADMIN','WORKER']) assert.equal((await request('/auth/register','POST',{email:'forbidden-'+run+'@example.invalid',password,role,first_name:'Test',last_name:'Test',address:'Test'})).status,403);
    const registered = await request('/auth/register','POST',{email:ownerEmail,password,role:'BUSINESS_OWNER',first_name:'Test',last_name:'Owner',address:'Test address'});
    assert.equal(registered.status,201,registered.bytes.toString());
    const oldToken = registered.json.token;
    owner=claims(oldToken).sub;
    assert.equal(claims(oldToken).business_id,null);
    const created=data(await request('/auth/businesses','POST',{name:'Radionica Đorđević',address:'Šumadija'},oldToken),201);
    business=created.business.id;
    assert.equal(claims(created.token).business_id,business);
    const refreshed = await request('/auth/me','GET',undefined,oldToken);
    assert.equal(refreshed.status,200);
    assert.equal(refreshed.json.business_id,business);
    const token=refreshed.headers.get('x-session-token');
    assert.equal(claims(token).business_id,business);
    for (const path of ['/orders/analytics','/productions/batches','/products/business','/raw-materials/low-stock']) data(await request(path,'GET',undefined,token));
    const added=data(await request(`/auth/businesses/${business}/workers`,'POST',{email:workerEmail,password,first_name:'Test',last_name:'Worker',address:'Test address'},oldToken));
    worker=added.id;
    const login=await request('/auth/login','POST',{email:workerEmail,password});
    assert.equal(login.status,200);
    const workerToken=login.json.token;
    assert.equal(data(await request(`/auth/businesses/${business}`,'GET',undefined,workerToken)).name,'Radionica Đorđević');
    assert.equal((await request(`/auth/users/${worker}`,'DELETE',undefined,workerToken)).status,403);
    assert.equal((await request(`/auth/businesses/${business}`,'PUT',{name:'Forbidden'},workerToken)).status,403);
    const material=data(await request('/raw-materials','POST',{name:'Sveže mleko',material_type:'dairy',quantity:20,unit:'l',origin:'Šumadija, Srbija',supplier:'Gazdinstvo Đorđević',harvest_date:'2026-09-13',low_stock_threshold:60,received_date:'2026-09-14',expiry_date:'2026-09-20'},token),201);
    const batch=data(await request('/productions/batches','POST',{name:'Domaći sir - septembar',process_type:'fermentation',start_date:'2026-09-15',end_date:'2026-09-16',raw_materials:[{raw_material_id:material.id,quantity_used:5,unit:'l'}]},token),201);
    data(await request('/raw-materials/'+material.id,'PUT',{low_stock_threshold:60,quantity:20},token));
    assert.equal((await request('/productions/batches/'+batch.id,'PUT',{status:'COMPLETED',output_quantity:2},token)).status,400);
    data(await request('/productions/batches/'+batch.id,'PUT',{status:'IN_PROGRESS'},token));
    assert.equal((await request('/productions/batches/'+batch.id,'PUT',{status:'COMPLETED'},token)).status,400);
    data(await request('/productions/batches/'+batch.id,'PUT',{status:'COMPLETED',output_name:'Domaći sir',output_type:'dairy',output_unit:'kg',output_quantity:2,output_expiry_date:'2026-09-30'},token));
    let product;
    for(let i=0;i<60;i++) {
      const stock=data(await request('/products/business?is_active=false','GET',undefined,token));
      product=stock.data.find(p=>p.batch_id===batch.id);
      if(product) break;
      await new Promise(r=>setTimeout(r,500));
    }
    assert.ok(product,'Completed batch did not create Storage output');
    assert.equal(product.is_active,false); assert.equal(Number(product.quantity),2);
    assert.equal((await request('/products/'+product.id,'PUT',{quantity:99},token)).status,400);
    assert.equal((await request('/products/'+product.id,'PUT',{is_active:true},token)).status,400);
    const registeredCustomer=await request('/auth/register','POST',{email:customerEmail,password,role:'CUSTOMER',first_name:'Customer',last_name:'Account',address:'Test address'});
    assert.equal(registeredCustomer.status,201);
    const customerToken=registeredCustomer.json.token;
    assert.ok(!data(await request('/products?is_active=false&business_id='+business,'GET',undefined,customerToken)).data.some(p=>p.id===product.id),'Customer saw Storage');
    product=data(await request('/products/'+product.id,'PUT',{price:8,is_active:true},token));
    const active=data(await request('/products?business_id='+business,'GET',undefined,customerToken));
    assert.equal(active.total,1);assert.equal(active.data[0].id,product.id);
    const order=data(await request('/orders','POST',{customer_name:'Spoofed',customer_email:'spoof@example.invalid',items:[{product_id:product.id,quantity:1}],notes:'At the door'},customerToken),201).orders[0];
    assert.equal(order.customer_name,'Customer Account');assert.equal(order.customer_email,customerEmail);
    const customerId=claims(customerToken).sub;
    const filtered=data(await request('/orders/user/'+customerId+'?business_id='+business,'GET',undefined,customerToken));
    assert.equal(filtered.total,1);assert.equal(filtered.data[0].id,order.id);
    assert.equal(data(await request('/orders/user/'+customerId+'?business_id='+crypto.randomUUID(),'GET',undefined,customerToken)).total,0);
    assert.equal((await request('/orders/user/'+owner,'GET',undefined,customerToken)).status,403);
    assert.equal((await request('/productions/batches/'+batch.id,'PUT',{status:'COMPLETED',output_quantity:90},token)).status,400);
    sql('productions-db','productions_db',`UPDATE integration_outbox SET published_at=NULL WHERE entity_id='${batch.id}';`);
    await new Promise(r=>setTimeout(r,1600));
    assert.equal(data(await request('/products/business?is_active=true','GET',undefined,token)).total,1);
    assert.equal(Number(data(await request('/products/'+product.id,'GET',undefined,token)).quantity),1);

    // Authorization, no public stock mutation, immutable completed children.
    const other=await request('/auth/register','POST',{email:otherEmail,password,role:'CUSTOMER',first_name:'Other',last_name:'Customer',address:'Test'});
    assert.equal(other.status,201);otherCustomer=claims(other.json.token).sub;
    assert.equal((await request('/orders/'+order.id,'GET',undefined,other.json.token)).status,404);
    assert.equal((await request('/products/'+product.id+'/decrement','PATCH',{quantity:1},customerToken)).status,404);
    assert.ok([400,409].includes((await request('/productions/batches/'+batch.id+'/steps','POST',{name:'Late step',step_order:1},token)).status));
    data(await request('/orders/'+order.id+'/status','PUT',{status:'CANCELLED'},token));
    assert.equal(Number(data(await request('/products/'+product.id,'GET',undefined,token)).quantity),2);
    assert.equal((await request('/orders/'+order.id+'/status','PUT',{status:'CANCELLED'},token)).status,400);
    const keys=[crypto.randomUUID(),crypto.randomUUID()];
    const cart={items:[{product_id:product.id,quantity:2}]};
    const race=await Promise.all(keys.map(k=>request('/orders','POST',cart,customerToken,k)));
    assert.deepEqual(race.map(r=>r.status).sort(),[201,409]);
    const winner=race.findIndex(r=>r.status===201);
    const replay=data(await request('/orders','POST',cart,customerToken,keys[winner]),201);
    assert.equal(replay.orders[0].id,race[winner].json.data.orders[0].id);
    assert.equal(Number(data(await request('/products/'+product.id,'GET',undefined,token)).quantity),0);
    assert.equal((await request('/orders','POST',{items:[{product_id:product.id,quantity:1}]},customerToken,keys[winner])).status,409);
    // Crash window: reserve remotely, then restart Orders before its local order commit.
    data(await request('/orders/'+replay.orders[0].id+'/status','PUT',{status:'CANCELLED'},token));
    ordersStopped=true;execFileSync('docker',['compose','stop','orders-service'],{stdio:'pipe'});
    const recoverKey=crypto.randomUUID();
    sql('orders-db','orders_db',`INSERT INTO checkout_jobs(id,customer_id,request,payload) SELECT '${recoverKey}',customer_id,request,payload FROM checkout_jobs WHERE id='${keys[winner]}';`);
    const settings=Object.fromEntries(fs.readFileSync('.env','utf8').split(/\r?\n/).filter(s=>/^[A-Z_]+=/.test(s)).map(s=>{const i=s.indexOf('=');return [s.slice(0,i),s.slice(i+1).replace(/^['"]|['"]$/g,'')]}));
    const enc=x=>Buffer.from(JSON.stringify(x)).toString('base64url');const now=Math.floor(Date.now()/1000);
    const unsigned=enc({alg:'HS256',typ:'JWT'})+'.'+enc({sub:recoverKey,role:'ORDER_STOCK',email:'',business_id:null,iat:now,exp:now+60});
    const technical=unsigned+'.'+crypto.createHmac('sha256',settings.JWT_SECRET).update(unsigned).digest('base64url');
    // Match the exact decimal strings stored in the copied checkout payload; reservation replay
    // deliberately rejects semantically equal but byte-different payloads.
    const reserved=await fetch('http://localhost:'+settings.PRODUCTS_PORT+'/internal/reservations/'+recoverKey,{method:'POST',headers:{Authorization:'Bearer '+technical,'Content-Type':'application/json'},body:JSON.stringify([{product_id:product.id,quantity:'2',unit_price:'8'}])});
    assert.equal(reserved.status,204,await reserved.text());
    execFileSync('docker',['compose','start','orders-service'],{stdio:'pipe'});ordersStopped=false;
    let recovered;
    for(let i=0;i<40;i++){recovered=sql('orders-db','orders_db',`SELECT status FROM checkout_jobs WHERE id='${recoverKey}';`);if(recovered.includes('COMPLETED'))break;await new Promise(r=>setTimeout(r,500));}
    assert.ok(recovered.includes('COMPLETED'),'Checkout was not recovered');
    const recoveredResponse=data(await request('/orders','POST',cart,customerToken,recoverKey),201);
    assert.equal(recoveredResponse.orders.length,1);
    assert.equal(Number(data(await request('/products/'+product.id,'GET',undefined,token)).quantity),0);
    data(await request('/products/'+product.id,'PUT',{price:9},token));
    assert.equal(Number(data(await request('/products/'+product.id,'GET',undefined,token)).quantity),0);

    qrPath=product.qr_path;
    const trace=await projectedTrace(product.qr_token);
    assert.equal(trace.business_name,'Radionica Đorđević');
    assert.equal(trace.product.expiry_date,'2026-09-30');
    assert.equal(trace.batch.start_date,'2026-09-15');
    assert.equal(trace.batch.end_date,'2026-09-16');
    assert.equal(trace.batch.raw_materials[0].harvest_date,'2026-09-13');
    assert.equal(trace.batch.raw_materials[0].origin,'Šumadija, Srbija');
    assert.equal(trace.batch.raw_materials[0].received_date,undefined);
    assert.equal(trace.batch.raw_materials[0].expiry_date,undefined);
    assert.equal(trace.product.business_id,undefined); assert.equal(trace.batch.raw_materials[0].supplier,undefined);
    // Trace dates are historical snapshots, not overwritten by a stock edit.
    data(await request(`/raw-materials/${material.id}`,'PUT',{received_date:'2026-09-15',expiry_date:'2026-09-22'},token));
    assert.equal(data(await request(`/products/${product.id}/provenance`,'GET',undefined,token)).batch.raw_materials[0].expiry_date,'2026-09-20');
    const pdf=await request(`/products/public/${product.qr_token}/certificate.pdf`);
    assert.equal(pdf.status,200,pdf.bytes.toString());
    assert.ok(pdf.bytes.subarray(0,5).equals(Buffer.from('%PDF-')));
    fs.mkdirSync('tmp/pdfs',{recursive:true});
    fs.writeFileSync('tmp/pdfs/traceability-check.pdf',pdf.bytes);
    fs.writeFileSync('tmp/pdfs/provenance-check.json',JSON.stringify(trace));
    assert.equal((await request(`/auth/businesses/${business}/trace`,'GET',undefined,workerToken)).status,403);
    assert.equal((await request(`/productions/batches/${batch.id}/trace`,'GET',undefined,workerToken)).status,403);
    assert.equal((await request('/products','POST',{name:'Invalid expiry',product_type:'dairy',price:8,quantity:2,unit:'kg',expiry_date:'2026-09-10',batch_id:batch.id},token)).status,400);
    console.log('PASS: completion validation, automatic Storage, immutable yield, publish/price checks, producer filters, account contact data, duplicate delivery, stock deduction, trace/PDF and role restrictions.');
  } catch(error) { console.error(error.stack); throw error; } finally {
    if(ordersStopped)execFileSync('docker',['compose','start','orders-service'],{stdio:'pipe'});
    if(qrPath && /^qr\/[a-zA-Z0-9_-]+\.png$/.test(qrPath)) {
      execFileSync('docker',['exec','products-service','python3','-c','import pathlib,sys; pathlib.Path("/app/uploads",sys.argv[1]).unlink(missing_ok=True)',qrPath]);
    }
    if(business) {
      sql('orders-db','orders_db',`DELETE FROM orders WHERE business_id='${business}'; DELETE FROM checkout_jobs WHERE payload->'lines' @> '[{"business_id":"${business}"}]'::jsonb;`);
      sql('products-db','products_db',`WITH removed AS (DELETE FROM stock_reservation_items WHERE business_id='${business}' RETURNING reservation_id) DELETE FROM stock_reservations WHERE id IN (SELECT reservation_id FROM removed); DELETE FROM production_outputs WHERE product_id IN (SELECT id FROM products WHERE business_id='${business}'); DELETE FROM products WHERE business_id='${business}'; DELETE FROM production_states WHERE business_id='${business}';`);
      sql('productions-db','productions_db',`DELETE FROM production_batches WHERE business_id='${business}';`);
      sql('raw-materials-db','raw_materials_db',`DELETE FROM production_consumption WHERE business_id='${business}'; DELETE FROM raw_materials WHERE business_id='${business}';`);
      sql('auth-db','auth_db',`UPDATE users SET business_id=NULL WHERE business_id='${business}'; DELETE FROM businesses WHERE id='${business}';`);
    }
    sql('auth-db','auth_db',`DELETE FROM users WHERE email IN ('${ownerEmail}','${workerEmail}','${customerEmail}','${otherEmail}');`);
    console.log('Removed disposable accounts and business data.');
  }
}
main().catch(e=>{console.error(e.message);process.exitCode=1;});
