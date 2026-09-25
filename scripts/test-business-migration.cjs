// Apply the full migration chain and upgrade fixtures in rolled-back isolated schemas.
const {readFileSync, readdirSync} = require('node:fs');
const {join} = require('node:path');
const {execFileSync} = require('node:child_process');
const root = join(__dirname, '..');
const id = '00000000-0000-0000-0000-000000000001';
const other = '00000000-0000-0000-0000-000000000002';
const check = (condition, message) => `DO $$ BEGIN IF NOT (${condition}) THEN RAISE EXCEPTION '${message}'; END IF; END $$;`;
const fixtures = {
  auth: `INSERT INTO users(id,email,password_hash,role,farm_id,first_name,last_name,address) VALUES ('${id}','migration@example.invalid','unused','FARM_OWNER','${id}','A','B','Address');
    INSERT INTO farms(id,name,owner_id,address) VALUES ('${id}','Farm soap','${id}','Address');`,
  'raw-materials': `INSERT INTO raw_materials(id,farm_id,name,material_type,quantity,unit) VALUES ('${id}','${id}','Oil','other',5,'l');
    INSERT INTO type_catalog(farm_id,name) VALUES ('${id}','Custom oil');`,
  productions: `INSERT INTO production_batches(id,farm_id,name,status) VALUES ('${id}','${id}','Soap batch','PLANNED');
    INSERT INTO batch_raw_materials(id,batch_id,farm_id,raw_material_id,raw_material_name,material_type,quantity_used,unit) VALUES ('${other}','${id}','${id}','${other}','Oil','other',1,'l');`,
  products: `INSERT INTO products(id,farm_id,name,product_type,quantity,unit,price) VALUES ('${id}','${id}','Soap','other',5,'pcs',3);
    INSERT INTO stock_reservations(id,payload) VALUES ('${id}','[{"farm_id":"${id}","name":"Farm soap"}]');
    INSERT INTO stock_reservation_items(reservation_id,product_id,farm_id,quantity) VALUES ('${id}','${id}','${id}',1);`,
  orders: `INSERT INTO orders(id,farm_id) VALUES ('${id}','${id}');
    INSERT INTO checkout_jobs(id,customer_id,request,payload,response) VALUES ('${id}','${other}','{}','{"lines":[{"farm_id":"${id}"}]}','{"orders":[{"farm_id":"${id}"}]}');
    INSERT INTO order_stock_links(order_id,checkout_id,farm_id) VALUES ('${id}','${id}','${id}');
    INSERT INTO stock_release_jobs(order_id,checkout_id,farm_id) VALUES ('${id}','${id}','${id}');`,
  'read-models': `INSERT INTO projection_entities(source,entity_type,entity_id,sequence,deleted,data) VALUES
    ('auth','farms','${id}',1,false,'{"id":"${id}","name":"Farm soap"}'),
    ('auth','users','${other}',2,false,'{"id":"${other}","farm_id":"${id}","role":"FARM_OWNER"}');`
};
const assertions = {
  auth: check(`(SELECT role='BUSINESS_OWNER' AND business_id='${id}' FROM users WHERE id='${id}')`, 'Owner or scope lost')
    + check(`(SELECT name='Farm soap' FROM businesses WHERE id='${id}')`, 'User content changed')
    + `UPDATE businesses SET description='Updated' WHERE id='${id}';`
    + check(`EXISTS(SELECT 1 FROM integration_outbox WHERE entity_type='users' AND data->>'role'='BUSINESS_OWNER' AND data->>'business_id'='${id}')`, 'Auth event not upgraded'),
  'raw-materials': check(`(SELECT business_id='${id}' AND quantity=5 FROM raw_materials WHERE id='${id}')`, 'Material scope or stock lost')
    + `INSERT INTO type_catalog(business_id,name) VALUES ('${id}','CUSTOM OIL') ON CONFLICT (business_id,lower(name)) WHERE business_id IS NOT NULL DO UPDATE SET name=type_catalog.name;`
    + check(`(SELECT count(*)=1 FROM type_catalog WHERE business_id='${id}')`, 'Catalog isolation changed'),
  productions: `UPDATE production_batches SET status='CANCELLED' WHERE id='${id}';`
    + check(`EXISTS(SELECT 1 FROM material_release_jobs WHERE operation_id='${other}' AND business_id='${id}')`, 'Cancellation trigger not upgraded')
    + `INSERT INTO production_batches(id,business_id,name) VALUES ('${other}','${id}','Second batch');
       INSERT INTO batch_raw_materials(id,batch_id,business_id,raw_material_id,raw_material_name,material_type,quantity_used,unit) VALUES ('${id}','${other}','${id}','${id}','Oil','other',1,'l');
       DELETE FROM batch_raw_materials WHERE id='${id}';`
    + check(`EXISTS(SELECT 1 FROM material_release_jobs WHERE operation_id='${id}' AND business_id='${id}')`, 'Deletion trigger not upgraded'),
  products: check(`(SELECT business_id='${id}' FROM stock_reservation_items WHERE reservation_id='${id}')`, 'Reservation scope lost')
    + check(`(SELECT payload->0->>'business_id'='${id}' AND payload->0->>'name'='Farm soap' FROM stock_reservations WHERE id='${id}')`, 'Reservation payload lost'),
  orders: check(`(SELECT payload->'lines'->0->>'business_id'='${id}' AND response->'orders'->0->>'business_id'='${id}' FROM checkout_jobs WHERE id='${id}')`, 'Checkout recovery not upgraded')
    + check(`(SELECT business_id='${id}' FROM stock_release_jobs WHERE order_id='${id}')`, 'Release job lost'),
  'read-models': check(`EXISTS(SELECT 1 FROM projection_entities WHERE entity_type='businesses' AND data->>'name'='Farm soap')`, 'Business projection lost')
    + check(`EXISTS(SELECT 1 FROM projection_entities WHERE entity_type='users' AND data->>'business_id'='${id}' AND data->>'role'='BUSINESS_OWNER')`, 'User projection lost')
    + check(`EXISTS(SELECT 1 FROM pg_indexes WHERE schemaname=current_schema() AND indexname='projection_business' AND indexdef LIKE '%business_id%')`, 'Projection index not upgraded')
};
for (const service of Object.keys(fixtures)) {
  const directory = join(root,'services',service,'migrations');
  const files = readdirSync(directory).filter(f=>f.endsWith('.sql')).sort();
  const upgrade = files.find(f=>f.includes('business_terminology'));
  const schema = 'business_upgrade_test';
  const baseline = files.filter(f=>f!==upgrade).map(f=>readFileSync(join(directory,f),'utf8')).join('\n');
  const sql = `BEGIN; CREATE SCHEMA ${schema}; SET LOCAL search_path=${schema};\n${baseline}\n${fixtures[service]}\n${readFileSync(join(directory,upgrade),'utf8')}\n${assertions[service]}\n`
    + check(`NOT EXISTS(SELECT 1 FROM information_schema.columns WHERE table_schema=current_schema() AND column_name='farm_id')`, 'Legacy column remains')
    + (service==='read-models' ? '' : check(`NOT EXISTS(SELECT 1 FROM integration_outbox WHERE data ? 'farm_id' OR entity_type='farms')`, 'Legacy event remains'))
    + '\nROLLBACK;';
  try {
    execFileSync('docker',['exec','-i',`${service}-db`,'sh','-c','psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -v ON_ERROR_STOP=1 -q'], {input:sql,encoding:'utf8',stdio:['pipe','pipe','pipe'],timeout:60000});
    console.log(`${service}: business migration and data preservation passed`);
  } catch (error) {
    console.error(error.stderr?.toString() || error.message);
    process.exitCode=1;
    break;
  }
}
