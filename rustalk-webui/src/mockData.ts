// Mock data for development and testing
import type { CallInfo, Stats, HealthResponse } from './types';

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
