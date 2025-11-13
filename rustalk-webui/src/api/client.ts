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
  CertificateOperationResponse 
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

export default api;
