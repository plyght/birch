import { getConfig } from '../config';
import { envTracker } from '../env-tracker';
import { daemonClient } from '../daemon-client';

let isAxiosInterceptorInstalled = false;

function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

export function installAxiosInterceptor(): void {
  if (isAxiosInterceptorInstalled) {
    return;
  }

  let axios: any;
  
  try {
    if (typeof require !== 'undefined') {
      axios = require('axios');
    }
  } catch (e) {
    if (getConfig().debug) {
      console.log('[Birch] axios not found, skipping axios interceptor');
    }
    return;
  }

  if (!axios || !axios.interceptors) {
    if (getConfig().debug) {
      console.log('[Birch] axios.interceptors not available');
    }
    return;
  }

  axios.interceptors.request.use(
    (config: any) => {
      if (config.url && config.headers) {
        envTracker.trackRequest(config.url, config.headers);
      }
      return config;
    },
    (error: any) => Promise.reject(error)
  );

  axios.interceptors.response.use(
    (response: any) => response,
    async (error: any) => {
      const config = getConfig();
      
      if (error.response?.status === 429 && error.config) {
        const url = error.config.url;
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

            error.config.headers['Authorization'] = `Bearer ${result.new_value}`;
            
            return axios.request(error.config);
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

      return Promise.reject(error);
    }
  );

  isAxiosInterceptorInstalled = true;

  if (getConfig().debug) {
    console.log('[Birch] Axios interceptor installed');
  }
}

