### Prerequisites
    - Rust - Node.js - PostgreSQL - just - sqlx-cli - typeshare-cli

### Quick start
   ##### Clone and setup
   cp .env.example .env
   > Edit .env with your database credentials

   # 1. Start PostgreSQL in Docker
   just db-up

   # 2. Run migrations
   just migrate

   ###### Install dependencies
   just install

   ###### Run (in separate terminals)
   just dev-server
   just dev-client
