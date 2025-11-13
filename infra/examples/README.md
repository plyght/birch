# Birch Examples

Example applications demonstrating automatic API key rotation with Birch.

## Directory Structure

```
examples/
├── rust/          Rust examples
└── typescript/    TypeScript/Node.js examples
```

## Rust Examples

### App Signal Hook

Demonstrates manual rotation signal integration from Rust applications.

**Location**: `rust/app_signal_hook.rs`

**Prerequisites**:
- Rust toolchain
- Birch daemon running

**Run**:
```bash
cd /path/to/birch
cargo run --example app_signal_hook
```

**Functionality**:
- Simulates rate limit detection (HTTP 429)
- Sends rotation request to daemon
- Displays response

## TypeScript Examples

All TypeScript examples use the `@inaplight/birch-client` SDK for automatic rotation.

### CLI Script

Standalone CLI script with automatic rotation.

**Location**: `typescript/cli-script/`

**Prerequisites**:
- Bun or Node.js
- Birch daemon running
- API keys configured

**Setup**:
```bash
cd examples/typescript/cli-script
bun install @inaplight/birch-client
```

**Run**:
```bash
export TIKTOK_API_KEY="your-key"
export OPENAI_API_KEY="your-key"
bun run script.ts
```

**Functionality**:
- Zero-config setup via `@inaplight/birch-client/auto`
- Automatic key rotation on 429 responses
- Works with fetch, axios, and other HTTP clients

### Express API

REST API server with automatic key rotation.

**Location**: `typescript/express-api/`

**Prerequisites**:
- Bun or Node.js
- Birch daemon running
- API keys configured

**Setup**:
```bash
cd examples/typescript/express-api
bun install @inaplight/birch-client express
bun install -D @types/express
```

**Run**:
```bash
export TWITTER_API_KEY="your-key"
export TIKTOK_API_KEY="your-key"
bun run server.ts
```

**Test**:
```bash
curl http://localhost:3000/tweets
curl http://localhost:3000/tiktok
```

**Functionality**:
- Single import enables automatic rotation
- REST endpoints proxy external APIs
- Transparent rate limit handling

### Next.js App

Next.js App Router application with automatic rotation.

**Location**: `typescript/nextjs-app/`

**Prerequisites**:
- Bun or Node.js
- Birch daemon running
- API keys configured

**Setup**:
```bash
cd examples/typescript/nextjs-app
bun install @inaplight/birch-client next react react-dom
```

**Configure**:
```bash
echo "TIKTOK_API_KEY=your-key" > .env.local
```

**Run**:
```bash
bun run next dev
```

**Test**:
```bash
curl http://localhost:3000/api/tiktok
```

**Functionality**:
- Root layout imports `@inaplight/birch-client/auto`
- API routes fetch from external APIs
- Automatic rotation on rate limits

## Common Setup

### 1. Start the Birch Daemon

All examples require the Birch daemon to be running:

```bash
birch daemon start
```

Verify it's running:
```bash
birch daemon status
# or
curl http://localhost:9123/health
```

### 2. Set Up Key Pools

For automatic rotation to work, set up key pools:

```bash
birch pool init TIKTOK_API_KEY --keys "key1,key2,key3"
birch pool init TWITTER_API_KEY --keys "key1,key2,key3"
birch pool init OPENAI_API_KEY --keys "key1,key2,key3"
```

### 3. Configure Environment Variables

Each example needs the appropriate API keys set as environment variables:

```bash
export TIKTOK_API_KEY="your-first-key"
export TWITTER_API_KEY="your-first-key"
export OPENAI_API_KEY="your-first-key"
```

## How It Works

### Rust Examples

Manual integration approach:
1. Application detects rotation trigger (e.g., 429 response)
2. Sends HTTP POST to `http://localhost:9123/rotate`
3. Daemon queues rotation and returns immediately
4. Key updates occur asynchronously

### TypeScript Examples

Automatic integration via SDK:
1. Import `@inaplight/birch-client/auto` at entry point
2. SDK intercepts HTTP requests (fetch, axios, etc.)
3. Automatic 429 detection
4. Immediate rotation and retry
5. Asynchronous production secret updates

## Troubleshooting

### "Daemon not available"

Ensure the daemon is running:
```bash
birch daemon start
curl http://localhost:9123/health
```

### "Could not detect secret name"

Enable debug mode to see what the SDK detects:
```bash
export BIRCH_DEBUG=true
bun run script.ts
```

The SDK looks for tokens in environment variables ending with:
- `API_KEY`
- `TOKEN`
- `SECRET`

### "Pool exhausted"

Check your pool status:
```bash
birch pool status TIKTOK_API_KEY
```

Add more keys if needed:
```bash
birch pool add TIKTOK_API_KEY --key "new-key"
```

## Further Reading

- [Birch CLI Documentation](../docs/content/docs/cli-reference.mdx)
- [SDK Documentation](../docs/content/docs/sdk/)
- [App-Signal Rotation](../docs/content/docs/usage/app-signals.mdx)
- [Key Pools](../docs/content/docs/usage/key-pools.mdx)

