# dracker

A modern, fast backend service for dracker.io, built with Rust.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Setup

Clone the repository.

```bash
git clone git@github.com:junjitree/dracker.git
```

Create a `.env` file to configure environment variables.

```bash
cp .env.sample .env

```

Run the server: NOTE: The server will start on `http://localhost:3000`.

```bash
cargo run
```

Check the server is working.

```bash
curl -i http://localhost:3000/
```

## API Endpoints

- `GET /` - Health check / Root endpoint (returns `418 I'm a teapot`).

## Project Structure

- `src/main.rs`: Entry point and server initialization.
- `src/http/`: Route definitions and handlers.
- `src/error.rs`: Centralized error handling.
- `src/util.rs`: Utility functions (logging, environment setup).
- `src/result.rs`: Custom Result type.

## Development

- **Tracing:** Tracing is enabled and log level is set based on build type
  (Debug: `DEBUG`, Release: `INFO`).
- **Error Handling:** Custom `Error` enum that implements `IntoResponse` for
  consistent API error responses.
