import { describe, it, expect, beforeEach } from 'bun:test';
import { detectService, autoDetectConfig } from '../src/config';

describe('Config', () => {
  describe('detectService', () => {
    beforeEach(() => {
      delete process.env.VERCEL;
      delete process.env.NETLIFY_SITE_ID;
      delete process.env.RENDER_SERVICE_ID;
      delete process.env.CF_PAGES;
      delete process.env.FLY_APP_NAME;
    });

    it('detects Vercel', () => {
      process.env.VERCEL = '1';
      expect(detectService()).toBe('vercel');
    });

    it('detects Netlify', () => {
      process.env.NETLIFY_SITE_ID = 'test-site';
      expect(detectService()).toBe('netlify');
    });

    it('detects Render', () => {
      process.env.RENDER_SERVICE_ID = 'test-service';
      expect(detectService()).toBe('render');
    });

    it('detects Cloudflare', () => {
      process.env.CF_PAGES = '1';
      expect(detectService()).toBe('cloudflare');
    });

    it('detects Fly.io', () => {
      process.env.FLY_APP_NAME = 'test-app';
      expect(detectService()).toBe('fly');
    });

    it('returns undefined when no service detected', () => {
      expect(detectService()).toBeUndefined();
    });
  });

  describe('autoDetectConfig', () => {
    it('uses environment variables', async () => {
      process.env.BIRCH_DAEMON_URL = 'http://localhost:9999';
      process.env.BIRCH_ENV = 'production';
      
      const config = await autoDetectConfig();
      
      expect(config.daemonUrl).toBe('http://localhost:9999');
      expect(config.environment).toBe('production');
    });

    it('falls back to defaults', async () => {
      delete process.env.BIRCH_DAEMON_URL;
      delete process.env.BIRCH_ENV;
      delete process.env.NODE_ENV;
      
      const config = await autoDetectConfig();
      
      expect(config.daemonUrl).toBe('http://localhost:9123');
      expect(config.environment).toBe('dev');
    });
  });
});

