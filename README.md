
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

## What You Get
Pepino generates a complete, production-ready fullstack project:

### Project Structure
\```
my-app/
├── Cargo.toml              # Workspace configuration
├── .env.example            # Environment template
├── .gitignore             
├── justfile                # Task runner commands
├── README.md               # Project documentation
│
├── server/                 # Rust backend (Axum + SQLx)
│   ├── Cargo.toml
│   ├── migrations/
│   │   └── *_create_users.sql
│   └── src/
│       ├── main.rs         # Server entry point
│       ├── config.rs       # Configuration management
│       ├── db.rs           # Database connection pool
│       ├── common/         # Shared utilities
│       │   ├── api.rs      # API response types
│       │   ├── errors.rs   # Error handling
│       │   └── mod.rs
│       ├── handlers/       # API endpoints
│       │   ├── public.rs   # Public routes (users, health)
│       │   └── mod.rs
│       └── models/         # Data models
│           ├── user.rs     # User model with typeshare
│           └── mod.rs
│
└── client/                 # React frontend (Vite + TypeScript)
    ├── package.json
    ├── vite.config.ts      # Proxy config for API
    ├── tsconfig.json
    ├── index.html
    └── src/
        ├── main.tsx        # App entry
        ├── App.tsx         # Main component
        ├── api/
        │   └── users.ts    # API client
        └── types/
            └── api.ts      # Generated TypeScript types
\```

### Out-of-the-Box Features

**Backend:**
- ✅ Health check endpoint (`GET /api/health`)
- ✅ User CRUD endpoints (`GET/POST /api/users`)
- ✅ PostgreSQL connection pooling
- ✅ Structured error handling
- ✅ CORS configuration
- ✅ Request logging with `tracing`

**Frontend:**
- ✅ React 19 with TypeScript
- ✅ TanStack Query for data fetching
- ✅ Type-safe API client
- ✅ Example components with real API calls
- ✅ Hot module replacement

**Type Safety:**
- ✅ Rust structs automatically generate TypeScript types
- ✅ End-to-end type safety from database to UI
- ✅ `just generate-types` command for updates

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
