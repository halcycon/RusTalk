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
  acme?: {
    enabled: boolean;
    email: string;
    domains: string[];
    cert_dir: string;
    account_dir: string;
    use_staging: boolean;
    http_challenge_port: number;
    challenge_type: string;
    auto_renew_days: number;
  };
}

export interface HealthResponse {
  status: string;
  service: string;
  version: string;
}

// Certificate types
export interface CertificateInfo {
  domain: string;
  domains: string[];
  status: 'valid' | 'expiring_soon' | 'expired';
  expires_at: string;
  days_until_expiry: number;
  cert_path?: string;
  key_path?: string;
  serial?: string;
  needs_renewal: boolean;
}

export interface CertificateRequest {
  domains: string[];
  email: string;
  challenge_type: 'http-01' | 'dns-01';
  use_staging?: boolean;
}

export interface CertificateRenewal {
  domain: string;
}

export interface CertificateListResponse {
  certificates: CertificateInfo[];
  total: number;
}

export interface CertificateOperationResponse {
  success: boolean;
  message: string;
  domain?: string;
  domains?: string[];
  expires_at?: string;
  cert_path?: string;
  key_path?: string;
}
