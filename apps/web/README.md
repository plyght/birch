# Birch Dashboard

Web dashboard for Birch SaaS built with Next.js and Supabase.

## Features

- **Workspace Management**: Create and manage organizations
- **Credential Configuration**: Set up credential modes per provider
- **Policy Management**: Define rotation policies and thresholds
- **Approval Workflows**: Two-person approval system
- **Audit Logging**: View and export security audit trails
- **Team Management**: Invite and manage team members with RBAC
- **Usage Analytics**: Monitor rotation metrics and billing

## Getting Started

### Prerequisites

- Node.js 18+ or Bun
- Supabase account
- Birch API running

### Environment Variables

Copy `.env.example` to `.env.local` and configure:

```bash
NEXT_PUBLIC_SUPABASE_URL=https://your-project.supabase.co
NEXT_PUBLIC_SUPABASE_ANON_KEY=your-anon-key
NEXT_PUBLIC_API_URL=http://localhost:3000
```

### Development

```bash
bun install
bun dev
```

Open [http://localhost:3001](http://localhost:3001) in your browser.

### Build

```bash
bun build
bun start
```

## Project Structure

```
src/
├── app/              # Next.js app router pages
├── components/       # React components
│   ├── workspaces/  # Workspace management
│   ├── credentials/ # Credential configuration
│   ├── policies/    # Policy management
│   ├── approvals/   # Approval workflows
│   ├── audit/       # Audit logging
│   └── team/        # Team management
├── lib/             # Utilities and configurations
│   ├── supabase.ts # Supabase client
│   └── api.ts      # API client
└── types/           # TypeScript types
```

## Authentication

The dashboard uses Supabase Auth for authentication:

- Email/password authentication
- OAuth providers (GitHub, GitLab, etc.)
- SSO (Enterprise)
- SCIM provisioning (Enterprise)

## Real-time Updates

Uses Supabase Realtime for live updates:

- Approval request notifications
- Rotation status updates
- Team member changes
- Policy violations

## Deployment

### Vercel

```bash
vercel deploy
```

### Docker

```bash
docker build -t birch-dashboard .
docker run -p 3001:3001 birch-dashboard
```

## License

MIT - See LICENSE file in repository root

