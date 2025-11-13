import { getConfig } from './config';
import { RotateResult } from './types';

declare const globalThis: {
  __birch_original_fetch?: typeof fetch;
};

function getOriginalFetch(): typeof fetch {
  return globalThis.__birch_original_fetch || fetch;
}

export class DaemonClient {
  async rotate(secretName: string): Promise<RotateResult> {
    const config = getConfig();
    
    if (!config.enabled) {
      if (config.debug) {
        console.log('[Birch] Daemon not available, skipping rotation');
      }
      return { success: false, message: 'Daemon not available' };
    }

    try {
      const originalFetch = getOriginalFetch();
      const response = await originalFetch(`${config.daemonUrl}/rotate`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          secret_name: secretName,
          env: config.environment,
          service: config.service
        }),
        signal: AbortSignal.timeout(10000)
      });

      if (!response.ok) {
        const text = await response.text();
        if (config.debug) {
          console.error(`[Birch] Rotation failed: ${response.status} - ${text}`);
        }
        return { 
          success: false, 
          message: `Daemon returned ${response.status}: ${text}` 
        };
      }

      const data = await response.json() as RotateResult;
      
      if (config.debug) {
        console.log('[Birch] Rotation response:', {
          success: data.success,
          has_new_value: !!data.new_value,
          pool_status: data.pool_status
        });
      }
      
      return data;
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      if (config.debug) {
        console.error('[Birch] Daemon unreachable:', message);
      }
      return { 
        success: false, 
        message: `Daemon unreachable: ${message}` 
      };
    }
  }

  async checkHealth(): Promise<boolean> {
    try {
      const config = getConfig();
      const originalFetch = getOriginalFetch();
      const response = await originalFetch(`${config.daemonUrl}/health`, {
        signal: AbortSignal.timeout(2000)
      });
      return response.ok;
    } catch {
      return false;
    }
  }
}

export const daemonClient = new DaemonClient();

