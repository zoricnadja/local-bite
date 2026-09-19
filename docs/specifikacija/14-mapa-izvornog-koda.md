# 14 — Mapa izvornog koda

[Sadržaj specifikacije](README.md)

Inventar programskih, konfiguracionih i dokumentacionih izvora u pregledanom radnom stablu. Biblioteke, build rezultati, privatna lokalna konfiguracija, generisani binarni dokumenti i IDE/agent metadata nisu poslovni izvor i nisu pojedinačno popisani. Postojanje helper-a ne znači da je aktivno pozvan. Opisi u ovom katalogu klasifikuju ulogu fajla; detaljno ponašanje je u tematskim poglavljima.

## Dopune recovery/reservation iteracije

| Fajl | Uloga |
|---|---|
| [libs/common/src/service_auth.rs](../../libs/common/src/service_auth.rs) | Kratkotrajni tehnički JWT za operaciju i farm scope |
| [services/orders/src/checkout.rs](../../services/orders/src/checkout.rs) | Trajna checkout namera, rezervacija, recovery i stock release |
| [services/products/src/reservations.rs](../../services/products/src/reservations.rs) | Atomska idempotentna rezervacija i vraćanje zalihe |
| [services/productions/src/material_recovery.rs](../../services/productions/src/material_recovery.rs) | Intent/release recovery sirovinske potrošnje |
| [services/products/src/dtos/public_provenance.rs](../../services/products/src/dtos/public_provenance.rs) | Javni allowlist DTO porekla |
| [services/products/src/output_consumer.rs](../../services/products/src/output_consumer.rs) | Sequence-safe output consumer i poison DLQ |
| [scripts/backup-verify.cjs](../../scripts/backup-verify.cjs) | Backup, checksum i izolovani restore drill |
| [scripts/benchmark.cjs](../../scripts/benchmark.cjs) | Read-only lokalni HTTP baseline |
| [scripts/verify-output-events.cjs](../../scripts/verify-output-events.cjs) | Stale/legacy/poison output regresija |
| [frontend/local-bite-frontend/src/app/core/services/decimal-wire.ts](../../frontend/local-bite-frontend/src/app/core/services/decimal-wire.ts) | NUMERIC string → prikazni broj |
| [frontend/local-bite-frontend/src/app/core/auth/authenticated-media.directive.ts](../../frontend/local-bite-frontend/src/app/core/auth/authenticated-media.directive.ts) | Autorizovano blob učitavanje medija |


## .gitattributes

| Fajl | Uloga |
|---|---|
| [.gitattributes](../../.gitattributes) | Konfiguracioni ili pomoćni projektni izvor. |

## .gitignore

| Fajl | Uloga |
|---|---|
| [.gitignore](../../.gitignore) | Konfiguracioni ili pomoćni projektni izvor. |

## .gitmodules

| Fajl | Uloga |
|---|---|
| [.gitmodules](../../.gitmodules) | Konfiguracioni ili pomoćni projektni izvor. |

## Cargo.lock

| Fajl | Uloga |
|---|---|
| [Cargo.lock](../../Cargo.lock) | Zaključane Rust zavisnosti radi ponovljivog build-a. |

## Cargo.toml

| Fajl | Uloga |
|---|---|
| [Cargo.toml](../../Cargo.toml) | Rust paket/workspace, feature-i i deklarisane zavisnosti. |

## README.md

| Fajl | Uloga |
|---|---|
| [README.md](../../README.md) | Postojeća dokumentacija; nivo ažurnosti proveriti prema kodu. |

## docker-compose.public.yml

| Fajl | Uloga |
|---|---|
| [docker-compose.public.yml](../../docker-compose.public.yml) | Compose topologija, okruženje, mreža i volumeni. |

## docker-compose.yml

| Fajl | Uloga |
|---|---|
| [docker-compose.yml](../../docker-compose.yml) | Compose topologija, okruženje, mreža i volumeni. |

## docs

| Fajl | Uloga |
|---|---|
| [docs/messaging-cqrs.md](../../docs/messaging-cqrs.md) | Postojeća dokumentacija; nivo ažurnosti proveriti prema kodu. |

## frontend/local-bite-frontend

| Fajl | Uloga |
|---|---|
| [frontend/local-bite-frontend/.gitignore](../../frontend/local-bite-frontend/.gitignore) | Konfiguracioni ili pomoćni projektni izvor. |
| [frontend/local-bite-frontend/.vscode/extensions.json](../../frontend/local-bite-frontend/.vscode/extensions.json) | Build, alatna ili TypeScript konfiguracija. |
| [frontend/local-bite-frontend/.vscode/launch.json](../../frontend/local-bite-frontend/.vscode/launch.json) | Build, alatna ili TypeScript konfiguracija. |
| [frontend/local-bite-frontend/.vscode/tasks.json](../../frontend/local-bite-frontend/.vscode/tasks.json) | Build, alatna ili TypeScript konfiguracija. |
| [frontend/local-bite-frontend/Dockerfile](../../frontend/local-bite-frontend/Dockerfile) | Build i runtime definicija kontejnera. |
| [frontend/local-bite-frontend/Dockerfile.public](../../frontend/local-bite-frontend/Dockerfile.public) | Build i runtime definicija kontejnera. |
| [frontend/local-bite-frontend/README.md](../../frontend/local-bite-frontend/README.md) | Postojeća dokumentacija; nivo ažurnosti proveriti prema kodu. |
| [frontend/local-bite-frontend/angular.json](../../frontend/local-bite-frontend/angular.json) | Build, alatna ili TypeScript konfiguracija. |
| [frontend/local-bite-frontend/package.json](../../frontend/local-bite-frontend/package.json) | Frontend zavisnosti i npm komande. |
| [frontend/local-bite-frontend/proxy.conf.json](../../frontend/local-bite-frontend/proxy.conf.json) | HTTP gateway/proxy mapiranje i pravila pristupa. |
| [frontend/local-bite-frontend/public-nginx.conf](../../frontend/local-bite-frontend/public-nginx.conf) | HTTP gateway/proxy mapiranje i pravila pristupa. |
| [frontend/local-bite-frontend/src/app/app.config.ts](../../frontend/local-bite-frontend/src/app/app.config.ts) | Frontend bootstrap, konfiguracija ili deljeni helper. Identifikatori: `appConfig`. |
| [frontend/local-bite-frontend/src/app/app.css](../../frontend/local-bite-frontend/src/app/app.css) | Globalni/korenski stilovi aplikacije. |
| [frontend/local-bite-frontend/src/app/app.html](../../frontend/local-bite-frontend/src/app/app.html) | HTML ulaz ili korenski template. |
| [frontend/local-bite-frontend/src/app/app.routes.ts](../../frontend/local-bite-frontend/src/app/app.routes.ts) | Mapiranje ruta na komponente ili HTTP handlere. Identifikatori: `routes`. |
| [frontend/local-bite-frontend/src/app/app.spec.ts](../../frontend/local-bite-frontend/src/app/app.spec.ts) | Frontend automatizovana provera ponašanja. |
| [frontend/local-bite-frontend/src/app/app.ts](../../frontend/local-bite-frontend/src/app/app.ts) | Frontend bootstrap, konfiguracija ili deljeni helper. Identifikatori: `AppComponent`. |
| [frontend/local-bite-frontend/src/app/core/auth/auth.guard.ts](../../frontend/local-bite-frontend/src/app/core/auth/auth.guard.ts) | Klijentska sesija, prava pristupa ili zaštita navigacije. Identifikatori: `authGuard`. |
| [frontend/local-bite-frontend/src/app/core/auth/auth.service.ts](../../frontend/local-bite-frontend/src/app/core/auth/auth.service.ts) | Klijentska sesija, prava pristupa ili zaštita navigacije. Identifikatori: `AuthService`. |
| [frontend/local-bite-frontend/src/app/core/auth/can.directive.ts](../../frontend/local-bite-frontend/src/app/core/auth/can.directive.ts) | Klijentska sesija, prava pristupa ili zaštita navigacije. Identifikatori: `CanDirective`. |
| [frontend/local-bite-frontend/src/app/core/auth/farm-session.spec.ts](../../frontend/local-bite-frontend/src/app/core/auth/farm-session.spec.ts) | Frontend automatizovana provera ponašanja. |
| [frontend/local-bite-frontend/src/app/core/auth/permissions.spec.ts](../../frontend/local-bite-frontend/src/app/core/auth/permissions.spec.ts) | Frontend automatizovana provera ponašanja. |
| [frontend/local-bite-frontend/src/app/core/auth/permissions.ts](../../frontend/local-bite-frontend/src/app/core/auth/permissions.ts) | Klijentska sesija, prava pristupa ili zaštita navigacije. Identifikatori: `PERMISSIONS`, `Permission`, `permissionGuard`. |
| [frontend/local-bite-frontend/src/app/core/auth/role.guard.ts](../../frontend/local-bite-frontend/src/app/core/auth/role.guard.ts) | Klijentska sesija, prava pristupa ili zaštita navigacije. Identifikatori: `roleGuard`. |
| [frontend/local-bite-frontend/src/app/core/interceptors/auth.interceptor.ts](../../frontend/local-bite-frontend/src/app/core/interceptors/auth.interceptor.ts) | Zajednička obrada izlaznog HTTP zahteva. Identifikatori: `authInterceptor`. |
| [frontend/local-bite-frontend/src/app/core/services/farm.service.ts](../../frontend/local-bite-frontend/src/app/core/services/farm.service.ts) | Angular API adapter za HTTP pozive. Identifikatori: `FarmService`. |
| [frontend/local-bite-frontend/src/app/core/services/orders.service.ts](../../frontend/local-bite-frontend/src/app/core/services/orders.service.ts) | Angular API adapter za HTTP pozive. Identifikatori: `OrdersService`. |
| [frontend/local-bite-frontend/src/app/core/services/producer.service.ts](../../frontend/local-bite-frontend/src/app/core/services/producer.service.ts) | Angular API adapter za HTTP pozive. Identifikatori: `Producer`, `ProducerService`. |
| [frontend/local-bite-frontend/src/app/core/services/product.service.ts](../../frontend/local-bite-frontend/src/app/core/services/product.service.ts) | Angular API adapter za HTTP pozive. Identifikatori: `ProductService`. |
| [frontend/local-bite-frontend/src/app/core/services/production.service.ts](../../frontend/local-bite-frontend/src/app/core/services/production.service.ts) | Angular API adapter za HTTP pozive. Identifikatori: `ProductionService`. |
| [frontend/local-bite-frontend/src/app/core/services/raw-materials.service.ts](../../frontend/local-bite-frontend/src/app/core/services/raw-materials.service.ts) | Angular API adapter za HTTP pozive. Identifikatori: `RawMaterialsService`. |
| [frontend/local-bite-frontend/src/app/core/services/users.service.ts](../../frontend/local-bite-frontend/src/app/core/services/users.service.ts) | Angular API adapter za HTTP pozive. Identifikatori: `UserService`. |
| [frontend/local-bite-frontend/src/app/features/auth/auth.routes.ts](../../frontend/local-bite-frontend/src/app/features/auth/auth.routes.ts) | Mapiranje ruta na komponente ili HTTP handlere. Identifikatori: `AUTH_ROUTES`. |
| [frontend/local-bite-frontend/src/app/features/auth/login/login.component.css](../../frontend/local-bite-frontend/src/app/features/auth/login/login.component.css) | Stilovi poslovnog ekrana. |
| [frontend/local-bite-frontend/src/app/features/auth/login/login.component.html](../../frontend/local-bite-frontend/src/app/features/auth/login/login.component.html) | Template poslovnog ekrana. |
| [frontend/local-bite-frontend/src/app/features/auth/login/login.component.ts](../../frontend/local-bite-frontend/src/app/features/auth/login/login.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `LoginComponent`. |
| [frontend/local-bite-frontend/src/app/features/auth/register/register.component.css](../../frontend/local-bite-frontend/src/app/features/auth/register/register.component.css) | Stilovi poslovnog ekrana. |
| [frontend/local-bite-frontend/src/app/features/auth/register/register.component.html](../../frontend/local-bite-frontend/src/app/features/auth/register/register.component.html) | Template poslovnog ekrana. |
| [frontend/local-bite-frontend/src/app/features/auth/register/register.component.ts](../../frontend/local-bite-frontend/src/app/features/auth/register/register.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `RegisterComponent`. |
| [frontend/local-bite-frontend/src/app/features/dashboard/dashboard.component.css](../../frontend/local-bite-frontend/src/app/features/dashboard/dashboard.component.css) | Stilovi poslovnog ekrana. |
| [frontend/local-bite-frontend/src/app/features/dashboard/dashboard.component.html](../../frontend/local-bite-frontend/src/app/features/dashboard/dashboard.component.html) | Template poslovnog ekrana. |
| [frontend/local-bite-frontend/src/app/features/dashboard/dashboard.component.ts](../../frontend/local-bite-frontend/src/app/features/dashboard/dashboard.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `DashboardComponent`. |
| [frontend/local-bite-frontend/src/app/features/farm/create/create-farm.component.css](../../frontend/local-bite-frontend/src/app/features/farm/create/create-farm.component.css) | Stilovi poslovnog ekrana. |
| [frontend/local-bite-frontend/src/app/features/farm/create/create-farm.component.html](../../frontend/local-bite-frontend/src/app/features/farm/create/create-farm.component.html) | Template poslovnog ekrana. |
| [frontend/local-bite-frontend/src/app/features/farm/create/create-farm.component.ts](../../frontend/local-bite-frontend/src/app/features/farm/create/create-farm.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `CreateFarmComponent`. |
| [frontend/local-bite-frontend/src/app/features/farm/farm.routes.ts](../../frontend/local-bite-frontend/src/app/features/farm/farm.routes.ts) | Mapiranje ruta na komponente ili HTTP handlere. Identifikatori: `FARM_ROUTES`. |
| [frontend/local-bite-frontend/src/app/features/farm/workers/add-worker.component.css](../../frontend/local-bite-frontend/src/app/features/farm/workers/add-worker.component.css) | Stilovi poslovnog ekrana. |
| [frontend/local-bite-frontend/src/app/features/farm/workers/add-worker.component.html](../../frontend/local-bite-frontend/src/app/features/farm/workers/add-worker.component.html) | Template poslovnog ekrana. |
| [frontend/local-bite-frontend/src/app/features/farm/workers/add-worker.component.ts](../../frontend/local-bite-frontend/src/app/features/farm/workers/add-worker.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `AddWorkerComponent`. |
| [frontend/local-bite-frontend/src/app/features/farm/workers/workers-list.component.ts](../../frontend/local-bite-frontend/src/app/features/farm/workers/workers-list.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `WorkersListComponent`. |
| [frontend/local-bite-frontend/src/app/features/orders/analytics/orders-analytics.component.ts](../../frontend/local-bite-frontend/src/app/features/orders/analytics/orders-analytics.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `OrdersAnalyticsComponent`. |
| [frontend/local-bite-frontend/src/app/features/orders/detail/order-detail.component.ts](../../frontend/local-bite-frontend/src/app/features/orders/detail/order-detail.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `OrderDetailComponent`. |
| [frontend/local-bite-frontend/src/app/features/orders/form/order-form.component.ts](../../frontend/local-bite-frontend/src/app/features/orders/form/order-form.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `OrderFormComponent`. |
| [frontend/local-bite-frontend/src/app/features/orders/list/orders-list.component.ts](../../frontend/local-bite-frontend/src/app/features/orders/list/orders-list.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `OrdersListComponent`. |
| [frontend/local-bite-frontend/src/app/features/orders/orders.routes.ts](../../frontend/local-bite-frontend/src/app/features/orders/orders.routes.ts) | Mapiranje ruta na komponente ili HTTP handlere. Identifikatori: `ORDERS_ROUTES`. |
| [frontend/local-bite-frontend/src/app/features/production/detail/production-detail.component.ts](../../frontend/local-bite-frontend/src/app/features/production/detail/production-detail.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `ProductionDetailComponent`. |
| [frontend/local-bite-frontend/src/app/features/production/form/production-form.component.ts](../../frontend/local-bite-frontend/src/app/features/production/form/production-form.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `ProductionFormComponent`. |
| [frontend/local-bite-frontend/src/app/features/production/list/production-list.component.ts](../../frontend/local-bite-frontend/src/app/features/production/list/production-list.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `ProductionListComponent`. |
| [frontend/local-bite-frontend/src/app/features/production/production.routes.ts](../../frontend/local-bite-frontend/src/app/features/production/production.routes.ts) | Mapiranje ruta na komponente ili HTTP handlere. Identifikatori: `PRODUCTION_ROUTES`. |
| [frontend/local-bite-frontend/src/app/features/products/detail/product-detail.component.ts](../../frontend/local-bite-frontend/src/app/features/products/detail/product-detail.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `ProductDetailComponent`. |
| [frontend/local-bite-frontend/src/app/features/products/form/product-form.component.ts](../../frontend/local-bite-frontend/src/app/features/products/form/product-form.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `ProductFormComponent`. |
| [frontend/local-bite-frontend/src/app/features/products/list/products-list.component.ts](../../frontend/local-bite-frontend/src/app/features/products/list/products-list.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `ProductsListComponent`. |
| [frontend/local-bite-frontend/src/app/features/products/products.routes.ts](../../frontend/local-bite-frontend/src/app/features/products/products.routes.ts) | Mapiranje ruta na komponente ili HTTP handlere. Identifikatori: `PRODUCTS_ROUTES`. |
| [frontend/local-bite-frontend/src/app/features/profile/profile.component.css](../../frontend/local-bite-frontend/src/app/features/profile/profile.component.css) | Stilovi poslovnog ekrana. |
| [frontend/local-bite-frontend/src/app/features/profile/profile.component.html](../../frontend/local-bite-frontend/src/app/features/profile/profile.component.html) | Template poslovnog ekrana. |
| [frontend/local-bite-frontend/src/app/features/profile/profile.component.spec.ts](../../frontend/local-bite-frontend/src/app/features/profile/profile.component.spec.ts) | Frontend automatizovana provera ponašanja. |
| [frontend/local-bite-frontend/src/app/features/profile/profile.component.ts](../../frontend/local-bite-frontend/src/app/features/profile/profile.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `ProfileComponent`. |
| [frontend/local-bite-frontend/src/app/features/public-trace/public-trace.component.ts](../../frontend/local-bite-frontend/src/app/features/public-trace/public-trace.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `PublicTraceComponent`. |
| [frontend/local-bite-frontend/src/app/features/raw-materials/form/raw-material-form.component.html](../../frontend/local-bite-frontend/src/app/features/raw-materials/form/raw-material-form.component.html) | Template poslovnog ekrana. |
| [frontend/local-bite-frontend/src/app/features/raw-materials/form/raw-material-form.component.ts](../../frontend/local-bite-frontend/src/app/features/raw-materials/form/raw-material-form.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `RawMaterialFormComponent`. |
| [frontend/local-bite-frontend/src/app/features/raw-materials/list/raw-materials-list.component.ts](../../frontend/local-bite-frontend/src/app/features/raw-materials/list/raw-materials-list.component.ts) | Komponenta poslovnog ekrana, stanje, akcije i eventualno inline template/stilovi. Identifikatori: `RawMaterialsListComponent`. |
| [frontend/local-bite-frontend/src/app/features/raw-materials/list/raw-materials.component.css](../../frontend/local-bite-frontend/src/app/features/raw-materials/list/raw-materials.component.css) | Stilovi poslovnog ekrana. |
| [frontend/local-bite-frontend/src/app/features/raw-materials/list/raw-materials.component.html](../../frontend/local-bite-frontend/src/app/features/raw-materials/list/raw-materials.component.html) | Template poslovnog ekrana. |
| [frontend/local-bite-frontend/src/app/features/raw-materials/raw-materials.routes.ts](../../frontend/local-bite-frontend/src/app/features/raw-materials/raw-materials.routes.ts) | Mapiranje ruta na komponente ili HTTP handlere. Identifikatori: `RAW_MATERIALS_ROUTES`. |
| [frontend/local-bite-frontend/src/app/infrastructure/material/material.imports.ts](../../frontend/local-bite-frontend/src/app/infrastructure/material/material.imports.ts) | Frontend bootstrap, konfiguracija ili deljeni helper. Identifikatori: `MATERIAL_IMPORTS`. |
| [frontend/local-bite-frontend/src/app/shared/field-errors.directive.spec.ts](../../frontend/local-bite-frontend/src/app/shared/field-errors.directive.spec.ts) | Frontend automatizovana provera ponašanja. |
| [frontend/local-bite-frontend/src/app/shared/field-errors.directive.ts](../../frontend/local-bite-frontend/src/app/shared/field-errors.directive.ts) | Frontend bootstrap, konfiguracija ili deljeni helper. Identifikatori: `FieldErrorsDirective`. |
| [frontend/local-bite-frontend/src/app/shared/form-validators.ts](../../frontend/local-bite-frontend/src/app/shared/form-validators.ts) | Frontend bootstrap, konfiguracija ili deljeni helper. Identifikatori: `requiredText`, `websiteUrl`, `dateOrder`. |
| [frontend/local-bite-frontend/src/app/shared/models/api.models.ts](../../frontend/local-bite-frontend/src/app/shared/models/api.models.ts) | Domenski, persistence ili frontend tip i povezane konstante. Identifikatori: `PaginatedResponse`, `ApiResponse`. |
| [frontend/local-bite-frontend/src/app/shared/models/auth.models.ts](../../frontend/local-bite-frontend/src/app/shared/models/auth.models.ts) | Domenski, persistence ili frontend tip i povezane konstante. Identifikatori: `User`, `Farm`, `LoginRequest`, `LoginResponse`, `RegisterRequest`, `UpdateUserRequest`, `AddWorkerRequest`, `WorkerOut`, `UpdateFarmRequest`, `CreateFarmRequest`, `CreateFarmResult`. |
| [frontend/local-bite-frontend/src/app/shared/models/order.models.ts](../../frontend/local-bite-frontend/src/app/shared/models/order.models.ts) | Domenski, persistence ili frontend tip i povezane konstante. Identifikatori: `OrderStatus`, `OrderItem`, `Order`, `CreatesOrdersResponse`, `CreateOrderRequest`, `UpdateStatusRequest`, `OrderListQuery`, `StatusCount`, `MonthlyRevenue`, `TopProduct`, `AnalyticsResponse`, `ORDER_STATUS_TRANSITIONS`. |
| [frontend/local-bite-frontend/src/app/shared/models/product.models.ts](../../frontend/local-bite-frontend/src/app/shared/models/product.models.ts) | Domenski, persistence ili frontend tip i povezane konstante. Identifikatori: `Product`, `ProvenanceStep`, `ProvenanceMaterial`, `ProvenanceBatch`, `ProvenanceResponse`, `CreateProductRequest`, `UpdateProductRequest`, `ProductListQuery`. |
| [frontend/local-bite-frontend/src/app/shared/models/production.models.ts](../../frontend/local-bite-frontend/src/app/shared/models/production.models.ts) | Domenski, persistence ili frontend tip i povezane konstante. Identifikatori: `BatchStatus`, `ProcessStep`, `BatchRawMaterial`, `ProductionBatch`, `CreateBatchRequest`, `UpdateBatchRequest`, `CreateStepRequest`, `UpdateStepRequest`, `AddRawMaterialRequest`, `BatchListQuery`, `BATCH_STATUS_TRANSITIONS`. |
| [frontend/local-bite-frontend/src/app/shared/models/raw-material.models.ts](../../frontend/local-bite-frontend/src/app/shared/models/raw-material.models.ts) | Domenski, persistence ili frontend tip i povezane konstante. Identifikatori: `RawMaterial`, `RawMaterialRequest`, `AdjustQuantityRequest`, `RawMaterialListQuery`. |
| [frontend/local-bite-frontend/src/index.html](../../frontend/local-bite-frontend/src/index.html) | HTML ulaz ili korenski template. |
| [frontend/local-bite-frontend/src/main.ts](../../frontend/local-bite-frontend/src/main.ts) | Frontend bootstrap, konfiguracija ili deljeni helper. |
| [frontend/local-bite-frontend/src/styles.css](../../frontend/local-bite-frontend/src/styles.css) | Globalni/korenski stilovi aplikacije. |
| [frontend/local-bite-frontend/tsconfig.app.json](../../frontend/local-bite-frontend/tsconfig.app.json) | Build, alatna ili TypeScript konfiguracija. |
| [frontend/local-bite-frontend/tsconfig.json](../../frontend/local-bite-frontend/tsconfig.json) | Build, alatna ili TypeScript konfiguracija. |
| [frontend/local-bite-frontend/tsconfig.spec.json](../../frontend/local-bite-frontend/tsconfig.spec.json) | Build, alatna ili TypeScript konfiguracija. |
| [frontend/package-lock.json](../../frontend/package-lock.json) | Zaključane npm zavisnosti. |

## libs

| Fajl | Uloga |
|---|---|
| [libs/common/Cargo.toml](../../libs/common/Cargo.toml) | Rust paket/workspace, feature-i i deklarisane zavisnosti. |
| [libs/common/src/errors.rs](../../libs/common/src/errors.rs) | Zajednički backend infrastrukturni ugovor/helper; videti dokument 04. Identifikatori: `into_response`, `from`. |
| [libs/common/src/events.rs](../../libs/common/src/events.rs) | Zajednički backend infrastrukturni ugovor/helper; videti dokument 04. Identifikatori: `connect`, `topology`, `start_outbox`, `relay`. |
| [libs/common/src/jwt.rs](../../libs/common/src/jwt.rs) | Zajednički backend infrastrukturni ugovor/helper; videti dokument 04. Identifikatori: `encode_jwt`, `decode_jwt`. |
| [libs/common/src/lib.rs](../../libs/common/src/lib.rs) | Zajednički backend infrastrukturni ugovor/helper; videti dokument 04. |
| [libs/common/src/middleware.rs](../../libs/common/src/middleware.rs) | Zajednički backend infrastrukturni ugovor/helper; videti dokument 04. Identifikatori: `from_request_parts`, `require_farm`, `require_role`. |
| [libs/common/src/models.rs](../../libs/common/src/models.rs) | Zajednički backend infrastrukturni ugovor/helper; videti dokument 04. Identifikatori: `as_str`, `from`, `from_str`. |
| [libs/common/src/paginated_response.rs](../../libs/common/src/paginated_response.rs) | Zajednički backend infrastrukturni ugovor/helper; videti dokument 04. |
| [libs/common/src/response.rs](../../libs/common/src/response.rs) | Zajednički backend infrastrukturni ugovor/helper; videti dokument 04. Identifikatori: `no_content`. |

## nginx

| Fajl | Uloga |
|---|---|
| [nginx/nginx.conf](../../nginx/nginx.conf) | HTTP gateway/proxy mapiranje i pravila pristupa. |

## scripts

| Fajl | Uloga |
|---|---|
| [scripts/Replay-Events.ps1](../../scripts/Replay-Events.ps1) | Operativna PowerShell procedura za javni pristup ili replay. |
| [scripts/Start-PublicQr.ps1](../../scripts/Start-PublicQr.ps1) | Operativna PowerShell procedura za javni pristup ili replay. |
| [scripts/verify-cqrs.cjs](../../scripts/verify-cqrs.cjs) | Integraciona regresiona skripta sa test podacima i Docker/SQL proverama. |
| [scripts/verify-farm-trace.cjs](../../scripts/verify-farm-trace.cjs) | Integraciona regresiona skripta sa test podacima i Docker/SQL proverama. |
| [scripts/verify-role-stock.cjs](../../scripts/verify-role-stock.cjs) | Integraciona regresiona skripta sa test podacima i Docker/SQL proverama. |

## services/auth

| Fajl | Uloga |
|---|---|
| [services/auth/.sqlx/query-b43ec7e5c30e033e87352c0e68129bdd3d1bb32d390515eba6ae40975e8a66c4.json](../../services/auth/.sqlx/query-b43ec7e5c30e033e87352c0e68129bdd3d1bb32d390515eba6ae40975e8a66c4.json) | Build, alatna ili TypeScript konfiguracija. |
| [services/auth/.sqlx/query-de06c47092b5c792f98b12fa4f6cfb6bfca0411ae6cbed1b057b1660abb98362.json](../../services/auth/.sqlx/query-de06c47092b5c792f98b12fa4f6cfb6bfca0411ae6cbed1b057b1660abb98362.json) | Build, alatna ili TypeScript konfiguracija. |
| [services/auth/Cargo.toml](../../services/auth/Cargo.toml) | Rust paket/workspace, feature-i i deklarisane zavisnosti. |
| [services/auth/Dockerfile](../../services/auth/Dockerfile) | Build i runtime definicija kontejnera. |
| [services/auth/migrations/0001_init.sql](../../services/auth/migrations/0001_init.sql) | Migracija šeme, ograničenja ili integracionog outbox-a vlasničke baze. |
| [services/auth/migrations/20260917000000_transactional_outbox.sql](../../services/auth/migrations/20260917000000_transactional_outbox.sql) | Migracija šeme, ograničenja ili integracionog outbox-a vlasničke baze. |
| [services/auth/src/db.rs](../../services/auth/src/db.rs) | Konekcioni pool i automatsko izvršavanje migracija. Identifikatori: `create_pool`. |
| [services/auth/src/dtos/create_farm_request.rs](../../services/auth/src/dtos/create_farm_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/auth/src/dtos/create_farm_response.rs](../../services/auth/src/dtos/create_farm_response.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/auth/src/dtos/login_request.rs](../../services/auth/src/dtos/login_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/auth/src/dtos/login_response.rs](../../services/auth/src/dtos/login_response.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/auth/src/dtos/mod.rs](../../services/auth/src/dtos/mod.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/auth/src/dtos/register_request.rs](../../services/auth/src/dtos/register_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/auth/src/dtos/update_farm_request.rs](../../services/auth/src/dtos/update_farm_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/auth/src/dtos/update_user_request.rs](../../services/auth/src/dtos/update_user_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/auth/src/dtos/worker_dto.rs](../../services/auth/src/dtos/worker_dto.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/auth/src/handlers/auth.rs](../../services/auth/src/handlers/auth.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. Identifikatori: `register`, `login`. |
| [services/auth/src/handlers/farms.rs](../../services/auth/src/handlers/farms.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. Identifikatori: `trace_farm`, `create_farm`, `add_worker`, `get_farm`, `list_workers`, `list_farms`, `update_farm`, `delete_farm`. |
| [services/auth/src/handlers/mod.rs](../../services/auth/src/handlers/mod.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. |
| [services/auth/src/handlers/users.rs](../../services/auth/src/handlers/users.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. Identifikatori: `me`, `list_users`, `update_user`, `delete_user`. |
| [services/auth/src/main.rs](../../services/auth/src/main.rs) | Pokretanje procesa, konfiguracija, DI, rute i pozadinski taskovi. Identifikatori: `main`. |
| [services/auth/src/middleware/auth_middleware.rs](../../services/auth/src/middleware/auth_middleware.rs) | Konfiguracioni ili pomoćni projektni izvor. Identifikatori: `auth_middleware`. |
| [services/auth/src/middleware/mod.rs](../../services/auth/src/middleware/mod.rs) | Deklaracija i vidljivost Rust podmodula. |
| [services/auth/src/models/farms.rs](../../services/auth/src/models/farms.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/auth/src/models/mod.rs](../../services/auth/src/models/mod.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/auth/src/models/user.rs](../../services/auth/src/models/user.rs) | Domenski, persistence ili frontend tip i povezane konstante. Identifikatori: `from`. |
| [services/auth/src/repository/farm_repository.rs](../../services/auth/src/repository/farm_repository.rs) | SQL pristup vlasničkim tabelama i persistence operacije. Identifikatori: `new`, `insert_farm`, `email_exists`, `find_by_id`, `find_by_owner`, `find_all`, `update_farm`, `delete_farm`, `list_workers_by_farm`. |
| [services/auth/src/repository/mod.rs](../../services/auth/src/repository/mod.rs) | SQL pristup vlasničkim tabelama i persistence operacije. |
| [services/auth/src/repository/repository.rs](../../services/auth/src/repository/repository.rs) | SQL pristup vlasničkim tabelama i persistence operacije. Identifikatori: `new`, `create_user`, `find_by_id`, `find_by_email`, `find_all`, `update_user`, `delete_user`, `set_farm_id`, `clear_farm_id`. |
| [services/auth/src/service/farm_service.rs](../../services/auth/src/service/farm_service.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. Identifikatori: `new`, `create_farm`, `add_worker`, `get_farm`, `list_farms`, `update_farm`, `delete_farm`, `list_workers`, `require_admin`. |
| [services/auth/src/service/mod.rs](../../services/auth/src/service/mod.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. |
| [services/auth/src/service/service.rs](../../services/auth/src/service/service.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. Identifikatori: `new`, `register_user`, `issue_token`, `login`, `get_user`, `verify_token`. |
| [services/auth/src/service/user_service.rs](../../services/auth/src/service/user_service.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. Identifikatori: `new`, `get_user`, `list_users`, `update_user`, `delete_user`, `require_admin`. |

## services/orders

| Fajl | Uloga |
|---|---|
| [services/orders/Cargo.toml](../../services/orders/Cargo.toml) | Rust paket/workspace, feature-i i deklarisane zavisnosti. |
| [services/orders/Dockerfile](../../services/orders/Dockerfile) | Build i runtime definicija kontejnera. |
| [services/orders/migrations/0002_initial.sql](../../services/orders/migrations/0002_initial.sql) | Migracija šeme, ograničenja ili integracionog outbox-a vlasničke baze. |
| [services/orders/migrations/20260917000000_transactional_outbox.sql](../../services/orders/migrations/20260917000000_transactional_outbox.sql) | Migracija šeme, ograničenja ili integracionog outbox-a vlasničke baze. |
| [services/orders/src/db.rs](../../services/orders/src/db.rs) | Konekcioni pool i automatsko izvršavanje migracija. Identifikatori: `create_pool`. |
| [services/orders/src/dtos/analytics/analytics_query.rs](../../services/orders/src/dtos/analytics/analytics_query.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/orders/src/dtos/analytics/analytics_response.rs](../../services/orders/src/dtos/analytics/analytics_response.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/orders/src/dtos/analytics/mod.rs](../../services/orders/src/dtos/analytics/mod.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/orders/src/dtos/mod.rs](../../services/orders/src/dtos/mod.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/orders/src/dtos/order/create_order_request.rs](../../services/orders/src/dtos/order/create_order_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/orders/src/dtos/order/list_orders_query.rs](../../services/orders/src/dtos/order/list_orders_query.rs) | Ulazni/izlazni ili međuservisni tip ugovora. Identifikatori: `offset`, `limit`. |
| [services/orders/src/dtos/order/mod.rs](../../services/orders/src/dtos/order/mod.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/orders/src/dtos/order/order_response.rs](../../services/orders/src/dtos/order/order_response.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/orders/src/dtos/order/update_status_request.rs](../../services/orders/src/dtos/order/update_status_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/orders/src/dtos/order_item/mod.rs](../../services/orders/src/dtos/order_item/mod.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/orders/src/dtos/order_item/new_order_item_dto.rs](../../services/orders/src/dtos/order_item/new_order_item_dto.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/orders/src/dtos/order_item/order_item_request.rs](../../services/orders/src/dtos/order_item/order_item_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/orders/src/dtos/order_item/order_item_response.rs](../../services/orders/src/dtos/order_item/order_item_response.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/orders/src/dtos/product/mod.rs](../../services/orders/src/dtos/product/mod.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/orders/src/dtos/product/product_api_data.rs](../../services/orders/src/dtos/product/product_api_data.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/orders/src/dtos/product/product_api_response.rs](../../services/orders/src/dtos/product/product_api_response.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/orders/src/handlers/mod.rs](../../services/orders/src/handlers/mod.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. |
| [services/orders/src/handlers/orders.rs](../../services/orders/src/handlers/orders.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. Identifikatori: `list`, `get_orders_by_user`, `create`, `get_one`, `update_status`, `delete`, `analytics`, `extract_token`. |
| [services/orders/src/main.rs](../../services/orders/src/main.rs) | Pokretanje procesa, konfiguracija, DI, rute i pozadinski taskovi. Identifikatori: `main`. |
| [services/orders/src/models/mod.rs](../../services/orders/src/models/mod.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/orders/src/models/order.rs](../../services/orders/src/models/order.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/orders/src/models/order_item.rs](../../services/orders/src/models/order_item.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/orders/src/models/order_status.rs](../../services/orders/src/models/order_status.rs) | Domenski, persistence ili frontend tip i povezane konstante. Identifikatori: `from_str`, `as_str`, `allowed_transitions`, `can_transition_to`. |
| [services/orders/src/repositories/mod.rs](../../services/orders/src/repositories/mod.rs) | SQL pristup vlasničkim tabelama i persistence operacije. |
| [services/orders/src/repositories/order_item_repository.rs](../../services/orders/src/repositories/order_item_repository.rs) | SQL pristup vlasničkim tabelama i persistence operacije. Identifikatori: `new`, `find_by_order_id`, `insert_batch`, `top_products`. |
| [services/orders/src/repositories/order_repository.rs](../../services/orders/src/repositories/order_repository.rs) | SQL pristup vlasničkim tabelama i persistence operacije. Identifikatori: `new`, `find_by_id`, `find_all`, `find_all_by_user_id`, `count_by_user`, `count`, `insert`, `update_status`, `soft_delete`, `total_revenue`, `total_orders`, `orders_by_status`, `revenue_by_month`, `bigdecimal_to_f64`. |
| [services/orders/src/routes.rs](../../services/orders/src/routes.rs) | Mapiranje ruta na komponente ili HTTP handlere. Identifikatori: `order_routes`. |
| [services/orders/src/services/mod.rs](../../services/orders/src/services/mod.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. |
| [services/orders/src/services/order_service.rs](../../services/orders/src/services/order_service.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. Identifikatori: `new`, `list_orders`, `find_all_by_user_id`, `get_order`, `create_order`, `update_status`, `cancel_order`, `delete_order`, `get_analytics`, `map_order_response`. |
| [services/orders/src/services/product_service.rs](../../services/orders/src/services/product_service.rs) | HTTP adapter za čitanje proizvoda. Identifikatori: `base_url`, `fetch_product`. |

## services/productions

| Fajl | Uloga |
|---|---|
| [services/productions/Cargo.toml](../../services/productions/Cargo.toml) | Rust paket/workspace, feature-i i deklarisane zavisnosti. |
| [services/productions/Dockerfile](../../services/productions/Dockerfile) | Build i runtime definicija kontejnera. |
| [services/productions/migrations/0001_initial.sql](../../services/productions/migrations/0001_initial.sql) | Migracija šeme, ograničenja ili integracionog outbox-a vlasničke baze. |
| [services/productions/migrations/20260916000000_material_dates.sql](../../services/productions/migrations/20260916000000_material_dates.sql) | Migracija šeme, ograničenja ili integracionog outbox-a vlasničke baze. |
| [services/productions/migrations/20260917000000_transactional_outbox.sql](../../services/productions/migrations/20260917000000_transactional_outbox.sql) | Migracija šeme, ograničenja ili integracionog outbox-a vlasničke baze. |
| [services/productions/migrations/20260918000000_production_output.sql](../../services/productions/migrations/20260918000000_production_output.sql) | Migracija šeme, ograničenja ili integracionog outbox-a vlasničke baze. |
| [services/productions/src/db.rs](../../services/productions/src/db.rs) | Konekcioni pool i automatsko izvršavanje migracija. Identifikatori: `create_pool`. |
| [services/productions/src/dtos/create_process_step_request.rs](../../services/productions/src/dtos/create_process_step_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/productions/src/dtos/create_production_batch_request.rs](../../services/productions/src/dtos/create_production_batch_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/productions/src/dtos/mod.rs](../../services/productions/src/dtos/mod.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/productions/src/dtos/process_step_response.rs](../../services/productions/src/dtos/process_step_response.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/productions/src/dtos/production_batch_response.rs](../../services/productions/src/dtos/production_batch_response.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/productions/src/dtos/raw_material_api_data.rs](../../services/productions/src/dtos/raw_material_api_data.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/productions/src/dtos/raw_material_request.rs](../../services/productions/src/dtos/raw_material_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/productions/src/dtos/raw_material_response.rs](../../services/productions/src/dtos/raw_material_response.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/productions/src/dtos/update_process_step_request.rs](../../services/productions/src/dtos/update_process_step_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/productions/src/dtos/update_production_batch_request.rs](../../services/productions/src/dtos/update_production_batch_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/productions/src/handlers/batches.rs](../../services/productions/src/handlers/batches.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. Identifikatori: `list`, `create`, `get_one`, `trace`, `update`, `delete`, `extract_token`. |
| [services/productions/src/handlers/materials.rs](../../services/productions/src/handlers/materials.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. Identifikatori: `add_material`, `remove_material`. |
| [services/productions/src/handlers/mod.rs](../../services/productions/src/handlers/mod.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. |
| [services/productions/src/handlers/steps.rs](../../services/productions/src/handlers/steps.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. Identifikatori: `list_steps`, `add_step`, `update_step`, `delete_step`. |
| [services/productions/src/main.rs](../../services/productions/src/main.rs) | Pokretanje procesa, konfiguracija, DI, rute i pozadinski taskovi. Identifikatori: `main`. |
| [services/productions/src/models/batch_raw_material.rs](../../services/productions/src/models/batch_raw_material.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/productions/src/models/insert_production_params.rs](../../services/productions/src/models/insert_production_params.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/productions/src/models/insert_raw_material_params.rs](../../services/productions/src/models/insert_raw_material_params.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/productions/src/models/insert_step_params.rs](../../services/productions/src/models/insert_step_params.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/productions/src/models/mod.rs](../../services/productions/src/models/mod.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/productions/src/models/process_step.rs](../../services/productions/src/models/process_step.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/productions/src/models/production_batch.rs](../../services/productions/src/models/production_batch.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/productions/src/models/query.rs](../../services/productions/src/models/query.rs) | Domenski, persistence ili frontend tip i povezane konstante. Identifikatori: `offset`, `limit`. |
| [services/productions/src/models/update_production_params.rs](../../services/productions/src/models/update_production_params.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/productions/src/models/update_step_params.rs](../../services/productions/src/models/update_step_params.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/productions/src/repositories/batch_repository.rs](../../services/productions/src/repositories/batch_repository.rs) | SQL pristup vlasničkim tabelama i persistence operacije. Identifikatori: `new`, `list`, `find_by_id_and_farm`, `insert_in`, `update`, `soft_delete`. |
| [services/productions/src/repositories/mod.rs](../../services/productions/src/repositories/mod.rs) | SQL pristup vlasničkim tabelama i persistence operacije. |
| [services/productions/src/repositories/raw_materials_repository.rs](../../services/productions/src/repositories/raw_materials_repository.rs) | SQL pristup vlasničkim tabelama i persistence operacije. Identifikatori: `new`, `find_by_batch`, `exists`, `insert_in`, `delete`. |
| [services/productions/src/repositories/step_repository.rs](../../services/productions/src/repositories/step_repository.rs) | SQL pristup vlasničkim tabelama i persistence operacije. Identifikatori: `new`, `find_by_batch`, `find_by_id_and_batch`, `order_exists`, `insert`, `update`, `delete`. |
| [services/productions/src/routes.rs](../../services/productions/src/routes.rs) | Mapiranje ruta na komponente ili HTTP handlere. Identifikatori: `production_routes`. |
| [services/productions/src/services/batch_service.rs](../../services/productions/src/services/batch_service.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. Identifikatori: `dec`, `new`, `list`, `create`, `get_one`, `update`, `delete`, `assemble_detail`, `validate_status_transition`, `step_to_response`, `material_to_response`. |
| [services/productions/src/services/mod.rs](../../services/productions/src/services/mod.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. |
| [services/productions/src/services/raw_materials_service.rs](../../services/productions/src/services/raw_materials_service.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. Identifikatori: `dec`, `new`, `add`, `remove`, `consume`, `compensate`, `stock_request`, `fetch_raw_material`. |
| [services/productions/src/services/step_service.rs](../../services/productions/src/services/step_service.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. Identifikatori: `dec`, `new`, `list`, `add`, `update`, `delete`. |

## services/products

| Fajl | Uloga |
|---|---|
| [services/products/Cargo.toml](../../services/products/Cargo.toml) | Rust paket/workspace, feature-i i deklarisane zavisnosti. |
| [services/products/Dockerfile](../../services/products/Dockerfile) | Build i runtime definicija kontejnera. |
| [services/products/README.md](../../services/products/README.md) | Postojeća dokumentacija; nivo ažurnosti proveriti prema kodu. |
| [services/products/assets/certificate.py](../../services/products/assets/certificate.py) | Python/ReportLab PDF renderer proizvoda. |
| [services/products/migrations/20240101000000_init.sql](../../services/products/migrations/20240101000000_init.sql) | Migracija šeme, ograničenja ili integracionog outbox-a vlasničke baze. |
| [services/products/migrations/20260916000000_product_expiry.sql](../../services/products/migrations/20260916000000_product_expiry.sql) | Migracija šeme, ograničenja ili integracionog outbox-a vlasničke baze. |
| [services/products/migrations/20260917000000_transactional_outbox.sql](../../services/products/migrations/20260917000000_transactional_outbox.sql) | Migracija šeme, ograničenja ili integracionog outbox-a vlasničke baze. |
| [services/products/migrations/20260918000000_storage.sql](../../services/products/migrations/20260918000000_storage.sql) | Migracija šeme, ograničenja ili integracionog outbox-a vlasničke baze. |
| [services/products/src/db.rs](../../services/products/src/db.rs) | Konekcioni pool i automatsko izvršavanje migracija. Identifikatori: `create_pool`. |
| [services/products/src/dtos/clients.rs](../../services/products/src/dtos/clients.rs) | Ulazni/izlazni ili međuservisni tip ugovora. Identifikatori: `fetch_farm_name`, `fetch_batch`. |
| [services/products/src/dtos/create_product_request.rs](../../services/products/src/dtos/create_product_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/products/src/dtos/mod.rs](../../services/products/src/dtos/mod.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/products/src/dtos/provenance_response.rs](../../services/products/src/dtos/provenance_response.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/products/src/dtos/update_product_request.rs](../../services/products/src/dtos/update_product_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/products/src/handlers/image_handler.rs](../../services/products/src/handlers/image_handler.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. Identifikatori: `upload_image`, `get_image`, `serve_file`. |
| [services/products/src/handlers/mod.rs](../../services/products/src/handlers/mod.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. |
| [services/products/src/handlers/products_handler.rs](../../services/products/src/handlers/products_handler.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. Identifikatori: `list_by_farm`, `list`, `create`, `get_one`, `provenance`, `update`, `delete`. |
| [services/products/src/handlers/public_handler.rs](../../services/products/src/handlers/public_handler.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. Identifikatori: `scan`, `certificate`. |
| [services/products/src/handlers/qr_handler.rs](../../services/products/src/handlers/qr_handler.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. Identifikatori: `get_qr`, `regenerate_qr`, `serve_png_file`. |
| [services/products/src/main.rs](../../services/products/src/main.rs) | Pokretanje procesa, konfiguracija, DI, rute i pozadinski taskovi. Identifikatori: `main`. |
| [services/products/src/models/batch_ref.rs](../../services/products/src/models/batch_ref.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/products/src/models/mod.rs](../../services/products/src/models/mod.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/products/src/models/process_step_ref.rs](../../services/products/src/models/process_step_ref.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/products/src/models/product.rs](../../services/products/src/models/product.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/products/src/models/query.rs](../../services/products/src/models/query.rs) | Domenski, persistence ili frontend tip i povezane konstante. Identifikatori: `offset`, `limit`. |
| [services/products/src/models/raw_material_ref.rs](../../services/products/src/models/raw_material_ref.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/products/src/models/update_product_params.rs](../../services/products/src/models/update_product_params.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/products/src/output_consumer.rs](../../services/products/src/output_consumer.rs) | AMQP potrošač serija i idempotentno kreiranje proizvodnog izlaza. Identifikatori: `start`, `apply`, `consume`. |
| [services/products/src/repositories/mod.rs](../../services/products/src/repositories/mod.rs) | SQL pristup vlasničkim tabelama i persistence operacije. |
| [services/products/src/repositories/product_repository.rs](../../services/products/src/repositories/product_repository.rs) | SQL pristup vlasničkim tabelama i persistence operacije. Identifikatori: `new`, `find_all_by_farm_id`, `find_all`, `list_filtered`, `find_by_id_and_farm`, `find_by_id`, `find_by_qr_token`, `insert`, `update`, `soft_delete`, `set_qr_path`, `set_qr_path_returning`, `set_image_path`, `update_quantity`. |
| [services/products/src/routes.rs](../../services/products/src/routes.rs) | Mapiranje ruta na komponente ili HTTP handlere. Identifikatori: `product_routes`, `static_routes`. |
| [services/products/src/services/certificate_service.rs](../../services/products/src/services/certificate_service.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. Identifikatori: `new`, `generate`. |
| [services/products/src/services/image_service.rs](../../services/products/src/services/image_service.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. Identifikatori: `new`, `upload`, `get_image_path`. |
| [services/products/src/services/mod.rs](../../services/products/src/services/mod.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. |
| [services/products/src/services/product_policy.rs](../../services/products/src/services/product_policy.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. Identifikatori: `validate`, `required_fields`, `for_type`. |
| [services/products/src/services/product_service.rs](../../services/products/src/services/product_service.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. Identifikatori: `dec`, `new`, `find_all_by_farm_id`, `find_all`, `create`, `get_one`, `update`, `delete`, `validate_batch`, `authorize_read`. |
| [services/products/src/services/provenance_service.rs](../../services/products/src/services/provenance_service.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. Identifikatori: `new`, `get_provenance`, `get_provenance_by_qr`, `build`, `trace_token`. |
| [services/products/src/services/qr_service.rs](../../services/products/src/services/qr_service.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. Identifikatori: `new`, `get_qr_path`, `regenerate`. |
| [services/products/src/utils/image_utils.rs](../../services/products/src/utils/image_utils.rs) | Obrada slika/QR i filesystem pomoćne funkcije. Identifikatori: `is_allowed_mime`, `save_image`, `delete_image`. |
| [services/products/src/utils/mod.rs](../../services/products/src/utils/mod.rs) | Deklaracija i vidljivost Rust podmodula. |
| [services/products/src/utils/qr_utils.rs](../../services/products/src/utils/qr_utils.rs) | Obrada slika/QR i filesystem pomoćne funkcije. Identifikatori: `generate_qr`, `media_path`. |

## services/raw-materials

| Fajl | Uloga |
|---|---|
| [services/raw-materials/Cargo.toml](../../services/raw-materials/Cargo.toml) | Rust paket/workspace, feature-i i deklarisane zavisnosti. |
| [services/raw-materials/Dockerfile](../../services/raw-materials/Dockerfile) | Build i runtime definicija kontejnera. |
| [services/raw-materials/migrations/20240101000000_init.sql](../../services/raw-materials/migrations/20240101000000_init.sql) | Migracija šeme, ograničenja ili integracionog outbox-a vlasničke baze. |
| [services/raw-materials/migrations/20260914000000_production_consumption.sql](../../services/raw-materials/migrations/20260914000000_production_consumption.sql) | Migracija šeme, ograničenja ili integracionog outbox-a vlasničke baze. |
| [services/raw-materials/migrations/20260916000000_received_date.sql](../../services/raw-materials/migrations/20260916000000_received_date.sql) | Migracija šeme, ograničenja ili integracionog outbox-a vlasničke baze. |
| [services/raw-materials/migrations/20260917000000_transactional_outbox.sql](../../services/raw-materials/migrations/20260917000000_transactional_outbox.sql) | Migracija šeme, ograničenja ili integracionog outbox-a vlasničke baze. |
| [services/raw-materials/src/db.rs](../../services/raw-materials/src/db.rs) | Konekcioni pool i automatsko izvršavanje migracija. Identifikatori: `create_pool`. |
| [services/raw-materials/src/dtos/adjust_quantity_request.rs](../../services/raw-materials/src/dtos/adjust_quantity_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/raw-materials/src/dtos/create_raw_material_request.rs](../../services/raw-materials/src/dtos/create_raw_material_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/raw-materials/src/dtos/mod.rs](../../services/raw-materials/src/dtos/mod.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/raw-materials/src/dtos/update_raw_material_request.rs](../../services/raw-materials/src/dtos/update_raw_material_request.rs) | Ulazni/izlazni ili međuservisni tip ugovora. |
| [services/raw-materials/src/handlers/consumption.rs](../../services/raw-materials/src/handlers/consumption.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. Identifikatori: `consume`, `release`, `claims`, `item`, `quantity`, `consumption_is_atomic_scoped_and_idempotent`. |
| [services/raw-materials/src/handlers/materials.rs](../../services/raw-materials/src/handlers/materials.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. Identifikatori: `list`, `create`, `get_one`, `update`, `delete`, `low_stock`, `adjust_quantity`. |
| [services/raw-materials/src/handlers/mod.rs](../../services/raw-materials/src/handlers/mod.rs) | HTTP ulaz, extractors, uloge i mapiranje odgovora. |
| [services/raw-materials/src/main.rs](../../services/raw-materials/src/main.rs) | Pokretanje procesa, konfiguracija, DI, rute i pozadinski taskovi. Identifikatori: `main`. |
| [services/raw-materials/src/models/mod.rs](../../services/raw-materials/src/models/mod.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/raw-materials/src/models/query.rs](../../services/raw-materials/src/models/query.rs) | Domenski, persistence ili frontend tip i povezane konstante. Identifikatori: `offset`, `limit`. |
| [services/raw-materials/src/models/raw_material.rs](../../services/raw-materials/src/models/raw_material.rs) | Domenski, persistence ili frontend tip i povezane konstante. |
| [services/raw-materials/src/repository/mod.rs](../../services/raw-materials/src/repository/mod.rs) | SQL pristup vlasničkim tabelama i persistence operacije. |
| [services/raw-materials/src/repository/repository.rs](../../services/raw-materials/src/repository/repository.rs) | SQL pristup vlasničkim tabelama i persistence operacije. Identifikatori: `new`, `find_all`, `find_by_id`, `insert`, `update`, `soft_delete`, `find_low_stock`, `adjust_quantity`. |
| [services/raw-materials/src/routes.rs](../../services/raw-materials/src/routes.rs) | Mapiranje ruta na komponente ili HTTP handlere. Identifikatori: `raw_material_routes`. |
| [services/raw-materials/src/service/mod.rs](../../services/raw-materials/src/service/mod.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. |
| [services/raw-materials/src/service/service.rs](../../services/raw-materials/src/service/service.rs) | Poslovna pravila, koordinacija ili infrastrukturni helper servisa. Identifikatori: `to_decimal`, `new`, `list`, `create`, `get_one`, `update`, `delete`, `low_stock`, `adjust_quantity`, `validate_dates`. |

## services/read-models

| Fajl | Uloga |
|---|---|
| [services/read-models/Cargo.toml](../../services/read-models/Cargo.toml) | Rust paket/workspace, feature-i i deklarisane zavisnosti. |
| [services/read-models/Dockerfile](../../services/read-models/Dockerfile) | Build i runtime definicija kontejnera. |
| [services/read-models/migrations/20260917000000_projections.sql](../../services/read-models/migrations/20260917000000_projections.sql) | Migracija šeme, ograničenja ili integracionog outbox-a vlasničke baze. |
| [services/read-models/src/main.rs](../../services/read-models/src/main.rs) | Pokretanje procesa, konfiguracija, DI, rute i pozadinski taskovi. Identifikatori: `main`. |
| [services/read-models/src/projector.rs](../../services/read-models/src/projector.rs) | AMQP validacija, receipts, sekvence i upsert projekcije. Identifikatori: `valid`, `apply`, `start`, `consume`. |
| [services/read-models/src/queries.rs](../../services/read-models/src/queries.rs) | Role-scoped dashboard, proizvođači i interni query porekla. Identifikatori: `entity`, `farm_entities`, `dashboard`, `provenance`, `number`, `producers`. |

## typst-ftn

| Fajl | Uloga |
|---|---|
| [typst-ftn/.gitignore](../../typst-ftn/.gitignore) | Pomoćni izvor, formular ili dokumentacija akademskog rada. |
| [typst-ftn/README.md](../../typst-ftn/README.md) | Pomoćni izvor, formular ili dokumentacija akademskog rada. |
| [typst-ftn/biografija.typ](../../typst-ftn/biografija.typ) | Typst struktura, metapodaci ili prednji/završni deo akademskog dokumenta. |
| [typst-ftn/build_ftn_template_docx.py](../../typst-ftn/build_ftn_template_docx.py) | Python alat za generisanje akademskog DOCX dokumenta. |
| [typst-ftn/build_local_bite_docx.py](../../typst-ftn/build_local_bite_docx.py) | Python alat za generisanje akademskog DOCX dokumenta. |
| [typst-ftn/docx-artifact.md](../../typst-ftn/docx-artifact.md) | Pomoćni izvor, formular ili dokumentacija akademskog rada. |
| [typst-ftn/formulari/README.md](../../typst-ftn/formulari/README.md) | Pomoćni izvor, formular ili dokumentacija akademskog rada. |
| [typst-ftn/formulari/croppdfs.sh](../../typst-ftn/formulari/croppdfs.sh) | Pomoćni izvor, formular ili dokumentacija akademskog rada. |
| [typst-ftn/funkcije.typ](../../typst-ftn/funkcije.typ) | Typst struktura, metapodaci ili prednji/završni deo akademskog dokumenta. |
| [typst-ftn/kljucna.typ](../../typst-ftn/kljucna.typ) | Typst struktura, metapodaci ili prednji/završni deo akademskog dokumenta. |
| [typst-ftn/literatura.bib](../../typst-ftn/literatura.bib) | Bibliografija diplomskog rada. |
| [typst-ftn/metadata.typ](../../typst-ftn/metadata.typ) | Typst struktura, metapodaci ili prednji/završni deo akademskog dokumenta. |
| [typst-ftn/naslovna.typ](../../typst-ftn/naslovna.typ) | Typst struktura, metapodaci ili prednji/završni deo akademskog dokumenta. |
| [typst-ftn/poglavlja/1-uvod.typ](../../typst-ftn/poglavlja/1-uvod.typ) | Izvor poglavlja ili dodatka diplomskog rada; nije runtime aplikacije. |
| [typst-ftn/poglavlja/2-stanje.typ](../../typst-ftn/poglavlja/2-stanje.typ) | Izvor poglavlja ili dodatka diplomskog rada; nije runtime aplikacije. |
| [typst-ftn/poglavlja/3-specifikacija.typ](../../typst-ftn/poglavlja/3-specifikacija.typ) | Izvor poglavlja ili dodatka diplomskog rada; nije runtime aplikacije. |
| [typst-ftn/poglavlja/4-arhitektura.typ](../../typst-ftn/poglavlja/4-arhitektura.typ) | Izvor poglavlja ili dodatka diplomskog rada; nije runtime aplikacije. |
| [typst-ftn/poglavlja/5-implementacija.typ](../../typst-ftn/poglavlja/5-implementacija.typ) | Izvor poglavlja ili dodatka diplomskog rada; nije runtime aplikacije. |
| [typst-ftn/poglavlja/6-evaluacija.typ](../../typst-ftn/poglavlja/6-evaluacija.typ) | Izvor poglavlja ili dodatka diplomskog rada; nije runtime aplikacije. |
| [typst-ftn/poglavlja/7-zakljucak.typ](../../typst-ftn/poglavlja/7-zakljucak.typ) | Izvor poglavlja ili dodatka diplomskog rada; nije runtime aplikacije. |
| [typst-ftn/poglavlja/dodatak 1 - skracenice.typ](../../typst-ftn/poglavlja/dodatak%201%20-%20skracenice.typ) | Izvor poglavlja ili dodatka diplomskog rada; nije runtime aplikacije. |
| [typst-ftn/poglavlja/dodatak 2 - pojmovi.typ](../../typst-ftn/poglavlja/dodatak%202%20-%20pojmovi.typ) | Izvor poglavlja ili dodatka diplomskog rada; nije runtime aplikacije. |
| [typst-ftn/sukob-interesa.typ](../../typst-ftn/sukob-interesa.typ) | Typst struktura, metapodaci ili prednji/završni deo akademskog dokumenta. |
| [typst-ftn/zadatak.typ](../../typst-ftn/zadatak.typ) | Typst struktura, metapodaci ili prednji/završni deo akademskog dokumenta. |
| [typst-ftn/zavrsni-rad.typ](../../typst-ftn/zavrsni-rad.typ) | Typst struktura, metapodaci ili prednji/završni deo akademskog dokumenta. |

## Grupisani artefakti

- `.sqlx/`: 84 metadata fajlova, koriste ih SQLx query makroi za offline kompilaciju.
- `.env`: lokalni URL-ovi, portovi i tajne; sadržaj nije kopiran.
- `target/`, frontend `node_modules/`, `dist/`, `.angular/`: zavisnosti i generisani build/cache rezultati.
- `typst-ftn/*.docx`, `*.pdf`, logo/slike/formulari i `typst.exe`: akademski artefakti i resursi, van poslovnog runtime-a.
- `.git/` i frontend Git metadata: istorija/verzionisanje, ne runtime podaci.

Popisano 325 tekstualnih izvora. Za glavni tehnički opis koristiti dokumente 01–12.
