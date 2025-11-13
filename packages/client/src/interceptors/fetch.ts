import { getConfig } from '../config';
import { envTracker } from '../env-tracker';
import { daemonClient } from '../daemon-client';

declare const globalThis: {
  fetch: typeof fetch;
  __birch_original_fetch?: typeof fetch;
  __birch_fetch_installed?: boolean;
};

function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

export function installFetchInterceptor(): void {
  if (globalThis.__birch_fetch_installed) {
    return;
  }

  const originalFetch = globalThis.fetch;
  globalThis.__birch_original_fetch = originalFetch;

  globalThis.fetch = async function birchFetch(
    input: RequestInfo | URL,
    init?: RequestInit
  ): Promise<Response> {
    const url = typeof input === 'string' ? input : input instanceof URL ? input.href : input.url;
    const config = getConfig();

    envTracker.trackRequest(url, init?.headers);

    const response = await originalFetch(input, init);

    if (response.status === 429) {
      const secretName = envTracker.getSecretName(url);

      if (secretName) {
        if (config.debug) {
          console.log(`[Birch] Rate limit hit (429) for ${secretName}, triggering rotation...`);
        }

        const result = await daemonClient.rotate(secretName);

        if (result.success && result.new_value) {
          if (config.debug) {
            console.log(`[Birch] Rotation successful, retrying with new key ***${result.new_value.slice(-4)}`);
          }

          await sleep(1000);

          const newHeaders = new Headers(init?.headers);
          newHeaders.set('Authorization', `Bearer ${result.new_value}`);

          return originalFetch(input, {
            ...init,
            headers: newHeaders
          });
        } else {
          if (config.debug) {
            console.warn('[Birch] Rotation failed or no new value, returning 429');
          }
        }
      } else {
        if (config.debug) {
          console.warn('[Birch] Could not detect secret name for URL:', url);
        }
      }
    }

    return response;
  };

  globalThis.__birch_fetch_installed = true;

  if (getConfig().debug) {
    console.log('[Birch] Fetch interceptor installed');
  }
}

export function uninstallFetchInterceptor(): void {
  if (globalThis.__birch_original_fetch) {
    globalThis.fetch = globalThis.__birch_original_fetch;
    delete globalThis.__birch_original_fetch;
    delete globalThis.__birch_fetch_installed;
    
    if (getConfig().debug) {
      console.log('[Birch] Fetch interceptor uninstalled');
    }
  }
}

