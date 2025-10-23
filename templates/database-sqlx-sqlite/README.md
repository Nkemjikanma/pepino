### Prerequisites
    - Rust - Node.js - PostgreSQL - just - sqlx-cli - typeshare-cli

### Quick start
   ##### Clone and setup
   cp .env.example .env
   > Edit .env with your database credentials
   
   ###### Install dependencies
   just install
   
   ###### Set up database
   createdb your_database_name
   just migrate
   
   ###### Generate types
   just generate-types
   
   ###### Run (in separate terminals)
   just dev-server
   just dev-client
   
### Project Structure
    - Explain the directory layout
    - What each folder does
### API Endpoints
    - List the endpoints
    - Example requests
### Adding Features
    - How to add new models
    - How to regenerate types
    - How to add endpoints
