# dracker

A modern, fast backend service for dracker.io, built with Rust.

## Tech Stack

- **Framework:** [Axum](https://github.com/tokio-rs/axum)
- **Runtime:** [Tokio](https://tokio.rs/)
- **ORM:** [SeaORM](https://www.sea-ql.org/SeaORM/) (MySQL)
- **Serialization:** [Serde](https://serde.rs/)
- **Logging:** [Tracing](https://tracing.rs/)

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Docker](https://www.docker.com/) or [Podman](https://podman.io/) (for local
  database)
- [MySQL/MariaDB client](https://dev.mysql.com/downloads/client-shell/) (for
  local database setup)

### Setup

- Clone the repository:

  ```bash
  git clone git@github.com:junjitree/dracker.git
  cd dracker
  ```

- Configure environment variables:

  ```bash
  cp .env.sample .env
  ```

- Start the local database (requires Docker/Podman):

  ```bash
  ./bin/db
  ```

- Run migrations:

```bash
./bin/migration up
```

- Run the server:

  ```bash
  cargo run
  ```

- The server will start on `http://localhost:3000`.

  ```bash
  curl -i http://localhost:3000
  ```

## Health Check endpoint

- `GET /` - Health check / Root endpoint (returns `418 I'm a teapot`).

## Project Structure

- `src/main.rs`: Entry point and server initialization.
- `src/http/`: Route definitions and handlers.
- `src/error.rs`: Centralized error handling.
- `src/util.rs`: Utility functions (logging, environment setup).
- `src/result.rs`: Custom Result type.
- `migration/`: SeaORM migration project.

## Development

- **Tracing:** Tracing is enabled and log level is set based on build type
  (Debug: `DEBUG`, Release: `INFO`).
- **Error Handling:** Custom `Error` enum that implements `IntoResponse` for
  consistent API error responses.
- **Database:** Use `./bin/db` to start a local MySQL instance in Docker/Podman.
- **Migrations:** Use `./bin/migration` to run migrations. This is a wrapper
  around `cargo run --package migration`.
