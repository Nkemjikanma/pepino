
# Pepino 🥒

> A fullstack scaffolder for modern Rust + TypeScript web applications

Pepino generates production-ready fullstack projects with:
- 🦀 **Rust backend** (Axum + SQLx)
- ⚡ **Vite frontend** (React + TanStack Query)
- 🔄 **Type sharing** (Rust types → TypeScript)
- 🗄️ **Database ready** (PostgreSQL + migrations)
- 🛠️ **Dev tools** (hot reload, justfile commands)

**Stop configuring, start building.**

## Quick Start

```bash
# 1. Install pepino
cargo install pepino

# 2. Create a new project
pepino new my-app
cd my-app

# 3. Set up database
createdb my_app_dev
cp .env.example .env
# Edit .env with your database URL

# 4. Run migrations
just migrate

# 5. Generate TypeScript types
just generate-types

# 6. Start development (in separate terminals)
just dev-server
just dev-client

# 7. Open http://localhost:5173
```

## Prerequisites
Install these tools before starting:

### Required
- **Rust** (1.75+) - [Install](https://rustup.rs/)
- **Node.js** (20+) - [Install](https://nodejs.org/)
- **PostgreSQL** (14+) - [Install](https://www.postgresql.org/download/)

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

### **Available Commands**

## Available Commands

Once your project is generated, use these `just` commands:

```bash
just dev-server      # Start Rust backend with hot reload
just dev-client      # Start Vite dev server
just migrate         # Run database migrations
just generate-types  # Generate TS types from Rust
just build          # Build for production
just clean          # Clean build artifacts
just test           # Run tests
```

## Current Features

- ✅ Axum + SQLx backend generation
- ✅ React + Vite frontend generation
- ✅ TypeScript type generation from Rust
- ✅ PostgreSQL database setup
- ✅ Example CRUD implementation
- ✅ Development hot reload

## Roadmap
- ⏳ Actix-web backend option
- ⏳ Diesel ORM option
- ⏳ SQLite support
- ⏳ Authentication scaffolding
- ⏳ Docker configuration
- ⏳ CI/CD templates

Want a feature? [Open an issue](https://github.com/nkemjikanma/pepino/issues)!


## Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) for details

## Author

Created by [Nkemjika](https://github.com/nkemjikanma)

---

**Found this useful? Give it a ⭐ on GitHub!**
