import { describe, it, expect, beforeEach } from 'bun:test';
import { EnvTracker } from '../src/env-tracker';

describe('EnvTracker', () => {
  let tracker: EnvTracker;

  beforeEach(() => {
    tracker = new EnvTracker();
    process.env.TIKTOK_API_KEY = 'sk_test_12345';
    process.env.TWITTER_API_KEY = 'xoxb-test-67890';
  });

  it('tracks requests with Authorization header', () => {
    const headers = {
      'Authorization': 'Bearer sk_test_12345'
    };

    tracker.trackRequest('https://api.tiktok.com/v1/videos', headers);
    
    const secretName = tracker.getSecretName('https://api.tiktok.com/v1/videos');
    expect(secretName).toBe('TIKTOK_API_KEY');
  });

  it('detects secret from token prefix', () => {
    const headers = {
      'Authorization': 'Bearer sk_tiktok_unknown'
    };

    tracker.trackRequest('https://api.tiktok.com/v1/videos', headers);
    
    const secretName = tracker.getSecretName('https://api.tiktok.com/v1/videos');
    expect(secretName).toBe('TIKTOK_API_KEY');
  });

  it('returns undefined for unknown URLs', () => {
    const secretName = tracker.getSecretName('https://unknown-api.com');
    expect(secretName).toBeUndefined();
  });

  it('clears tracked data', () => {
    const headers = {
      'Authorization': 'Bearer sk_test_12345'
    };

    tracker.trackRequest('https://api.tiktok.com/v1/videos', headers);
    tracker.clear();
    
    const secretName = tracker.getSecretName('https://api.tiktok.com/v1/videos');
    expect(secretName).toBeUndefined();
  });
});

