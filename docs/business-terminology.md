# Business terminology

The owner role is `BusinessOwner` in Rust/TypeScript and user responses,
`BUSINESS_OWNER` in registration requests, stored roles and JWT claims, and
**Business owner** in the interface. The entity is `Business`, its identifier is
`business_id`, and its table and auth resource are `businesses`.

The profile uses **Personal profile**, **Business profile** and **Add your business**.
The frontend management route is `/business`; auth endpoints start with
`/api/auth/businesses`; the scoped product list is `/api/products/business`.
Public provenance uses `business_name`.

## Existing installations

Each service includes `20260925000000_business_terminology.sql`. Applied historical
migrations are unchanged so SQLx checksums remain valid. New installations run the
same complete migration chain. The forward migrations preserve identifiers and
relationships, rename columns and catalog objects, convert owner roles and persisted
JSON, and update production compensation triggers and the projection index.

Deploy all six services and the frontend together during a maintenance window:
back up the databases, stop the application services to avoid mixing old and new
database/API contracts, rebuild them, then start the new versions. SQLx applies the
migrations at service startup. Do not remove database volumes. Old application
binaries cannot run against the renamed schema.

Already-issued signed sessions and queued integration events are translated at the
read boundary in `common::business_upgrade`; newly written tokens, events and API
responses use only the new terminology. Existing frontend sessions refresh through
`/api/auth/me`. Previous API paths are not retained as aliases.

## Verification

`node scripts/test-business-migration.cjs` tests all migration chains in isolated,
rolled-back schemas using the six local database containers. It verifies owner
roles, business scope, custom types, compensation triggers, reservations, checkout
recovery, outbox events and projection indexes without changing application data.

`cargo test --workspace --locked` includes compatibility checks for signed sessions
and queued events. Set `SQLX_OFFLINE=true` to use the refreshed query metadata.
Run `npm run build` and `npm test -- --watch=false` in the frontend directory.
