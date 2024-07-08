# microservice
`valorant -> coding pipeline` Official Microservice monorepo 

# From Discord
### Stack
- Frontend:     React (TODO)
- Backend:      Rust
- Caching:      Redis (TODO)
- Database:     Postgres (TODO)
- CI/CD:        Terraform + Github Actions (TODO)
- AWS:
  - ECS (kubernetes)
  - ECR (docker)
  - RDS (postgres; do we even need this?)

### Initial service(s)
- Long-lived dynamic scraping service (scraper)
- Backend
  - A server to interface with the scraper
  - Will act as a reverse proxy/gateway
- Frontend
  - Will interface with the backend (or any other services that we would like to add potentially?)

# Getting Started
- Download rust [here](https://www.rust-lang.org/tools/install)
- Make sure you're using VSCode (if not let me know and we'll figure something out)
  - Download the `rust-analyzer` extension. This will help **_MASSIVELY_** with Rust development productivity.
### Setup TODOs
- .github
- Docker(file)
- Terraform