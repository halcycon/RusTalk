// Type definitions for RusTalk API

export interface CallInfo {
  id: string;
  from: string;
  to: string;
  status: 'ringing' | 'active' | 'ended' | 'failed';
  start_time: number;
  duration?: number;
}

export interface Stats {
  active_calls: number;
  total_calls_today: number;
  uptime_seconds: number;
  cpu_usage?: number;
  memory_usage?: number;
}

export interface Config {
  server?: {
    bind_address: string;
    bind_port: number;
    workers: number;
  };
  sip?: {
    domain: string;
    user_agent: string;
    max_forwards: number;
    session_expires: number;
  };
  transport?: {
    protocols: string[];
    udp_port: number;
    tcp_port: number;
    tls_port: number;
    tls_cert?: string;
    tls_key?: string;
  };
  teams?: {
    enabled: boolean;
    sbc_fqdn: string;
    mtls_cert?: string;
    mtls_key?: string;
    trunk_fqdn: string;
  };
  database?: {
    url: string;
    max_connections: number;
    min_connections: number;
  };
}

export interface HealthResponse {
  status: string;
  service: string;
  version: string;
}
