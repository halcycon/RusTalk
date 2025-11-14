// Mock data for development and testing
import type { CallInfo, Stats, HealthResponse, TeamsConfig, TeamsStatusResponse } from './types';

export const mockStats: Stats = {
  active_calls: 42,
  total_calls_today: 328,
  uptime_seconds: 86420, // ~24 hours
  cpu_usage: 23.5,
  memory_usage: 45.8,
};

export const mockHealth: HealthResponse = {
  status: 'healthy',
  service: 'RusTalk',
  version: '0.1.0',
};

export const mockCalls: CallInfo[] = [
  {
    id: 'call-001-abc-def',
    from: '+1-555-0123',
    to: '+1-555-9876',
    status: 'active',
    start_time: Math.floor(Date.now() / 1000) - 120,
    duration: 120,
  },
  {
    id: 'call-002-ghi-jkl',
    from: '+1-555-0456',
    to: '+1-555-5432',
    status: 'ringing',
    start_time: Math.floor(Date.now() / 1000) - 5,
    duration: 5,
  },
  {
    id: 'call-003-mno-pqr',
    from: '+1-555-0789',
    to: '+1-555-2109',
    status: 'active',
    start_time: Math.floor(Date.now() / 1000) - 300,
    duration: 300,
  },
];

export const mockTeamsConfig: TeamsConfig = {
  sbc_fqdn: 'sbc.example.com',
  tenant_domain: 'contoso.onmicrosoft.com',
  mtls_cert_path: '/etc/rustalk/teams-cert.pem',
  mtls_key_path: '/etc/rustalk/teams-key.pem',
  sip_proxies: [
    'sip.pstnhub.microsoft.com',
    'sip2.pstnhub.microsoft.com',
    'sip3.pstnhub.microsoft.com',
  ],
  options_ping_enabled: true,
  options_ping_interval: 60,
};

export const mockTeamsStatus: TeamsStatusResponse = {
  enabled: true,
  config: mockTeamsConfig,
  health_status: [
    {
      proxy: 'sip.pstnhub.microsoft.com',
      status: 'healthy',
      last_check: Date.now() - 30000,
      response_time_ms: 45,
    },
    {
      proxy: 'sip2.pstnhub.microsoft.com',
      status: 'healthy',
      last_check: Date.now() - 30000,
      response_time_ms: 52,
    },
    {
      proxy: 'sip3.pstnhub.microsoft.com',
      status: 'degraded',
      last_check: Date.now() - 30000,
      response_time_ms: 250,
    },
  ],
  total_calls: 1523,
  active_calls: 12,
};
