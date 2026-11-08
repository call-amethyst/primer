# Primer

Primer is a full-stack web application built with **Angular + Tailwind CSS** on the frontend and **Rust + Axum** on the backend, backed by **PostgreSQL**. This repository is a monorepo: both apps live here, but can be developed and tested separately.

## Tech stack

| Layer | Technology |
| --- | --- |
| Frontend | Angular, TypeScript, Tailwind CSS |
| Backend | Rust, Axum, Cargo Watch |
| Database | PostgreSQL |
| Development | Docker Compose |
| Git hooks | Lefthook, Prettier, Rustfmt, Clippy |

## Prerequisites

- Git
- Docker Engine or Docker Desktop with Docker Compose
- Node.js and npm (for running the repo's Lefthook tooling locally)
- Rust toolchain (for local Rustfmt/Clippy hooks)

## Getting started

Clone the repository and enter its directory:

```bash
git clone https://github.com/call-amethyst/primer.git
cd primer
```

Start the complete development stack:

```bash
docker compose up --build
```

Open **http://localhost:4200** in your browser.

| Service | URL / connection |
| --- | --- |
| Angular frontend | http://localhost:4200 |
| Axum API | http://localhost:3000 |
| PostgreSQL (from host) | localhost:5432 |
| PostgreSQL (inside Docker) | db:5432 |

> This Compose configuration is **for local development**. Do not deploy it as-is to a public server: it includes development servers and sample database credentials. If the database schema requires migrations, run those separately according to the backend's migration tooling.

### Try the frontend ↔ backend connection

The example frontend button sends a GET request to `GET /api/hello`, which returns `Hello from Axum!`.

```bash
curl http://localhost:3000/api/hello
curl http://localhost:4200/api/hello
```

The second request goes through Angular's development proxy to the `api` container. Both should return the greeting when the stack is healthy.

## Development workflow

- **Frontend:** edit files in `frontend/src/`. The Angular development server automatically refreshes when files change.
- **Backend:** edit files in `backend/src/`. Cargo Watch recompiles and restarts Axum when Rust files change.
- **Stop the stack:** `docker compose down` (keeps database data).
- **Follow logs:** `docker compose logs -f api` or `docker compose logs -f frontend`.
- **Reset local database and all Compose volumes:** `docker compose down -v` (**deletes local data**).

### API proxy

Inside Docker, Angular forwards `/api/...` requests to `http://api:3000`. From browser code, call relative URLs such as:

```ts
fetch('/api/hello');
```

This avoids hardcoding container hostnames or introducing unnecessary CORS configuration for local development.

## Git hooks

Install the repository tooling after cloning:

```bash
npm ci
npx lefthook install
```

Lefthook runs the configured formatting checks before commits and build/lint checks before pushes. You can also run them manually:

```bash
npx prettier --write frontend
cargo fmt --manifest-path backend/Cargo.toml
npx lefthook run pre-commit
```

These local hooks require Node.js and, for Rust-specific checks, a Rust toolchain on your machine. Docker does not automatically install tools on your host.

## Repository layout

```text
primer/
├── backend/        # Axum API, Rust source and migrations
├── frontend/       # Angular app, Tailwind and proxy configuration
├── e2e/            # End-to-end tests (when configured)
├── compose.yml     # Local development services
├── lefthook.yml    # Git hooks
├── LICENSE.md      # MIT License
└── README.md
```

## Contributing

Issues and pull requests are welcome. Please describe the change and verify the relevant formatting and build checks before opening a pull request. By contributing, you confirm you have the right to submit the code under the project's license. Contributor licensing and ownership arrangements should be agreed on by the project maintainers before accepting substantial outside contributions.

## License and attribution

Primer is licensed under the **[MIT License](LICENSE.md)**.

You may use, modify, redistribute, sublicense, and sell copies of the software, including for commercial purposes. The condition is that you **keep the copyright notice and MIT permission notice** in copies or substantial portions of the software.

Copyright © 2026 **Call-Amethyst and Luhplus**.

MIT does not require visible credit in a product's user interface or documentation. Third-party libraries and assets retain their own licenses.
