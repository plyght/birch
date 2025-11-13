# Birch API

Backend API service for Birch SaaS, built with Rust and Axum.

## Architecture

- **Framework**: Axum (async Rust web framework)
- **Database**: PostgreSQL via Supabase
- **Cache**: Redis
- **Authentication**: JWT validation and API keys
- **Encryption**: ChaCha20Poly1305 envelope encryption with workspace-specific keys

## Getting Started

### Prerequisites

- Rust 1.70+
- PostgreSQL (or Supabase account)
- Redis

### Environment Variables

Copy `../../env.example` to `.env` and configure. Generate the master key with:

```bash
openssl rand -hex 32
```

See `../../env.example` for full configuration details.

### Run Migrations

```bash
cd ../../infra/supabase
supabase migration up
```

### Start the Server

```bash
cargo run --bin birch-api
```

The API will be available at `http://localhost:3000`.

## API Endpoints

### Health Check

```
GET /health
```

Returns API health status.

### Workspaces

```
POST   /api/v1/workspaces          Create workspace
GET    /api/v1/workspaces          List workspaces
GET    /api/v1/workspaces/:id      Get workspace
PUT    /api/v1/workspaces/:id      Update workspace
DELETE /api/v1/workspaces/:id      Delete workspace
```

### Members

```
POST   /api/v1/workspaces/:id/members              Invite member
GET    /api/v1/workspaces/:id/members              List members
PUT    /api/v1/workspaces/:id/members/:user_id     Update role
DELETE /api/v1/workspaces/:id/members/:user_id     Remove member
```

### Providers

```
POST   /api/v1/workspaces/:id/providers/:provider     Configure provider
GET    /api/v1/workspaces/:id/providers               List providers
GET    /api/v1/workspaces/:id/providers/:provider     Get provider config
PUT    /api/v1/workspaces/:id/providers/:provider     Update provider
DELETE /api/v1/workspaces/:id/providers/:provider     Delete provider
```

### Credentials

```
POST /api/v1/workspaces/:id/credentials                         Store credential
GET  /api/v1/workspaces/:id/credentials/:provider/:secret_name  Retrieve credential
```

### API Keys

```
POST   /api/v1/workspaces/:id/api-keys          Create API key
GET    /api/v1/workspaces/:id/api-keys          List API keys
DELETE /api/v1/workspaces/:id/api-keys/:key_id  Revoke API key
```

## Authentication

All API requests require authentication via Bearer token:

```bash
curl -H "Authorization: Bearer <your-api-key>" \
  https://api.birch.sh/api/v1/workspaces
```

## Testing

Run tests:

```bash
cargo test
```

## Project Structure

```
src/
├── main.rs                 # Entry point
├── lib.rs                  # Module exports
├── supabase/              # Database client
├── auth/                  # Authentication & authorization
├── vault/                 # Credential encryption & storage
├── credentials/           # Credential resolution & caching
├── workspace/             # Multi-tenancy & RBAC
├── metering/              # Usage tracking
└── api/                   # HTTP routes & handlers
    ├── routes.rs
    └── handlers/
        ├── workspaces.rs
        ├── members.rs
        ├── providers.rs
        ├── credentials.rs
        └── api_keys.rs
```

## Security

### Encryption

- **Algorithm**: ChaCha20Poly1305 (AEAD cipher)
- **Key Derivation**: Workspace-specific keys derived from master key
- **Nonce**: Random 12-byte nonce per encryption operation
- **Storage**: Encrypted credentials stored in PostgreSQL with RLS

### Database Security

- **Row Level Security**: Enforces workspace isolation at database level
- **Prepared Statements**: All queries use parameterized statements
- **Connection Pooling**: Secure connection management via Deadpool

### Authentication

- **User Sessions**: JWT validation via Supabase Auth
- **API Keys**: Argon2id hashed keys for programmatic access
- **RBAC**: Five-tier role system (Owner/Admin/Operator/Viewer/Auditor)

## Deployment

### Docker

```bash
docker build -t birch-api .
docker run -p 3000:3000 --env-file .env birch-api
```

### Fly.io

```bash
fly launch
fly secrets set VAULT_MASTER_KEY=...
fly deploy
```

## License

MIT - See LICENSE file in repository root

