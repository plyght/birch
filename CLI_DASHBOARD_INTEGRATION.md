# CLI-Dashboard Integration Guide

## What Was Implemented

### 1. CLI OAuth Flow
- **Browser-Based Authentication**: `birch login` now opens your browser to authenticate
- **JWT Token Support**: CLI uses JWT tokens from Supabase instead of API keys
- **Callback Server**: CLI runs a local server (port 9124) to receive the auth token
- **Cross-Platform**: Works on macOS, Linux, and Windows

### 2. API JWT Authentication
- **Dual Auth Support**: API accepts both JWT tokens (from dashboard) and API keys (for backward compatibility)
- **Token Validation**: Validates Supabase JWT tokens using the JWT secret
- **User Mapping**: Extracts user_id from JWT and maps to workspaces

### 3. Dashboard UI/UX
- **Mathematical Paper Aesthetic**: Clean, minimal design with serif headings
- **Sidebar Navigation**: Persistent left sidebar for all core pages
- **Complete Pages**:
  - Dashboard (home with metrics)
  - Workspaces (list, create, detail)
  - Credentials (placeholder)
  - Policies (placeholder)
  - Approvals (placeholder)
  - Audit Logs (sample data)
  - Team (current user shown)
  - Settings (with API key management)
  - Login (dedicated page)
  - CLI OAuth (/auth/cli)

### 4. API Key Management
- **Create API Keys**: Generate API keys in the settings page
- **Copy Once**: Keys are only shown once after creation
- **Revoke Keys**: Revoke keys when no longer needed
- **List Keys**: View all API keys with status and last used date

## Testing the Integration

### Prerequisites
1. **Supabase Setup**:
   - JWT_SECRET configured in `apps/api/.env` (get from Supabase Project Settings > API > JWT Settings)
   - Database migrations applied
   - Supabase auth configured (GitHub OAuth optional)

2. **Services Running**:
   ```bash
   # Terminal 1: Redis
   docker run -d --name birch-redis -p 6379:6379 redis:alpine
   
   # Terminal 2: API
   cd apps/api
   export $(cat .env | grep -v '^#' | xargs) && cargo run --bin birch-api
   
   # Terminal 3: Dashboard
   cd apps/web
   bun dev
   ```

### Test Flow A: Browser OAuth (Recommended)

1. **Start CLI Login**:
   ```bash
   birch login http://localhost:3000
   ```

2. **Browser Opens Automatically** to `http://localhost:3001/auth/cli`
   - If not authenticated, redirects to login page
   - Login with GitHub or email/password
   - Returns to authorization page

3. **Authorize CLI**:
   - Click "Authorize" button
   - Browser redirects to `localhost:9124/auth/callback`
   - CLI receives token and displays success message

4. **Test CLI Commands**:
   ```bash
   # List workspaces
   birch workspace list
   
   # Create workspace
   birch workspace create "Production"
   
   # Select workspace
   birch workspace select <workspace-id>
   
   # Configure provider
   birch provider set vercel --mode hosted
   
   # List providers
   birch provider list
   ```

5. **Verify in Dashboard**:
   - Go to `http://localhost:3001/dashboard`
   - See workspaces created via CLI
   - Check audit logs for CLI activities

### Test Flow B: API Key (Legacy/Backup)

1. **Create API Key in Dashboard**:
   - Login to dashboard: `http://localhost:3001`
   - Go to Settings
   - Click "Create API Key"
   - Name it (e.g., "Test CLI")
   - Copy the key (shown only once!)

2. **Configure CLI**:
   ```bash
   birch login http://localhost:3000
   # Paste API key when prompted
   ```

3. **Test CLI Commands** (same as above)

### Verification Checklist

- [ ] CLI login opens browser
- [ ] Dashboard auth works (GitHub or email/password)
- [ ] CLI receives token after authorization
- [ ] `birch workspace list` returns workspaces
- [ ] Workspace created in CLI appears in dashboard
- [ ] Workspace created in dashboard appears in CLI
- [ ] API key creation works in dashboard
- [ ] API key works with CLI login
- [ ] Settings page shows API keys with status

## Troubleshooting

### CLI Login Fails
- **Error**: "Failed to bind to 127.0.0.1:9124"
  - **Fix**: Port 9124 is in use. Kill the process or wait a moment
  
- **Error**: "Authentication timeout"
  - **Fix**: You have 5 minutes to complete auth. Run `birch login` again

### Dashboard Connection Issues
- **Error**: "401 Unauthorized"
  - **Fix**: JWT_SECRET not configured in API. Check `apps/api/.env`
  
- **Error**: "Invalid JWT token"
  - **Fix**: Ensure JWT_SECRET matches your Supabase project's JWT secret

### API Errors
- **Error**: "DATABASE_URL must be set"
  - **Fix**: Run API with env vars: `export $(cat .env | grep -v '^#' | xargs) && cargo run`

- **Error**: "Connection refused"
  - **Fix**: Ensure Redis is running: `docker ps | grep redis`

## Environment Variables Reference

### API (`apps/api/.env`)
```bash
DATABASE_URL=postgresql://postgres:[password]@aws-1-us-east-1.pooler.supabase.com:6543/postgres
REDIS_URL=redis://localhost:6379
JWT_SECRET=<your-supabase-jwt-secret>
SUPABASE_URL=https://[project].supabase.co
SUPABASE_ANON_KEY=<your-anon-key>
SUPABASE_SERVICE_ROLE_KEY=<your-service-role-key>
VAULT_MASTER_KEY=<32-byte-hex-key>
```

### Dashboard (`apps/web/.env.local`)
```bash
NEXT_PUBLIC_SUPABASE_URL=https://[project].supabase.co
NEXT_PUBLIC_SUPABASE_ANON_KEY=<your-anon-key>
NEXT_PUBLIC_API_URL=http://localhost:3000
```

## How It Works

### OAuth Flow Diagram
```
┌─────────┐                    ┌───────────┐                ┌─────┐
│   CLI   │                    │ Dashboard │                │ API │
└────┬────┘                    └─────┬─────┘                └──┬──┘
     │                               │                         │
     │ 1. birch login                │                         │
     ├──────────────────────────────>│                         │
     │                               │                         │
     │ 2. Opens browser              │                         │
     │ /auth/cli?state=...          │                         │
     │                               │                         │
     │                          3. User logs in                │
     │                               │                         │
     │                          4. Generate JWT                │
     │                               │                         │
     │ 5. Redirect with token        │                         │
     │ localhost:9124/callback       │                         │
     │<──────────────────────────────┤                         │
     │                               │                         │
     │ 6. Store JWT token            │                         │
     │                               │                         │
     │ 7. API request with JWT       │                         │
     │ Authorization: Bearer <jwt>   │                         │
     ├───────────────────────────────┼────────────────────────>│
     │                               │                         │
     │                               │    8. Validate JWT      │
     │                               │    Extract user_id      │
     │                               │                         │
     │ 9. Response                   │                         │
     │<──────────────────────────────┼─────────────────────────┤
```

### Authentication Methods

1. **JWT Tokens** (Primary):
   - Generated by Supabase Auth
   - Used by dashboard and CLI (via browser OAuth)
   - Validated using JWT_SECRET
   - Short-lived, automatically refreshed

2. **API Keys** (Legacy/Backup):
   - Created in dashboard settings
   - Used by CLI and programmatic access
   - Stored as hashed values in database
   - Can be revoked individually

## Next Steps

1. **Test the integration** using the flows above
2. **Report any issues** you encounter
3. **Optional**: Configure GitHub OAuth in Supabase for social login
4. **Optional**: Add more provider configurations in credentials page
5. **Optional**: Implement full CRUD for policies and approval workflows

## Features Not Yet Implemented

The following pages have placeholder UI but no backend integration:
- Credentials management (add/edit credentials)
- Policy creation and editing
- Approval workflow (approve/reject)
- Team member invitations
- Detailed audit log filtering

These can be implemented as needed following the same patterns used in workspaces and settings.

