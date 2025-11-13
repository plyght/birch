#!/usr/bin/env bun

import { spawn, type Subprocess } from "bun";
import { existsSync } from "fs";

const processes: Subprocess[] = [];
let isShuttingDown = false;

const colors = {
  red: "\x1b[0;31m",
  green: "\x1b[0;32m",
  yellow: "\x1b[1;33m",
  blue: "\x1b[0;34m",
  magenta: "\x1b[0;35m",
  cyan: "\x1b[0;36m",
  reset: "\x1b[0m",
};

function log(color: keyof typeof colors, prefix: string, message: string) {
  console.log(`${colors[color]}[${prefix}]${colors.reset} ${message}`);
}

async function checkDocker(): Promise<boolean> {
  try {
    const proc = spawn(["docker", "ps"], {
      stdout: "pipe",
      stderr: "pipe",
    });
    await proc.exited;
    return proc.exitCode === 0;
  } catch {
    return false;
  }
}

async function isRedisRunning(): Promise<boolean> {
  try {
    const proc = spawn(["docker", "ps", "--format", "{{.Names}}"], {
      stdout: "pipe",
      stderr: "pipe",
    });
    const text = await new Response(proc.stdout).text();
    return text.split("\n").some((name) => name === "birch-redis");
  } catch {
    return false;
  }
}

async function startRedis(): Promise<void> {
  if (await isRedisRunning()) {
    log("yellow", "REDIS", "Container already running");
    return;
  }

  log("blue", "REDIS", "Starting Redis container...");
  
  const proc = spawn([
    "docker",
    "run",
    "-d",
    "--name",
    "birch-redis",
    "-p",
    "6379:6379",
    "redis:alpine",
  ], {
    stdout: "pipe",
    stderr: "pipe",
  });

  await proc.exited;

  if (proc.exitCode !== 0) {
    const error = await new Response(proc.stderr).text();
    log("red", "REDIS", `Failed to start: ${error}`);
    throw new Error("Failed to start Redis. Is Docker running?");
  }

  log("green", "REDIS", "Started on port 6379");
}

async function stopRedis(): Promise<void> {
  log("blue", "REDIS", "Stopping container...");
  
  const stop = spawn(["docker", "stop", "birch-redis"], {
    stdout: "pipe",
    stderr: "pipe",
  });
  await stop.exited;

  const rm = spawn(["docker", "rm", "birch-redis"], {
    stdout: "pipe",
    stderr: "pipe",
  });
  await rm.exited;
}

async function parseEnvFile(path: string): Promise<Record<string, string>> {
  if (!existsSync(path)) {
    return {};
  }
  
  const text = await Bun.file(path).text();
  const env: Record<string, string> = {};
  
  for (const line of text.split("\n")) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#")) {
      continue;
    }
    
    const equalIndex = trimmed.indexOf("=");
    if (equalIndex === -1) {
      continue;
    }
    
    const key = trimmed.slice(0, equalIndex).trim();
    const value = trimmed.slice(equalIndex + 1).trim();
    
    if (key && value) {
      env[key] = value;
    }
  }
  
  return env;
}

async function validateEnvFiles(): Promise<void> {
  if (!existsSync("apps/api/.env")) {
    log("red", "ERROR", "apps/api/.env not found");
    console.log("Copy env.example to apps/api/.env and configure it");
    process.exit(1);
  }

  const apiEnv = await parseEnvFile("apps/api/.env");
  const requiredVars = ["DATABASE_URL", "REDIS_URL", "JWT_SECRET"];
  const missing: string[] = [];
  
  for (const varName of requiredVars) {
    if (!apiEnv[varName] || apiEnv[varName] === `your-${varName.toLowerCase().replace(/_/g, "-")}` || apiEnv[varName].includes("your-")) {
      missing.push(varName);
    }
  }
  
  if (missing.length > 0) {
    log("red", "ERROR", `Missing or unconfigured required environment variables: ${missing.join(", ")}`);
    console.log("\nPlease configure these in apps/api/.env:");
    for (const varName of missing) {
      console.log(`  - ${varName}: See env.example for details`);
    }
    process.exit(1);
  }

  if (!existsSync("apps/web/.env.local")) {
    log("yellow", "WARN", "apps/web/.env.local not found");
    console.log("Copy env.example to apps/web/.env.local and configure it");
  }
}

async function cleanup() {
  if (isShuttingDown) return;
  isShuttingDown = true;

  console.log("\n");
  log("yellow", "SHUTDOWN", "Stopping all services...");

  for (const proc of processes) {
    try {
      proc.kill();
    } catch (e) {
      console.error("Error killing process:", e);
    }
  }

  await stopRedis();
  
  log("green", "SHUTDOWN", "All services stopped");
  process.exit(0);
}

async function startAPI() {
  log("blue", "API", "Starting API server on http://localhost:3000");

  const apiEnv = await parseEnvFile("apps/api/.env");

  const proc = spawn(["cargo", "run", "--bin", "birch-api"], {
    cwd: "apps/api",
    stdout: "pipe",
    stderr: "pipe",
    env: {
      ...process.env,
      ...apiEnv,
    },
  });

  processes.push(proc);

  const reader = proc.stdout.getReader();
  const decoder = new TextDecoder();

  (async () => {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      const text = decoder.decode(value);
      text.split("\n").forEach((line) => {
        if (line.trim()) log("cyan", "API", line);
      });
    }
  })();

  const errReader = proc.stderr.getReader();
  (async () => {
    while (true) {
      const { done, value } = await errReader.read();
      if (done) break;
      const text = decoder.decode(value);
      text.split("\n").forEach((line) => {
        if (line.trim()) log("cyan", "API", line);
      });
    }
  })();

  proc.exited.then(() => {
    if (!isShuttingDown) {
      log("red", "API", "Process exited unexpectedly");
      cleanup();
    }
  });
}

async function startWeb() {
  log("green", "WEB", "Starting dashboard on http://localhost:3001");

  const proc = spawn(["bun", "dev"], {
    cwd: "apps/web",
    stdout: "pipe",
    stderr: "pipe",
    env: process.env,
  });

  processes.push(proc);

  const reader = proc.stdout.getReader();
  const decoder = new TextDecoder();

  (async () => {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      const text = decoder.decode(value);
      text.split("\n").forEach((line) => {
        if (line.trim()) log("magenta", "WEB", line);
      });
    }
  })();

  const errReader = proc.stderr.getReader();
  (async () => {
    while (true) {
      const { done, value } = await errReader.read();
      if (done) break;
      const text = decoder.decode(value);
      text.split("\n").forEach((line) => {
        if (line.trim()) log("magenta", "WEB", line);
      });
    }
  })();

  proc.exited.then(() => {
    if (!isShuttingDown) {
      log("red", "WEB", "Process exited unexpectedly");
      cleanup();
    }
  });
}

async function main() {
  console.log("Starting Birch Development Environment\n");

  if (!(await checkDocker())) {
    log("red", "ERROR", "Docker is not running");
    process.exit(1);
  }

  await startRedis();
  await validateEnvFiles();

  console.log("\nStarting services...");
  console.log("----------------------------------------\n");

  process.on("SIGINT", cleanup);
  process.on("SIGTERM", cleanup);
  process.on("exit", cleanup);

  await startAPI();
  await startWeb();

  await Promise.race(processes.map((p) => p.exited));
}

main().catch((error) => {
  log("red", "ERROR", error.message);
  cleanup();
});

