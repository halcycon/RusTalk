// API client for RusTalk REST API

import axios from 'axios';
import type { 
  CallInfo, 
  Stats, 
  Config, 
  HealthResponse,
  CertificateInfo,
  CertificateRequest,
  CertificateRenewal,
  CertificateListResponse,
  CertificateOperationResponse,
  CallLogDetail,
  CallLogList,
  RateCard,
  RateListResponse,
  RateImportRequest,
  RateImportResponse,
  CallLogExportRequest,
} from '../types';

const api = axios.create({
  baseURL: '/api/v1',
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
});

export const healthCheck = async (): Promise<HealthResponse> => {
  const response = await axios.get('/health');
  return response.data;
};

export const getCalls = async (): Promise<CallInfo[]> => {
  const response = await api.get('/calls');
  return response.data.calls || [];
};

export const getCall = async (id: string): Promise<CallInfo> => {
  const response = await api.get(`/calls/${id}`);
  return response.data;
};

export const getStats = async (): Promise<Stats> => {
  const response = await api.get('/stats');
  return response.data;
};

export const getConfig = async (): Promise<Config> => {
  const response = await api.get('/config');
  return response.data;
};

export const updateConfig = async (config: Partial<Config>): Promise<void> => {
  await api.post('/config', config);
};

// Certificate management API calls
export const listCertificates = async (): Promise<CertificateListResponse> => {
  const response = await api.get('/certificates');
  return response.data;
};

export const getCertificateStatus = async (domain: string): Promise<CertificateInfo> => {
  const response = await api.get(`/certificates/${domain}`);
  return response.data;
};

export const requestCertificate = async (request: CertificateRequest): Promise<CertificateOperationResponse> => {
  const response = await api.post('/certificates/request', request);
  return response.data;
};

export const renewCertificate = async (renewal: CertificateRenewal): Promise<CertificateOperationResponse> => {
  const response = await api.post('/certificates/renew', renewal);
  return response.data;
};

// Call logs API calls
export const getCallLogs = async (params?: {
  page?: number;
  per_page?: number;
  start_date?: number;
  end_date?: number;
  status?: string;
}): Promise<CallLogList> => {
  const response = await api.get('/call-logs', { params });
  return response.data;
};

export const getCallLog = async (id: string): Promise<CallLogDetail> => {
  const response = await api.get(`/call-logs/${id}`);
  return response.data;
};

export const exportCallLogs = async (request: CallLogExportRequest): Promise<any> => {
  const response = await api.post('/call-logs/export', request);
  return response.data;
};

// Rates API calls
export const getRates = async (): Promise<RateListResponse> => {
  const response = await api.get('/rates');
  return response.data;
};

export const importRates = async (request: RateImportRequest): Promise<RateImportResponse> => {
  const response = await api.post('/rates/import', request);
  return response.data;
};

export const saveRate = async (rate: RateCard): Promise<{ success: boolean; id: string; message: string }> => {
  const response = await api.post('/rates', rate);
  return response.data;
};

export const deleteRate = async (id: string): Promise<{ success: boolean; message: string }> => {
  const response = await api.delete(`/rates/${id}`);
  return response.data;
};

export default api;
