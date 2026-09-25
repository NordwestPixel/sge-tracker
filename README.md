# SGE Tracker

A matchday tracker for Eintracht Frankfurt: a Rust backend that syncs fixtures, results and
standings from [OpenLigaDB](https://www.openligadb.de/) into PostgreSQL, and a Flutter app that
reads from it.

> **Work in progress.** The backend's OpenLigaDB client and data mapping are done; the sync job,
> the API endpoints and the app are still being built.

## How it works

- The **app talks only to this backend**, and the backend serves only from PostgreSQL.
- **OpenLigaDB is touched exclusively by a background sync job.** The API keeps working when
  OpenLigaDB is down, and the app never depends on a third-party API's availability or format.

## Stack

| Part | Technology |
|---|---|
| Backend | Rust: axum, tokio, sqlx, reqwest |
| Database | PostgreSQL 17 |
| App | Flutter (Android, iOS) |

## Repository layout

```
server/   Rust backend (standalone Cargo package, migrations, test fixtures)
app/      Flutter app
docs/     Design notes
```

## Running the backend locally

Requires Rust and Docker (or Podman). From `server/`:

```bash
cp .env.example .env          # set POSTGRES_PASSWORD and DATABASE_URL
docker compose up -d          # PostgreSQL on 127.0.0.1:5433
cargo run                     # runs migrations, then serves on BIND_ADDR
```

For the local container, `DATABASE_URL` is
`postgres://sge:<POSTGRES_PASSWORD>@127.0.0.1:5433/sge_tracker`.

```bash
cargo test                    # offline tests
cargo test -- --ignored       # tests against the live OpenLigaDB API
```

## Data and attribution

This project contains information from [OpenLigaDB](https://www.openligadb.de/), which is made
available here under the [Open Database License (ODbL) 1.0](https://opendatacommons.org/licenses/odbl/1-0/).
This applies to the test fixtures in `server/tests/fixtures/` and to all match and table data
the backend stores and serves.

OpenLigaDB is maintained by its community. Match data, results and standings are shown as
provided and may be incomplete, delayed or incorrect.

## Disclaimer

This is an unofficial, non-commercial fan project. It is not affiliated with, endorsed by or
connected to Eintracht Frankfurt e.V. or Eintracht Frankfurt Fußball AG. Club, team and
competition names are used only to describe the data shown and remain the property of their
respective owners.

## License

The source code is licensed under the [MIT License](LICENSE). The MIT License does not cover the
OpenLigaDB data, which remains under the ODbL as described above.
