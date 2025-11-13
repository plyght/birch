import { autoDetectConfig, setConfig } from './config';
import { installFetchInterceptor } from './interceptors/fetch';
import { installAxiosInterceptor } from './interceptors/axios';

async function initialize() {
  try {
    const config = await autoDetectConfig();
    setConfig(config);

    if (config.enabled) {
      installFetchInterceptor();
      installAxiosInterceptor();
      
      if (config.debug) {
        console.log('[Birch] Auto-rotation initialized', {
          environment: config.environment,
          service: config.service,
          daemonUrl: config.daemonUrl
        });
      }
    } else {
      if (config.debug) {
        console.warn('[Birch] Daemon not available, auto-rotation disabled');
      }
    }
  } catch (error) {
    console.error('[Birch] Failed to initialize:', error);
  }
}

initialize();

export { configureBirch } from './config';
export type { BirchConfig, ConfigureOptions } from './config';
export type { RotateResult, PoolStatus, RotationInfo } from './types';

