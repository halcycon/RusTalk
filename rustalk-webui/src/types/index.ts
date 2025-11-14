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

// Call log types
export interface CallLog {
  id: string;
  call_id: string;
  from_user: string;
  from_domain: string;
  to_user: string;
  to_domain: string;
  start_time: number;
  end_time?: number;
  duration_seconds?: number;
  status: string;
  termination_reason?: string;
  a_leg_codec?: string;
  b_leg_codec?: string;
  recording_path?: string;
  cost?: number;
}

export interface ChargeItem {
  description: string;
  rate: number;
  quantity: number;
  unit: string;
  amount: number;
}

export interface CallLogDetail {
  id: string;
  call_id: string;
  from_user: string;
  from_domain: string;
  to_user: string;
  to_domain: string;
  start_time: number;
  end_time?: number;
  duration_seconds?: number;
  status: string;
  termination_reason?: string;
  a_leg_codec?: string;
  b_leg_codec?: string;
  recording_path?: string;
  cost?: number;
  sip_call_id: string;
  from_tag?: string;
  to_tag?: string;
  charge_breakdown?: ChargeItem[];
  total_cost?: number;
}

export interface CallLogList {
  logs: CallLog[];
  total: number;
  page: number;
  per_page: number;
}

export interface RateCard {
  id: string;
  name: string;
  description?: string;
  prefix: string;
  rate_per_minute: number;
  connection_fee: number;
  minimum_charge_seconds: number;
  billing_increment_seconds: number;
  currency: string;
  effective_date: number;
  end_date?: number;
  active: boolean;
}

export interface RateListResponse {
  rates: RateCard[];
  total: number;
}

export interface RateImportRequest {
  format: 'json' | 'csv';
  data: string;
  overwrite: boolean;
}

export interface RateImportResponse {
  success: boolean;
  imported_count: number;
  errors: string[];
}

export interface CallLogExportRequest {
  format: 'json' | 'csv' | 'pdf';
  start_date?: number;
  end_date?: number;
  include_charges: boolean;
}

// Codec types
export interface Codec {
  name: string;
  payload_type: number;
  clock_rate: number;
  channels: number;
  description: string;
  enabled: boolean;
  is_standard: boolean;
  priority?: number;
}

export interface CodecListResponse {
  codecs: Codec[];
  total: number;
}

export interface CodecUpdateRequest {
  name: string;
  enabled: boolean;
}

export interface CodecAddRequest {
  name: string;
  payload_type: number;
  clock_rate: number;
  channels: number;
  description: string;
}

export interface CodecRemoveRequest {
  name: string;
}

export interface CodecReorderRequest {
  from_index: number;
  to_index: number;
}

// DID types
export interface Did {
  id: string;
  number: string;
  description?: string;
  destination: string;
  enabled: boolean;
  priority: number;
}

export interface DidListResponse {
  dids: Did[];
  total: number;
}

// Extension types
export interface Extension {
  id: string;
  extension: string;
  display_name: string;
  password: string;
  enabled: boolean;
  voicemail_enabled: boolean;
  priority: number;
}

export interface ExtensionListResponse {
  extensions: Extension[];
  total: number;
}

// Trunk types
export interface Trunk {
  id: string;
  name: string;
  description?: string;
  host: string;
  port: number;
  username?: string;
  password?: string;
  enabled: boolean;
  priority: number;
}

export interface TrunkListResponse {
  trunks: Trunk[];
  total: number;
}

// Ring Group types
export type RingStrategy = 'simultaneous' | 'sequential' | 'roundrobin';

export interface RingGroup {
  id: string;
  name: string;
  description?: string;
  extensions: string[];
  strategy: RingStrategy;
  timeout_seconds: number;
  enabled: boolean;
  priority: number;
}

export interface RingGroupListResponse {
  ring_groups: RingGroup[];
  total: number;
}

// Route types
export interface Route {
  id: string;
  name: string;
  description?: string;
  pattern: string;
  destination: string;
  enabled: boolean;
  priority: number;
}

export interface RouteListResponse {
  routes: Route[];
  total: number;
}

// SIP Profile types
export interface SipProfile {
  id: string;
  name: string;
  description?: string;
  bind_address: string;
  bind_port: number;
  domain: string;
  enabled: boolean;
  priority: number;
}

export interface SipProfileListResponse {
  sip_profiles: SipProfile[];
  total: number;
}

// Common reorder request
export interface ReorderRequest {
  from_index: number;
  to_index: number;
}
