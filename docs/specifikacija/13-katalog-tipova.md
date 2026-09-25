# 13 — Katalog Rust modela i DTO tipova

[Sadržaj specifikacije](README.md) · [Semantika podataka](05-modeli-i-podaci.md)

Polja su izdvojena iz trenutnog izvornog koda. Katalog uključuje persistence modele, DTO-e, upitne parametre i integracione strukture; sama deklaracija ne dokazuje da je tip aktivno korišćen. Option označava opcionu vrednost u Rust-u, Vec niz, BigDecimal decimalni tip. Serde atributi mogu menjati JSON reprezentaciju. User.password_hash se ne serializuje. Tačna SQL ograničenja nalaze se u migracijama i dokumentu 05.

## Dopune recovery/reservation iteracije

- `service_auth.rs`: `token(role, operation, business)` izdaje tehnički JWT od 60 sekundi; `require` proverava role i subject.
- `checkout.rs::Line`: trajni snapshot proizvoda, firme, naziva, tipa, jedinice, količine i cene; decimale su stringovi.
- `reservations.rs::Item`: interni `product_id`, `quantity`, `unit_price`; stringovi omogućavaju tačnu replay jednakost.
- `public_provenance.rs`: `PublicProduct`, `PublicMaterial`, `PublicStep`, `PublicBatch` i `PublicProvenance` čine javni allowlist.

Neaktivni tipovi za javni decrement, direktan product insert, stari product snapshot i stari create-order response uklonjeni su iz koda i kataloga.

## libs/common/src/events.rs

Izvor: [libs/common/src/events.rs](../../libs/common/src/events.rs)

### IntegrationEvent (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| schema_version | `u32` | Ne |
| source | `String` | Ne |
| sequence | `i64` | Ne |
| entity_type | `String` | Ne |
| entity_id | `uuid::Uuid` | Ne |
| operation | `String` | Ne |
| data | `Value` | Ne |

## libs/common/src/jwt.rs

Izvor: [libs/common/src/jwt.rs](../../libs/common/src/jwt.rs)

### Claims (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| sub | `Uuid` | Ne |
| email | `String` | Ne |
| role | `String` | Ne |
| business_id | `Option<Uuid>` | Da |
| exp | `usize` | Ne |
| iat | `usize` | Ne |

## libs/common/src/models.rs

Izvor: [libs/common/src/models.rs](../../libs/common/src/models.rs)

### Role (enum)

```rust
SystemAdmin,
    BusinessOwner,
    Worker,
    Customer,
```

## libs/common/src/paginated_response.rs

Izvor: [libs/common/src/paginated_response.rs](../../libs/common/src/paginated_response.rs)

### PaginatedResponse (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| data | `Vec<T>` | Ne |
| total | `i64` | Ne |
| page | `i64` | Ne |
| limit | `i64` | Ne |

## services/auth/src/dtos/create_business_request.rs

Izvor: [services/auth/src/dtos/create_business_request.rs](../../services/auth/src/dtos/create_business_request.rs)

### CreateBusinessRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| name | `String` | Ne |
| address | `String` | Ne |
| phone | `Option<String>` | Da |
| description | `Option<String>` | Da |
| website | `Option<String>` | Da |

## services/auth/src/dtos/create_business_response.rs

Izvor: [services/auth/src/dtos/create_business_response.rs](../../services/auth/src/dtos/create_business_response.rs)

### CreateBusinessResult (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| business | `Business` | Ne |
| token | `String` | Ne |

## services/auth/src/dtos/login_request.rs

Izvor: [services/auth/src/dtos/login_request.rs](../../services/auth/src/dtos/login_request.rs)

### LoginRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| email | `String` | Ne |
| password | `String` | Ne |

## services/auth/src/dtos/login_response.rs

Izvor: [services/auth/src/dtos/login_response.rs](../../services/auth/src/dtos/login_response.rs)

### LoginResponse (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| token | `String` | Ne |

## services/auth/src/dtos/register_request.rs

Izvor: [services/auth/src/dtos/register_request.rs](../../services/auth/src/dtos/register_request.rs)

### RegisterRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| email | `String` | Ne |
| password | `String` | Ne |
| role | `Option<String>` | Da |
| first_name | `String` | Ne |
| last_name | `String` | Ne |
| address | `String` | Ne |
| phone | `Option<String>` | Da |
| photo_url | `Option<String>` | Da |
| date_of_birth | `Option<NaiveDate>` | Da |

## services/auth/src/dtos/update_business_request.rs

Izvor: [services/auth/src/dtos/update_business_request.rs](../../services/auth/src/dtos/update_business_request.rs)

### UpdateBusinessRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| name | `Option<String>` | Da |
| address | `Option<String>` | Da |
| phone | `Option<String>` | Da |
| description | `Option<String>` | Da |
| website | `Option<String>` | Da |

## services/auth/src/dtos/update_user_request.rs

Izvor: [services/auth/src/dtos/update_user_request.rs](../../services/auth/src/dtos/update_user_request.rs)

### UpdateUserRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| email | `Option<String>` | Da |
| password | `Option<String>` | Da |
| role | `Option<String>` | Da |
| first_name | `Option<String>` | Da |
| last_name | `Option<String>` | Da |
| address | `Option<String>` | Da |
| phone | `Option<String>` | Da |
| photo_url | `Option<String>` | Da |
| date_of_birth | `Option<NaiveDate>` | Da |

## services/auth/src/dtos/worker_dto.rs

Izvor: [services/auth/src/dtos/worker_dto.rs](../../services/auth/src/dtos/worker_dto.rs)

### WorkerOut (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| email | `String` | Ne |
| role | `String` | Ne |
| business_id | `Uuid` | Ne |

## services/auth/src/models/businesses.rs

Izvor: [services/auth/src/models/businesses.rs](../../services/auth/src/models/businesses.rs)

### Business (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| name | `String` | Ne |
| owner_id | `Uuid` | Ne |
| address | `String` | Ne |
| phone | `Option<String>` | Da |
| description | `Option<String>` | Da |
| website | `Option<String>` | Da |
| created_at | `DateTime<Utc>` | Ne |
| updated_at | `DateTime<Utc>` | Ne |

## services/auth/src/models/user.rs

Izvor: [services/auth/src/models/user.rs](../../services/auth/src/models/user.rs)

### User (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| email | `String` | Ne |
| password_hash | `String` | Ne |
| role | `Role` | Ne |
| business_id | `Option<Uuid>` | Da |
| first_name | `String` | Ne |
| last_name | `String` | Ne |
| address | `String` | Ne |
| phone | `Option<String>` | Da |
| photo_url | `Option<String>` | Da |
| date_of_birth | `Option<NaiveDate>` | Da |
| created_at | `DateTime<Utc>` | Ne |
| updated_at | `DateTime<Utc>` | Ne |

### UserRow (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| email | `String` | Ne |
| password_hash | `String` | Ne |
| business_id | `Option<Uuid>` | Da |
| role | `String` | Ne |
| first_name | `String` | Ne |
| last_name | `String` | Ne |
| address | `String` | Ne |
| phone | `Option<String>` | Da |
| photo_url | `Option<String>` | Da |
| date_of_birth | `Option<NaiveDate>` | Da |
| created_at | `DateTime<Utc>` | Ne |
| updated_at | `DateTime<Utc>` | Ne |

## services/orders/src/dtos/analytics/analytics_query.rs

Izvor: [services/orders/src/dtos/analytics/analytics_query.rs](../../services/orders/src/dtos/analytics/analytics_query.rs)

### AnalyticsQuery (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| from | `Option<String>` | Da |
| to | `Option<String>` | Da |

## services/orders/src/dtos/analytics/analytics_response.rs

Izvor: [services/orders/src/dtos/analytics/analytics_response.rs](../../services/orders/src/dtos/analytics/analytics_response.rs)

### AnalyticsResponse (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| total_revenue | `f64` | Ne |
| total_orders | `i64` | Ne |
| orders_by_status | `Vec<StatusCount>` | Ne |
| revenue_by_month | `Vec<MonthlyRevenue>` | Ne |
| top_products | `Vec<TopProduct>` | Ne |

### StatusCount (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| status | `String` | Ne |
| count | `i64` | Ne |

### MonthlyRevenue (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| month | `String` | Ne |
| revenue | `f64` | Ne |
| orders | `i64` | Ne |

### TopProduct (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| product_id | `Uuid` | Ne |
| product_name | `String` | Ne |
| total_sold | `f64` | Ne |
| total_revenue | `f64` | Ne |

## services/orders/src/dtos/order/create_order_request.rs

Izvor: [services/orders/src/dtos/order/create_order_request.rs](../../services/orders/src/dtos/order/create_order_request.rs)

### CreateOrderRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| customer_id | `Option<Uuid>` | Da |
| customer_name | `Option<String>` | Da |
| customer_email | `Option<String>` | Da |
| notes | `Option<String>` | Da |
| items | `Vec<OrderItemRequest>` | Ne |
| business_id | `Option<Uuid>` | Da |

## services/orders/src/dtos/order/list_orders_query.rs

Izvor: [services/orders/src/dtos/order/list_orders_query.rs](../../services/orders/src/dtos/order/list_orders_query.rs)

### ListOrdersQuery (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| business_id | `Option<Uuid>` | Da |
| page | `Option<i64>` | Da |
| limit | `Option<i64>` | Da |
| status | `Option<String>` | Da |
| search | `Option<String>` | Da |

## services/orders/src/dtos/order/order_response.rs

Izvor: [services/orders/src/dtos/order/order_response.rs](../../services/orders/src/dtos/order/order_response.rs)

### OrderResponse (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| business_id | `Uuid` | Ne |
| customer_id | `Option<Uuid>` | Da |
| customer_name | `Option<String>` | Da |
| customer_email | `Option<String>` | Da |
| status | `String` | Ne |
| total_price | `f64` | Ne |
| notes | `Option<String>` | Da |
| items | `Vec<OrderItemResponse>` | Ne |
| created_at | `NaiveDateTime` | Ne |
| updated_at | `NaiveDateTime` | Ne |

## services/orders/src/dtos/order/update_status_request.rs

Izvor: [services/orders/src/dtos/order/update_status_request.rs](../../services/orders/src/dtos/order/update_status_request.rs)

### UpdateStatusRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| status | `String` | Ne |

## services/orders/src/dtos/order_item/new_order_item_dto.rs

Izvor: [services/orders/src/dtos/order_item/new_order_item_dto.rs](../../services/orders/src/dtos/order_item/new_order_item_dto.rs)

### NewOrderItem (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| product_id | `Uuid` | Ne |
| product_name | `String` | Ne |
| product_type | `String` | Ne |
| unit_price | `BigDecimal` | Ne |
| quantity | `BigDecimal` | Ne |
| unit | `String` | Ne |
| business_id | `Uuid` | Ne |

## services/orders/src/dtos/order_item/order_item_request.rs

Izvor: [services/orders/src/dtos/order_item/order_item_request.rs](../../services/orders/src/dtos/order_item/order_item_request.rs)

### OrderItemRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| product_id | `Uuid` | Ne |
| quantity | `f64` | Ne |

## services/orders/src/dtos/order_item/order_item_response.rs

Izvor: [services/orders/src/dtos/order_item/order_item_response.rs](../../services/orders/src/dtos/order_item/order_item_response.rs)

### OrderItemResponse (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| product_id | `Uuid` | Ne |
| product_name | `String` | Ne |
| product_type | `String` | Ne |
| unit_price | `f64` | Ne |
| quantity | `f64` | Ne |
| unit | `String` | Ne |
| subtotal | `f64` | Ne |

## services/orders/src/dtos/product/product_api_data.rs

Izvor: [services/orders/src/dtos/product/product_api_data.rs](../../services/orders/src/dtos/product/product_api_data.rs)

### ProductApiData (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| name | `String` | Ne |
| product_type | `String` | Ne |
| price | `Decimal` | Ne |
| quantity | `Decimal` | Ne |
| unit | `String` | Ne |
| is_active | `bool` | Ne |
| business_id | `Option<Uuid>` | Da |

## services/orders/src/dtos/product/product_api_response.rs

Izvor: [services/orders/src/dtos/product/product_api_response.rs](../../services/orders/src/dtos/product/product_api_response.rs)

### ProductApiResponse (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| data | `ProductApiData` | Ne |

## services/orders/src/models/order.rs

Izvor: [services/orders/src/models/order.rs](../../services/orders/src/models/order.rs)

### Order (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| business_id | `Uuid` | Ne |
| customer_id | `Option<Uuid>` | Da |
| customer_name | `Option<String>` | Da |
| customer_email | `Option<String>` | Da |
| status | `String` | Ne |
| total_price | `BigDecimal` | Ne |
| notes | `Option<String>` | Da |
| is_deleted | `bool` | Ne |
| created_at | `NaiveDateTime` | Ne |
| updated_at | `NaiveDateTime` | Ne |

## services/orders/src/models/order_item.rs

Izvor: [services/orders/src/models/order_item.rs](../../services/orders/src/models/order_item.rs)

### OrderItem (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| order_id | `Uuid` | Ne |
| product_id | `Uuid` | Ne |
| product_name | `String` | Ne |
| product_type | `String` | Ne |
| unit_price | `BigDecimal` | Ne |
| quantity | `BigDecimal` | Ne |
| unit | `String` | Ne |
| subtotal | `BigDecimal` | Ne |

## services/orders/src/models/order_status.rs

Izvor: [services/orders/src/models/order_status.rs](../../services/orders/src/models/order_status.rs)

### OrderStatus (enum)

```rust
Pending,
    Confirmed,
    Shipped,
    Delivered,
    Cancelled,
```

## services/productions/src/dtos/create_process_step_request.rs

Izvor: [services/productions/src/dtos/create_process_step_request.rs](../../services/productions/src/dtos/create_process_step_request.rs)

### CreateProcessStepRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| step_order | `i32` | Ne |
| name | `String` | Ne |
| description | `Option<String>` | Da |
| duration_hours | `Option<f64>` | Da |
| temperature | `Option<f64>` | Da |

## services/productions/src/dtos/create_production_batch_request.rs

Izvor: [services/productions/src/dtos/create_production_batch_request.rs](../../services/productions/src/dtos/create_production_batch_request.rs)

### CreateProductionBatchRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| name | `String` | Ne |
| process_type | `String` | Ne |
| start_date | `Option<NaiveDate>` | Da |
| end_date | `Option<NaiveDate>` | Da |
| notes | `Option<String>` | Da |
| raw_materials | `Option<Vec<RawMaterialRequest>>` | Da |

## services/productions/src/dtos/process_step_response.rs

Izvor: [services/productions/src/dtos/process_step_response.rs](../../services/productions/src/dtos/process_step_response.rs)

### ProcessStepResponse (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| step_order | `i32` | Ne |
| name | `String` | Ne |
| description | `Option<String>` | Da |
| duration_hours | `Option<f64>` | Da |
| temperature | `Option<f64>` | Da |

## services/productions/src/dtos/production_batch_response.rs

Izvor: [services/productions/src/dtos/production_batch_response.rs](../../services/productions/src/dtos/production_batch_response.rs)

### ProductionBatchResponse (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| business_id | `Uuid` | Ne |
| name | `String` | Ne |
| process_type | `String` | Ne |
| start_date | `Option<String>` | Da |
| end_date | `Option<String>` | Da |
| output_name | `Option<String>` | Da |
| output_type | `Option<String>` | Da |
| output_unit | `Option<String>` | Da |
| output_quantity | `Option<f64>` | Da |
| output_expiry_date | `Option<chrono::NaiveDate>` | Da |
| status | `String` | Ne |
| notes | `Option<String>` | Da |
| steps | `Vec<ProcessStepResponse>` | Ne |
| raw_materials | `Vec<RawMaterialResponse>` | Ne |
| created_at | `NaiveDateTime` | Ne |
| updated_at | `NaiveDateTime` | Ne |

## services/productions/src/dtos/raw_material_api_data.rs

Izvor: [services/productions/src/dtos/raw_material_api_data.rs](../../services/productions/src/dtos/raw_material_api_data.rs)

### RawMaterialApiData (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| name | `String` | Ne |
| material_type | `String` | Ne |
| unit | `String` | Ne |
| origin | `Option<String>` | Da |
| harvest_date | `Option<chrono::NaiveDate>` | Da |
| received_date | `Option<chrono::NaiveDate>` | Da |
| expiry_date | `Option<chrono::NaiveDate>` | Da |
| supplier | `Option<String>` | Da |

### RawMaterialWrapper (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| data | `RawMaterialApiData` | Ne |

## services/productions/src/dtos/raw_material_request.rs

Izvor: [services/productions/src/dtos/raw_material_request.rs](../../services/productions/src/dtos/raw_material_request.rs)

### RawMaterialRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| raw_material_id | `Uuid` | Ne |
| quantity_used | `f64` | Ne |
| unit | `String` | Ne |

## services/productions/src/dtos/raw_material_response.rs

Izvor: [services/productions/src/dtos/raw_material_response.rs](../../services/productions/src/dtos/raw_material_response.rs)

### RawMaterialResponse (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| name | `String` | Ne |
| material_type | `String` | Ne |
| quantity_used | `f64` | Ne |
| unit | `String` | Ne |
| origin | `Option<String>` | Da |
| harvest_date | `Option<chrono::NaiveDate>` | Da |
| received_date | `Option<chrono::NaiveDate>` | Da |
| expiry_date | `Option<chrono::NaiveDate>` | Da |
| supplier | `Option<String>` | Da |

## services/productions/src/dtos/update_process_step_request.rs

Izvor: [services/productions/src/dtos/update_process_step_request.rs](../../services/productions/src/dtos/update_process_step_request.rs)

### UpdateProcessStepRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| step_order | `Option<i32>` | Da |
| name | `Option<String>` | Da |
| description | `Option<String>` | Da |
| duration_hours | `Option<f64>` | Da |
| temperature | `Option<f64>` | Da |

## services/productions/src/dtos/update_production_batch_request.rs

Izvor: [services/productions/src/dtos/update_production_batch_request.rs](../../services/productions/src/dtos/update_production_batch_request.rs)

### UpdateProductionBatchRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| name | `Option<String>` | Da |
| process_type | `Option<String>` | Da |
| start_date | `Option<NaiveDate>` | Da |
| end_date | `Option<NaiveDate>` | Da |
| notes | `Option<String>` | Da |
| output_name | `Option<String>` | Da |
| output_type | `Option<String>` | Da |
| output_unit | `Option<String>` | Da |
| output_quantity | `Option<f64>` | Da |
| output_expiry_date | `Option<chrono::NaiveDate>` | Da |
| status | `Option<String>` | Da |

## services/productions/src/models/batch_raw_material.rs

Izvor: [services/productions/src/models/batch_raw_material.rs](../../services/productions/src/models/batch_raw_material.rs)

### BatchRawMaterial (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| batch_id | `Uuid` | Ne |
| business_id | `Uuid` | Ne |
| raw_material_id | `Uuid` | Ne |
| raw_material_name | `String` | Ne |
| material_type | `String` | Ne |
| quantity_used | `BigDecimal` | Ne |
| unit | `String` | Ne |
| origin | `Option<String>` | Da |
| harvest_date | `Option<chrono::NaiveDate>` | Da |
| received_date | `Option<chrono::NaiveDate>` | Da |
| expiry_date | `Option<chrono::NaiveDate>` | Da |
| supplier | `Option<String>` | Da |

## services/productions/src/models/insert_production_params.rs

Izvor: [services/productions/src/models/insert_production_params.rs](../../services/productions/src/models/insert_production_params.rs)

### InsertProductionParams (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| business_id | `Uuid` | Ne |
| name | `String` | Ne |
| process_type | `String` | Ne |
| start_date | `Option<chrono::NaiveDate>` | Da |
| end_date | `Option<chrono::NaiveDate>` | Da |
| notes | `Option<String>` | Da |

## services/productions/src/models/insert_raw_material_params.rs

Izvor: [services/productions/src/models/insert_raw_material_params.rs](../../services/productions/src/models/insert_raw_material_params.rs)

### InsertRawMaterialParams (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| batch_id | `Uuid` | Ne |
| business_id | `Uuid` | Ne |
| raw_material_id | `Uuid` | Ne |
| raw_material_name | `String` | Ne |
| material_type | `String` | Ne |
| quantity_used | `BigDecimal` | Ne |
| unit | `String` | Ne |
| origin | `Option<String>` | Da |
| harvest_date | `Option<chrono::NaiveDate>` | Da |
| received_date | `Option<chrono::NaiveDate>` | Da |
| expiry_date | `Option<chrono::NaiveDate>` | Da |
| supplier | `Option<String>` | Da |

## services/productions/src/models/insert_step_params.rs

Izvor: [services/productions/src/models/insert_step_params.rs](../../services/productions/src/models/insert_step_params.rs)

### InsertStepParams (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| batch_id | `Uuid` | Ne |
| business_id | `Uuid` | Ne |
| step_order | `i32` | Ne |
| name | `String` | Ne |
| description | `Option<String>` | Da |
| duration_hours | `Option<BigDecimal>` | Da |
| temperature | `Option<BigDecimal>` | Da |

## services/productions/src/models/process_step.rs

Izvor: [services/productions/src/models/process_step.rs](../../services/productions/src/models/process_step.rs)

### ProcessStep (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| batch_id | `Uuid` | Ne |
| business_id | `Uuid` | Ne |
| step_order | `i32` | Ne |
| name | `String` | Ne |
| description | `Option<String>` | Da |
| duration_hours | `Option<BigDecimal>` | Da |
| temperature | `Option<BigDecimal>` | Da |
| created_at | `NaiveDateTime` | Ne |
| updated_at | `NaiveDateTime` | Ne |

## services/productions/src/models/production_batch.rs

Izvor: [services/productions/src/models/production_batch.rs](../../services/productions/src/models/production_batch.rs)

### ProductionBatch (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| business_id | `Uuid` | Ne |
| name | `String` | Ne |
| process_type | `String` | Ne |
| start_date | `Option<NaiveDate>` | Da |
| end_date | `Option<NaiveDate>` | Da |
| output_name | `Option<String>` | Da |
| output_type | `Option<String>` | Da |
| output_unit | `Option<String>` | Da |
| output_quantity | `Option<f64>` | Da |
| output_expiry_date | `Option<chrono::NaiveDate>` | Da |
| status | `String` | Ne |
| notes | `Option<String>` | Da |
| is_deleted | `bool` | Ne |
| created_at | `NaiveDateTime` | Ne |
| updated_at | `NaiveDateTime` | Ne |

## services/productions/src/models/query.rs

Izvor: [services/productions/src/models/query.rs](../../services/productions/src/models/query.rs)

### ListQuery (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| page | `Option<i64>` | Da |
| limit | `Option<i64>` | Da |
| status | `Option<String>` | Da |
| process_type | `Option<String>` | Da |
| search | `Option<String>` | Da |

## services/productions/src/models/update_production_params.rs

Izvor: [services/productions/src/models/update_production_params.rs](../../services/productions/src/models/update_production_params.rs)

### UpdateProductionParams (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| name | `String` | Ne |
| process_type | `String` | Ne |
| start_date | `Option<chrono::NaiveDate>` | Da |
| end_date | `Option<chrono::NaiveDate>` | Da |
| notes | `Option<String>` | Da |
| output_name | `Option<String>` | Da |
| output_type | `Option<String>` | Da |
| output_unit | `Option<String>` | Da |
| output_quantity | `Option<f64>` | Da |
| output_expiry_date | `Option<chrono::NaiveDate>` | Da |
| status | `String` | Ne |

## services/productions/src/models/update_step_params.rs

Izvor: [services/productions/src/models/update_step_params.rs](../../services/productions/src/models/update_step_params.rs)

### UpdateStepParams (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| step_order | `i32` | Ne |
| name | `String` | Ne |
| description | `Option<String>` | Da |
| duration_hours | `Option<BigDecimal>` | Da |
| temperature | `Option<BigDecimal>` | Da |

## services/products/src/dtos/clients.rs

Izvor: [services/products/src/dtos/clients.rs](../../services/products/src/dtos/clients.rs)

### BusinessResponse (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| data | `BusinessData` | Ne |

### BusinessData (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| name | `String` | Ne |

### BatchApiResponse (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| data | `BatchApiData` | Ne |

### BatchApiData (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| name | `String` | Ne |
| process_type | `String` | Ne |
| start_date | `Option<String>` | Da |
| end_date | `Option<String>` | Da |
| status | `String` | Ne |
| steps | `Vec<StepApiData>` | Ne |
| raw_materials | `Vec<RawMaterialApiData>` | Ne |

### StepApiData (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| step_order | `i32` | Ne |
| name | `String` | Ne |
| description | `Option<String>` | Da |
| duration_hours | `Option<f64>` | Da |
| temperature | `Option<f64>` | Da |

### RawMaterialApiData (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| name | `String` | Ne |
| material_type | `String` | Ne |
| quantity_used | `f64` | Ne |
| unit | `String` | Ne |
| origin | `Option<String>` | Da |
| harvest_date | `Option<chrono::NaiveDate>` | Da |
| received_date | `Option<String>` | Da |
| expiry_date | `Option<String>` | Da |
| supplier | `Option<String>` | Da |

## services/products/src/dtos/create_product_request.rs

Izvor: [services/products/src/dtos/create_product_request.rs](../../services/products/src/dtos/create_product_request.rs)

### CreateProductRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| name | `String` | Ne |
| product_type | `String` | Ne |
| description | `Option<String>` | Da |
| quantity | `f64` | Ne |
| unit | `String` | Ne |
| price | `f64` | Ne |
| expiry_date | `Option<chrono::NaiveDate>` | Da |
| batch_id | `Option<Uuid>` | Da |

## services/products/src/dtos/provenance_response.rs

Izvor: [services/products/src/dtos/provenance_response.rs](../../services/products/src/dtos/provenance_response.rs)

### ProvenanceResponse (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| product | `Product` | Ne |
| business_name | `Option<String>` | Da |
| batch | `Option<BatchRef>` | Da |

## services/products/src/dtos/update_product_request.rs

Izvor: [services/products/src/dtos/update_product_request.rs](../../services/products/src/dtos/update_product_request.rs)

### UpdateProductRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| name | `Option<String>` | Da |
| product_type | `Option<String>` | Da |
| description | `Option<String>` | Da |
| quantity | `Option<f64>` | Da |
| unit | `Option<String>` | Da |
| price | `Option<f64>` | Da |
| expiry_date | `Option<chrono::NaiveDate>` | Da |
| batch_id | `Option<Uuid>` | Da |
| is_active | `Option<bool>` | Da |

## services/products/src/models/batch_ref.rs

Izvor: [services/products/src/models/batch_ref.rs](../../services/products/src/models/batch_ref.rs)

### BatchRef (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| name | `String` | Ne |
| process_type | `String` | Ne |
| start_date | `Option<String>` | Da |
| end_date | `Option<String>` | Da |
| status | `String` | Ne |
| steps | `Vec<ProcessStepRef>` | Ne |
| raw_materials | `Vec<RawMaterialRef>` | Ne |

## services/products/src/models/process_step_ref.rs

Izvor: [services/products/src/models/process_step_ref.rs](../../services/products/src/models/process_step_ref.rs)

### ProcessStepRef (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| step_order | `i32` | Ne |
| name | `String` | Ne |
| description | `Option<String>` | Da |
| duration_hours | `Option<f64>` | Da |
| temperature | `Option<f64>` | Da |

## services/products/src/models/product.rs

Izvor: [services/products/src/models/product.rs](../../services/products/src/models/product.rs)

### Product (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| business_id | `Uuid` | Ne |
| name | `String` | Ne |
| product_type | `String` | Ne |
| description | `Option<String>` | Da |
| quantity | `BigDecimal` | Ne |
| unit | `String` | Ne |
| price | `BigDecimal` | Ne |
| expiry_date | `Option<chrono::NaiveDate>` | Da |
| batch_id | `Option<Uuid>` | Da |
| image_path | `Option<String>` | Da |
| qr_token | `Uuid` | Ne |
| qr_path | `Option<String>` | Da |
| is_active | `bool` | Ne |
| is_deleted | `bool` | Ne |
| created_at | `NaiveDateTime` | Ne |
| updated_at | `NaiveDateTime` | Ne |

## services/products/src/models/query.rs

Izvor: [services/products/src/models/query.rs](../../services/products/src/models/query.rs)

### ListQuery (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| page | `Option<i64>` | Da |
| limit | `Option<i64>` | Da |
| product_type | `Option<String>` | Da |
| search | `Option<String>` | Da |
| business_id | `Option<uuid::Uuid>` | Da |
| is_active | `Option<bool>` | Da |
| active_only | `Option<bool>` | Da |

## services/products/src/models/raw_material_ref.rs

Izvor: [services/products/src/models/raw_material_ref.rs](../../services/products/src/models/raw_material_ref.rs)

### RawMaterialRef (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| name | `String` | Ne |
| material_type | `String` | Ne |
| quantity_used | `f64` | Ne |
| unit | `String` | Ne |
| origin | `Option<String>` | Da |
| harvest_date | `Option<chrono::NaiveDate>` | Da |
| received_date | `Option<String>` | Da |
| expiry_date | `Option<String>` | Da |
| supplier | `Option<String>` | Da |

## services/products/src/models/update_product_params.rs

Izvor: [services/products/src/models/update_product_params.rs](../../services/products/src/models/update_product_params.rs)

### UpdateParams (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| name | `String` | Ne |
| product_type | `String` | Ne |
| description | `Option<String>` | Da |
| quantity | `BigDecimal` | Ne |
| unit | `String` | Ne |
| price | `BigDecimal` | Ne |
| expiry_date | `Option<chrono::NaiveDate>` | Da |
| batch_id | `Option<Uuid>` | Da |
| is_active | `bool` | Ne |

## services/raw-materials/src/dtos/adjust_quantity_request.rs

Izvor: [services/raw-materials/src/dtos/adjust_quantity_request.rs](../../services/raw-materials/src/dtos/adjust_quantity_request.rs)

### AdjustQuantityRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| delta | `f64` | Ne |
| reason | `Option<String>` | Da |

## services/raw-materials/src/dtos/create_raw_material_request.rs

Izvor: [services/raw-materials/src/dtos/create_raw_material_request.rs](../../services/raw-materials/src/dtos/create_raw_material_request.rs)

### CreateRawMaterialRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| name | `String` | Ne |
| material_type | `String` | Ne |
| quantity | `f64` | Ne |
| unit | `String` | Ne |
| supplier | `Option<String>` | Da |
| origin | `Option<String>` | Da |
| received_date | `Option<NaiveDate>` | Da |
| harvest_date | `Option<NaiveDate>` | Da |
| expiry_date | `Option<NaiveDate>` | Da |
| notes | `Option<String>` | Da |
| low_stock_threshold | `Option<f64>` | Da |

## services/raw-materials/src/dtos/update_raw_material_request.rs

Izvor: [services/raw-materials/src/dtos/update_raw_material_request.rs](../../services/raw-materials/src/dtos/update_raw_material_request.rs)

### UpdateRawMaterialRequest (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| name | `Option<String>` | Da |
| material_type | `Option<String>` | Da |
| quantity | `Option<f64>` | Da |
| unit | `Option<String>` | Da |
| supplier | `Option<String>` | Da |
| origin | `Option<String>` | Da |
| received_date | `Option<NaiveDate>` | Da |
| harvest_date | `Option<NaiveDate>` | Da |
| expiry_date | `Option<NaiveDate>` | Da |
| notes | `Option<String>` | Da |
| low_stock_threshold | `Option<f64>` | Da |

## services/raw-materials/src/handlers/consumption.rs

Izvor: [services/raw-materials/src/handlers/consumption.rs](../../services/raw-materials/src/handlers/consumption.rs)

### Consumption (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| operation_id | `Uuid` | Ne |
| raw_material_id | `Uuid` | Ne |
| quantity | `f64` | Ne |
| unit | `String` | Ne |

## services/raw-materials/src/models/query.rs

Izvor: [services/raw-materials/src/models/query.rs](../../services/raw-materials/src/models/query.rs)

### ListQuery (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| page | `Option<i64>` | Da |
| limit | `Option<i64>` | Da |
| material_type | `Option<String>` | Da |
| search | `Option<String>` | Da |

## services/raw-materials/src/models/raw_material.rs

Izvor: [services/raw-materials/src/models/raw_material.rs](../../services/raw-materials/src/models/raw_material.rs)

### RawMaterial (struct)

| Polje | Rust tip | Opciono |
|---|---|---|
| id | `Uuid` | Ne |
| business_id | `Uuid` | Ne |
| name | `String` | Ne |
| material_type | `String` | Ne |
| quantity | `BigDecimal` | Ne |
| unit | `String` | Ne |
| supplier | `Option<String>` | Da |
| origin | `Option<String>` | Da |
| received_date | `Option<NaiveDate>` | Da |
| harvest_date | `Option<NaiveDate>` | Da |
| expiry_date | `Option<NaiveDate>` | Da |
| notes | `Option<String>` | Da |
| low_stock_threshold | `Option<BigDecimal>` | Da |
| is_deleted | `bool` | Ne |
| created_at | `NaiveDateTime` | Ne |
| updated_at | `NaiveDateTime` | Ne |

## Frontend ugovori

Frontend TypeScript modeli nalaze se u sledećim izvorima. Njihove razlike u response wrapper-u opisane su u dokumentima 06 i 08.

- [frontend/local-bite-frontend/src/app/shared/models/api.models.ts](../../frontend/local-bite-frontend/src/app/shared/models/api.models.ts)
- [frontend/local-bite-frontend/src/app/shared/models/auth.models.ts](../../frontend/local-bite-frontend/src/app/shared/models/auth.models.ts)
- [frontend/local-bite-frontend/src/app/shared/models/order.models.ts](../../frontend/local-bite-frontend/src/app/shared/models/order.models.ts)
- [frontend/local-bite-frontend/src/app/shared/models/product.models.ts](../../frontend/local-bite-frontend/src/app/shared/models/product.models.ts)
- [frontend/local-bite-frontend/src/app/shared/models/production.models.ts](../../frontend/local-bite-frontend/src/app/shared/models/production.models.ts)
- [frontend/local-bite-frontend/src/app/shared/models/raw-material.models.ts](../../frontend/local-bite-frontend/src/app/shared/models/raw-material.models.ts)
