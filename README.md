# Pepino 🥒

> A fullstack scaffolder for modern Rust + TypeScript web applications

Axum backend • React frontend • Shared types • SQLite or Postgres


**Stop configuring, start building.**

## Prerequisites
Install these tools before starting:

```bash
# Install just
cargo install just

# Install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# Install typeshare-cli
cargo install typeshare-cli

# Install cargo-watch (optional, for hot reload)
cargo install cargo-watch
```

## What You Get
Pepino generates a complete, production-ready fullstack project

- Backend: Axum + SQLx with example CRUD endpoints
- Frontend: React + Vite + TanStack Query
- Type Safety: Rust types auto-generate TypeScript interfaces
- Database: Choose PostgreSQL or SQLite with migrations included
- Dev Tools: Hot reload, justfile commands, structured error handling
- Working Example: Full user management flow out of the box

## Available Commands
Once your project is generated, use these `just` commands:

```bash
# Database (Docker)
just db-up           # Start PostgreSQL
just db-down         # Stop PostgreSQL
just install         # install server and vite packages
just dev-server      # Start Rust backend with hot reload
just dev-client      # Start Vite dev server
just migrate         # Run database migrations
just generate-types  # Generate TS types from Rust
just build          # Build for production
just clean          # Clean build artifacts
just test           # Run tests
```
## Roadmap
- ⏳ Actix-web backend option
- ⏳ Diesel ORM option
- ⏳ Authentication scaffolding
- ⏳ Docker configuration
- ⏳ CI/CD templates

## Contributing
## License
MIT • [Nkemjika](https://github.com/nkemjikanma)
