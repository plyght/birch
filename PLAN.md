# Keystone: Flexible Credential Model & SaaS Plan

## Overview

Keystone extends Birch with a SaaS offering providing flexible credential management modes, policy orchestration, team collaboration, and enterprise features—all while maintaining full OSS transparency.

## Technology Stack: Supabase

**Why Supabase:**
- PostgreSQL database, built-in auth (OAuth, email/password, SSO), Row Level Security (RLS) for multi-tenancy
- Real-time subscriptions, file storage, edge functions
- Open source (self-hostable), great DX, generous free tier

**Integration:**
- Auth: Supabase Auth for users, custom API keys table for programmatic access
- Database: Supabase PostgreSQL with RLS policies for workspace isolation
- Real-time: Dashboard updates via Supabase Realtime
- Storage: Audit exports via Supabase Storage
- Rust: `postgrest`/`tokio-postgres` for DB, `jsonwebtoken` for JWT validation

## Three Operating Modes

### 1. Self-Hosted (OSS)
- Local credentials (`~/.birch/config.toml` or env vars)
- Free, unlimited, CLI-only, full control
- Use cases: Individual devs, small teams, air-gapped environments

### 2. SaaS - Hosted Credentials
- SaaS stores encrypted credentials, zero management burden
- Envelope encryption, customer-managed keys (Enterprise)
- Use cases: Startups/SMBs prioritizing convenience

### 3. SaaS - Customer-Managed Credentials
- Credentials via OAuth, customer KMS, or API keys
- SaaS never stores credentials
- Use cases: Enterprise compliance, security-conscious teams

**Credential Modes:**
- **OAuth**: Encrypted refresh tokens, automatic exchange
- **KMS**: On-demand decryption from customer KMS (AWS/GCP/Azure)
- **API Key**: Short-lived keys to fetch credentials

## Flexible Per-Provider Configuration

Mix credential modes per provider:

```toml
[providers.vercel]
mode = "oauth"  # Customer-managed via OAuth

[providers.aws]
mode = "kms"    # Customer-managed via AWS KMS
kms_key_id = "arn:aws:kms:..."

[providers.render]
mode = "hosted" # SaaS-managed (encrypted vault)
```

**Benefits:** Convenience for less sensitive providers, security for critical infrastructure, compliance flexibility.

## Credential Resolution Flow

```
Rotation Request → Check Provider Mode
├─ Hosted: Decrypt from vault (customer KMS key if Enterprise), cache (short TTL)
├─ KMS: Request decryption from customer KMS, cache (5-15 min)
├─ OAuth: Exchange refresh token → access token, cache (provider TTL)
└─ API Key: Fetch from customer endpoint, use immediately, discard
```

**Implementation:** Credential cache (Redis, 5-15 min TTL), retry with exponential backoff, circuit breaker, never log decrypted values.

## Feature Set

### Policy Engine
- Thresholds (soft/hard limits), maintenance windows, preview-first deployments
- Two-person approval workflows, auto-redeploy toggle (default OFF)
- Policy inheritance (workspace → provider → secret)

### Alerts
- Channels: Slack, email, webhooks, in-app
- Types: Near-limit warnings, rotation notifications, policy violations, approval requests, credential expiration
- Features: Deduplication, cooldown management, escalation rules

### Audit
- Immutable, cryptographically signed logs
- Export (JSON/CSV), retention settings, auditor role (read-only)
- Fields: Timestamp, actor, action, secret/env/provider, success, policy results, signature

### Tenancy & RBAC
- Workspaces/organizations, SSO/SCIM (Enterprise), domain verification
- Roles: Owner (full + billing), Admin (full), Operator (rotate only), Viewer (read), Auditor (logs only)
- Permissions: `rotate`, `approve`, `view`, `audit`, `policy`, `workspace`

### Connector Orchestration
- Reuse existing Birch connectors, policy integration, enhanced error handling
- Features: Update secrets, trigger redeploys, status checks, batch ops, rollback, dry-run

## Value Proposition

**Hosted Credentials:** Zero credential management, convenience, policy/governance, team collaboration, dashboard

**Customer-Managed:** Policy/governance, zero infrastructure (credentials stay in customer KMS), team collaboration, enterprise features, security

**Self-Hosted:** Free unlimited, full control, CLI-only, can migrate to SaaS

## Pricing

| Tier | Price | Rotations | Features |
|------|-------|-----------|----------|
| Free | $0 | Self-hosted: unlimited<br>SaaS: 100/mo | Basic policies, 1 workspace, email alerts |
| Starter | $19/mo | 1K/mo | Policies, maintenance windows, Slack alerts, 2 hosted providers, 5 team members |
| Pro | $49/mo | 10K/mo | All policies, approvals, webhooks, unlimited providers, 3 workspaces, 20 members |
| Enterprise | Custom | Unlimited | All modes, SSO/SCIM, customer-managed keys, compliance, unlimited workspaces, SLA |

**Overage:** Free = hard stop; Paid = $0.01/rotation; Enterprise = included

## Repository Structure

```
birch/
├── src/                    # CLI/daemon (existing)
├── packages/client/        # SDK (existing)
├── services/
│   ├── api/               # SaaS backend (Rust)
│   │   ├── supabase/     # Supabase integration
│   │   ├── auth/         # API keys, custom logic
│   │   ├── vault/        # Credential vault
│   │   ├── credentials/  # Credential resolution (resolver, kms, oauth, cache)
│   │   ├── policy/       # Policy engine (engine, evaluator, approval)
│   │   ├── orchestration/# Connector orchestration
│   │   ├── audit/        # Audit logging
│   │   ├── alerts/       # Alerting (slack, email, webhook)
│   │   └── workspace/    # Multi-tenancy (models, rbac)
│   └── web/              # Dashboard (Next.js + Supabase client)
├── supabase/             # Migrations, functions, config
└── docs/                 # Documentation (existing)
```

**Principles:** Everything OSS, monorepo, transparent, self-hostable

## Database Schema (Supabase PostgreSQL)

**Note:** Supabase manages `auth.users`. Custom tables reference `auth.users(id)`.

**Core Tables:**
- `workspaces` - Organizations with plan tier
- `workspace_members` - Links `auth.users` to workspaces with roles
- `provider_configs` - Per-provider credential mode and config (JSONB)
- `credentials` - Encrypted vault (hosted mode), customer KMS key ID (Enterprise)
- `oauth_tokens` - Encrypted refresh tokens, cached access tokens
- `policies` - Policy rules (JSONB), environment/provider/secret pattern matching
- `approval_requests` - Two-person approval workflows with timeout
- `audit_logs` - Immutable signed logs with metadata (JSONB)
- `api_keys` - Hashed keys for programmatic access
- `rotation_metering` - Daily rotation counts for billing

**RLS:** Enable on all tables. Policies enforce workspace isolation via `workspace_id` checks using `auth.uid()`.

**Indexes:** On `workspace_id`, `user_id`, `created_at` for common queries.

## Implementation Phases

### Phase 1: Credential System (MVP) - 4-6 weeks
**Goal:** Basic SaaS with hosted credentials

**Components:**
1. Supabase setup: Project init, migrations, RLS policies, Rust client integration
2. Credential vault: Encrypted storage, envelope encryption, CRUD operations
3. Credential resolver: Mode detection, hosted mode resolution, caching (Redis)
4. Provider config: Per-provider mode, API endpoints, validation
5. Multi-tenancy: Workspace model, RBAC (owner/admin/viewer), Supabase Auth integration, API keys

**Success:** Users sign up/login, create workspaces, configure providers, store/retrieve encrypted credentials, rotate secrets, RLS enforces isolation

### Phase 2: Policy & Orchestration - 3-4 weeks
**Goal:** Policy engine and enhanced orchestration

**Components:**
1. Policy engine: Thresholds, maintenance windows, evaluation logic
2. Connector orchestration: Reuse Birch connectors, policy integration, error handling
3. Basic alerts: Email, Slack (basic)

**Success:** Policies created/evaluated, rotations respect thresholds/windows, alerts working

### Phase 3: Customer-Managed Credentials - 4-5 weeks
**Goal:** KMS and OAuth support

**Components:**
1. KMS integration: AWS/GCP/Azure clients, credential caching (TTL)
2. OAuth flow: Provider-specific handlers, token refresh, health monitoring
3. Enhanced resolver: Multi-mode support, retry logic, circuit breaker, fallbacks

**Success:** KMS/OAuth modes configurable, credentials resolve correctly, failures handled gracefully

### Phase 4: Approval Workflows - 2-3 weeks
**Goal:** Two-person approval system

**Components:**
1. Approval system: Request creation, state machine, timeout handling, notifications
2. Enhanced alerts: Approval notifications, escalation rules

**Success:** Approval workflows created, approvers notified, rotations blocked until approval, timeouts work

### Phase 5: Enterprise Features - 4-6 weeks
**Goal:** SSO, SCIM, advanced audit

**Components:**
1. SSO/SCIM: Supabase SSO (OIDC/SAML), domain verification, SCIM provisioning
2. Enhanced audit: Export (JSON/CSV), retention settings, advanced filtering, compliance reporting
3. Customer-managed keys: Key management UI, rotation support

**Success:** SSO login, SCIM provisioning, audit exports, customer-managed keys working

### Phase 6: Dashboard - 6-8 weeks
**Goal:** Web UI for all features

**Components:**
1. Dashboard: Credential config UI, policy management, approval workflows, audit viewer, usage analytics, team management

**Success:** All features accessible via UI, intuitive UX, responsive design, real-time updates

## Security

### Credential Vault
- **Encryption:** Envelope encryption (customer KMS key → vault key), separate keys per workspace
- **Access:** RLS for isolation, RBAC, audit all access
- **Keys:** Rotation strategy, secure storage (HSM Enterprise), escrow for DR

### KMS Integration
- **Handling:** Never log decrypted values, short-lived cache (5-15 min), secure eviction
- **Failures:** Retry with backoff, circuit breaker, graceful degradation
- **Logging:** Log requests (workspace/provider/timestamp), no credential values

### OAuth Tokens
- **Storage:** Encrypted refresh tokens, separate from app DB, secure key management
- **Refresh:** Automatic before expiration, handle token rotation, graceful revocation handling
- **Health:** Monitor expiration, alert on failures, re-auth flow

### Multi-Tenancy
- **Isolation:** Database-level (RLS), API-level validation, no cross-workspace leakage
- **RBAC:** Resource-level permissions, least privilege, regular audits
- **API:** Key authentication, per-workspace rate limiting, request signing (Enterprise)

## Operations

### Infrastructure
- **Database/Auth:** Supabase (managed PostgreSQL, Auth, Realtime, Storage)
- **Caching:** Redis (credential cache, rate limiting)
- **Queue:** Supabase Edge Functions or RabbitMQ/AWS SQS (background jobs, DLQ)
- **Monitoring:** Supabase dashboard, Prometheus, Loki/ELK, Sentry, uptime monitoring

### Scaling
- **Horizontal:** Stateless API servers, load balancer, connection pooling
- **Cache:** Redis Cluster, invalidation strategy, TTL management
- **Rate Limiting:** Per-workspace/user, global limits

### Disaster Recovery
- **Backups:** Daily encrypted backups, point-in-time recovery
- **HA:** Multi-region (Enterprise), replication, failover procedures
- **Incidents:** Runbooks, escalation, post-mortems

## Success Metrics

**Product:** Workspaces/users, rotations/month, provider distribution, DAU, feature usage, churn, upgrade rate

**Technical:** API latency (p50/p95/p99), credential resolution latency, DB performance, uptime (99.9%), error rate, failed rotation rate

**Security:** Security incidents, audit completeness, credential exposure (zero tolerance)

## Open Questions

1. **Per-Request Mode:** Encryption method, who encrypts, key management
2. **OAuth Refresh:** Provider-specific quirks (token rotation, expiration windows, rate limits)
3. **Policy Conflicts:** Evaluation order, conflict resolution strategy
4. **Billing:** Count successful only or failures? Per-workspace or per-user? Overage handling?

## Next Steps

1. Validate concept with potential users
2. Prototype minimal credential vault and resolver
3. Architecture review (security, scalability)
4. Begin Phase 1 (MVP credential system)
