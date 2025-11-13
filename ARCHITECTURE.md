# Birch Architecture

This document provides a high-level overview of Birch's architecture and design principles.

## Overview

Birch is a secret rotation engine built as a single Rust binary with optional SaaS extensions. The architecture prioritizes simplicity, security, and operational safety.

## Core Components

### CLI

The command-line interface provides:

- Manual secret rotation operations
- Pool management (create, add, remove keys)
- Configuration management
- Status monitoring via interactive TUI
- Rollback capabilities
- Dry-run mode for safe testing

**Key Features:**
- Single binary deployment
- No external dependencies for core functionality
- File-based configuration (`~/.birch/config.toml`)
- Environment variable overrides

### Daemon

Optional background service for application-triggered rotations:

- HTTP API on `localhost:9123` (configurable)
- Accepts rotation requests from applications
- Non-blocking: queues operations and returns immediately
- Updates production secrets asynchronously
- Health check endpoint for monitoring

**Integration Patterns:**
- Manual: Applications POST to `/rotate` endpoint
- Automatic: TypeScript SDK intercepts HTTP 429 responses

### Key Pools

Differentiating feature enabling automatic rate limit handling:

- Pre-configured sets of API keys per secret
- Sequential rotation through available keys
- Automatic marking of exhausted keys
- Low-threshold warnings
- Per-pool cooldown management

**Use Cases:**
- Rate-limited APIs (TikTok, Twitter, Stripe)
- High-volume applications
- Zero-downtime key rotation

### Connectors

Extensible provider integration system:

**Development:**
- `.env` file updates
- Atomic writes with rollback support

**Hosting Providers:**
- Vercel
- Netlify
- Render
- Cloudflare Workers
- Fly.io

**Cloud Secret Managers:**
- AWS Secrets Manager
- GCP Secret Manager
- Azure Key Vault

**Capabilities:**
- Secret updates
- Optional redeployment triggers
- Rollback support
- Status checks

### Audit Logging

Cryptographically signed immutable logs:

- **Algorithm**: Ed25519 signatures
- **Storage**: File-based (configurable path)
- **Format**: JSON lines
- **Fields**: Timestamp, actor, action, secret, environment, success, signature

**Use Cases:**
- Compliance audits
- Security forensics
- Operational troubleshooting

### Safety Rails

Multiple layers of protection:

**Locking:**
- File-based single-writer locks
- Per-secret/environment isolation
- Prevents concurrent rotations

**Cooldowns:**
- Configurable delay between rotations
- Default: 60 seconds
- Prevents rapid successive changes

**Maintenance Windows:**
- Time-based restrictions
- Day-of-week scheduling
- Protects production environments

**Dry-Run Mode:**
- Preview changes before applying
- Validates configuration
- Tests connector integration

**Rollback:**
- Time-boxed recovery window (default: 1 hour)
- Automatic key revocation
- Redeploy support

## SaaS Architecture

### Overview

Birch SaaS extends the CLI with team collaboration, policy orchestration, and enterprise features.

### Components

**API Server (Rust + Axum):**
- RESTful HTTP API
- JWT + API key authentication
- Multi-tenant with workspace isolation
- Credential storage and retrieval
- Policy evaluation engine

**Web Dashboard (Next.js + Supabase):**
- Workspace management
- Team member administration
- Provider configuration
- Policy management
- Approval workflows
- Audit log viewer
- Usage analytics

**Database (PostgreSQL via Supabase):**
- Workspace data
- User management
- Provider configurations
- Encrypted credentials
- OAuth tokens
- Policies and rules
- Audit logs
- API keys
- Usage metering

**Cache (Redis):**
- Credential caching (5-15 min TTL)
- Rate limiting
- Session management

### Credential Modes

Three operational modes per provider:

**1. Hosted Credentials:**
- SaaS stores encrypted credentials
- Envelope encryption (ChaCha20Poly1305)
- Workspace-specific keys
- Enterprise: Customer-managed master keys

**2. KMS Integration:**
- Credentials in customer AWS/GCP/Azure KMS
- On-demand decryption
- Short-lived caching
- Zero credential storage in SaaS

**3. OAuth Integration:**
- Encrypted refresh tokens
- Automatic token refresh
- Provider-specific implementations
- Secure token rotation

### Security Model

**Encryption:**
- ChaCha20Poly1305 AEAD cipher
- Workspace-specific key derivation
- Random 12-byte nonces
- Secure key storage

**Multi-Tenancy:**
- Database-level isolation (Row Level Security)
- API-level workspace validation
- No cross-workspace data leakage

**Authentication:**
- Supabase Auth for user sessions
- Argon2id for API key hashing
- Role-based access control (RBAC)

**Authorization:**
- Five-tier role system:
  - Owner: Full access + billing
  - Admin: Full operations
  - Operator: Rotate only
  - Viewer: Read-only
  - Auditor: Logs only

### Policy Engine

**Policy Types:**
- Rotation thresholds (soft/hard limits)
- Maintenance windows
- Approval requirements
- Preview-first deployments
- Auto-redeploy toggles

**Evaluation:**
- Policy inheritance (workspace → provider → secret)
- Conflict resolution
- Real-time evaluation
- Audit trail

**Approval Workflows:**
- Two-person rule
- Timeout handling
- Notification integration
- State machine tracking

### Alerts

**Channels:**
- Email
- Slack
- Webhooks
- In-app notifications

**Event Types:**
- Near-limit warnings
- Rotation completion
- Policy violations
- Approval requests
- Credential expiration
- System failures

**Features:**
- Deduplication
- Cooldown management
- Escalation rules

## Design Principles

### Simplicity

- Single binary for core functionality
- No runtime dependencies
- File-based configuration
- Direct provider API integration
- No traffic proxying

### Security

- Cryptographically signed audit logs
- Secrets never logged
- Masked output in CLI
- Envelope encryption for storage
- Multi-tenant isolation
- Least privilege access

### Safety

- Multiple safety rails
- Dry-run mode
- Rollback support
- Confirmation prompts
- Cooldown periods
- Maintenance windows

### Extensibility

- Connector architecture
- Plugin system for providers
- Configurable policies
- Webhook integration
- API-first design

### Operational Excellence

- Zero-downtime rotations
- Automatic rollback
- Health checks
- Status monitoring
- Comprehensive logging

## Data Flow

### CLI Rotation

```
User Command
    → Lock Acquisition
    → Policy Check (if SaaS)
    → Connector Execute
        → Generate New Secret
        → Update Provider
        → Optional Redeploy
    → Audit Log
    → Lock Release
```

### Daemon Rotation

```
App HTTP 429
    → POST /rotate
    → Queue Operation
    → Immediate Response
        → Next Key from Pool
    → Async Background:
        → Update Production Secrets
        → Mark Old Key
        → Audit Log
```

### SaaS Rotation

```
CLI/Dashboard Request
    → API Authentication
    → Workspace Validation
    → Policy Evaluation
    → Approval Check (if required)
    → Credential Resolution
        → Hosted: Decrypt from vault
        → KMS: Request from customer KMS
        → OAuth: Exchange refresh token
    → Connector Execute
    → Audit Log
    → Usage Metering
```

## Deployment

### CLI

**Distribution:**
- Pre-built binaries for major platforms
- Cargo install
- Source compilation

**Requirements:**
- None (static binary)

### SaaS

**Infrastructure:**
- API: Fly.io / Railway / AWS ECS
- Database: Supabase PostgreSQL
- Cache: Redis Cloud / AWS ElastiCache
- Frontend: Vercel / Netlify

**Scaling:**
- Stateless API servers (horizontal scaling)
- Connection pooling for database
- Redis cluster for caching
- CDN for frontend

**Monitoring:**
- Health checks
- Prometheus metrics
- Log aggregation (Loki/ELK)
- Error tracking (Sentry)
- Uptime monitoring

## Repository Structure

```
birch/
├── apps/
│   ├── cli/              Rust CLI and daemon
│   ├── api/              Rust API server
│   ├── web/              Next.js dashboard
│   └── docs/             Documentation site
├── packages/
│   └── client/           TypeScript SDK
├── infra/
│   ├── supabase/        Database migrations
│   ├── examples/        Example applications
│   └── tests/           Integration tests
└── Makefile             Build automation
```

## Future Considerations

### Planned Features

- Additional cloud provider support
- Enhanced policy templates
- Compliance reporting
- Advanced analytics
- Multi-region support
- SCIM provisioning
- Custom connectors API

### Scalability

- Distributed locking for multi-instance CLI
- Sharded credential storage
- Async job queue for high-volume rotations
- Regional deployments

### Security Enhancements

- Hardware security module (HSM) integration
- Key escrow for disaster recovery
- Advanced threat detection
- Anomaly detection

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for development guidelines and contribution process.

## License

MIT - See LICENSE file for details.

