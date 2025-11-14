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
  CodecListResponse,
  CodecUpdateRequest,
  CodecAddRequest,
  CodecRemoveRequest,
  CodecReorderRequest,
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

// Codec management API calls
export const getCodecs = async (): Promise<CodecListResponse> => {
  const response = await api.get('/codecs');
  return response.data;
};

export const updateCodec = async (request: CodecUpdateRequest): Promise<{ success: boolean; message: string }> => {
  const response = await api.put('/codecs/update', request);
  return response.data;
};

export const addCodec = async (request: CodecAddRequest): Promise<{ success: boolean; message: string }> => {
  const response = await api.post('/codecs/add', request);
  return response.data;
};

export const removeCodec = async (request: CodecRemoveRequest): Promise<{ success: boolean; message: string }> => {
  const response = await api.post('/codecs/remove', request);
  return response.data;
};

export const reorderCodecs = async (request: CodecReorderRequest): Promise<{ success: boolean; message: string; codecs: any[] }> => {
  const response = await api.post('/codecs/reorder', request);
  return response.data;
};

// DID management API calls
export const getDids = async (): Promise<import('../types').DidListResponse> => {
  const response = await api.get('/dids');
  return response.data;
};

export const getDid = async (id: string): Promise<import('../types').Did> => {
  const response = await api.get(`/dids/${id}`);
  return response.data;
};

export const createDid = async (did: import('../types').Did): Promise<{ success: boolean; message: string; id: string }> => {
  const response = await api.post('/dids', did);
  return response.data;
};

export const updateDid = async (id: string, did: import('../types').Did): Promise<{ success: boolean; message: string }> => {
  const response = await api.put(`/dids/${id}`, did);
  return response.data;
};

export const deleteDid = async (id: string): Promise<{ success: boolean; message: string }> => {
  const response = await api.delete(`/dids/${id}`);
  return response.data;
};

export const reorderDids = async (request: import('../types').ReorderRequest): Promise<{ success: boolean; message: string; dids: any[] }> => {
  const response = await api.post('/dids/reorder', request);
  return response.data;
};

// Extension management API calls
export const getExtensions = async (): Promise<import('../types').ExtensionListResponse> => {
  const response = await api.get('/extensions');
  return response.data;
};

export const getExtension = async (id: string): Promise<import('../types').Extension> => {
  const response = await api.get(`/extensions/${id}`);
  return response.data;
};

export const createExtension = async (extension: import('../types').Extension): Promise<{ success: boolean; message: string; id: string }> => {
  const response = await api.post('/extensions', extension);
  return response.data;
};

export const updateExtension = async (id: string, extension: import('../types').Extension): Promise<{ success: boolean; message: string }> => {
  const response = await api.put(`/extensions/${id}`, extension);
  return response.data;
};

export const deleteExtension = async (id: string): Promise<{ success: boolean; message: string }> => {
  const response = await api.delete(`/extensions/${id}`);
  return response.data;
};

export const reorderExtensions = async (request: import('../types').ReorderRequest): Promise<{ success: boolean; message: string; extensions: any[] }> => {
  const response = await api.post('/extensions/reorder', request);
  return response.data;
};

// Trunk management API calls
export const getTrunks = async (): Promise<import('../types').TrunkListResponse> => {
  const response = await api.get('/trunks');
  return response.data;
};

export const getTrunk = async (id: string): Promise<import('../types').Trunk> => {
  const response = await api.get(`/trunks/${id}`);
  return response.data;
};

export const createTrunk = async (trunk: import('../types').Trunk): Promise<{ success: boolean; message: string; id: string }> => {
  const response = await api.post('/trunks', trunk);
  return response.data;
};

export const updateTrunk = async (id: string, trunk: import('../types').Trunk): Promise<{ success: boolean; message: string }> => {
  const response = await api.put(`/trunks/${id}`, trunk);
  return response.data;
};

export const deleteTrunk = async (id: string): Promise<{ success: boolean; message: string }> => {
  const response = await api.delete(`/trunks/${id}`);
  return response.data;
};

export const reorderTrunks = async (request: import('../types').ReorderRequest): Promise<{ success: boolean; message: string; trunks: any[] }> => {
  const response = await api.post('/trunks/reorder', request);
  return response.data;
};

// Ring Group management API calls
export const getRingGroups = async (): Promise<import('../types').RingGroupListResponse> => {
  const response = await api.get('/ring-groups');
  return response.data;
};

export const getRingGroup = async (id: string): Promise<import('../types').RingGroup> => {
  const response = await api.get(`/ring-groups/${id}`);
  return response.data;
};

export const createRingGroup = async (ringGroup: import('../types').RingGroup): Promise<{ success: boolean; message: string; id: string }> => {
  const response = await api.post('/ring-groups', ringGroup);
  return response.data;
};

export const updateRingGroup = async (id: string, ringGroup: import('../types').RingGroup): Promise<{ success: boolean; message: string }> => {
  const response = await api.put(`/ring-groups/${id}`, ringGroup);
  return response.data;
};

export const deleteRingGroup = async (id: string): Promise<{ success: boolean; message: string }> => {
  const response = await api.delete(`/ring-groups/${id}`);
  return response.data;
};

export const reorderRingGroups = async (request: import('../types').ReorderRequest): Promise<{ success: boolean; message: string; ring_groups: any[] }> => {
  const response = await api.post('/ring-groups/reorder', request);
  return response.data;
};

// Route management API calls
export const getRoutes = async (): Promise<import('../types').RouteListResponse> => {
  const response = await api.get('/routes');
  return response.data;
};

export const getRoute = async (id: string): Promise<import('../types').Route> => {
  const response = await api.get(`/routes/${id}`);
  return response.data;
};

export const createRoute = async (route: import('../types').Route): Promise<{ success: boolean; message: string; id: string }> => {
  const response = await api.post('/routes', route);
  return response.data;
};

export const updateRoute = async (id: string, route: import('../types').Route): Promise<{ success: boolean; message: string }> => {
  const response = await api.put(`/routes/${id}`, route);
  return response.data;
};

export const deleteRoute = async (id: string): Promise<{ success: boolean; message: string }> => {
  const response = await api.delete(`/routes/${id}`);
  return response.data;
};

export const reorderRoutes = async (request: import('../types').ReorderRequest): Promise<{ success: boolean; message: string; routes: any[] }> => {
  const response = await api.post('/routes/reorder', request);
  return response.data;
};

export const testRoute = async (request: import('../types').RouteTestRequest): Promise<import('../types').RouteTestResponse> => {
  const response = await api.post('/routes/test', request);
  return response.data;
};

// SIP Profile management API calls
export const getSipProfiles = async (): Promise<import('../types').SipProfileListResponse> => {
  const response = await api.get('/sip-profiles');
  return response.data;
};

export const getSipProfile = async (id: string): Promise<import('../types').SipProfile> => {
  const response = await api.get(`/sip-profiles/${id}`);
  return response.data;
};

export const createSipProfile = async (profile: import('../types').SipProfile): Promise<{ success: boolean; message: string; id: string }> => {
  const response = await api.post('/sip-profiles', profile);
  return response.data;
};

export const updateSipProfile = async (id: string, profile: import('../types').SipProfile): Promise<{ success: boolean; message: string }> => {
  const response = await api.put(`/sip-profiles/${id}`, profile);
  return response.data;
};

export const deleteSipProfile = async (id: string): Promise<{ success: boolean; message: string }> => {
  const response = await api.delete(`/sip-profiles/${id}`);
  return response.data;
};

export const reorderSipProfiles = async (request: import('../types').ReorderRequest): Promise<{ success: boolean; message: string; sip_profiles: any[] }> => {
  const response = await api.post('/sip-profiles/reorder', request);
  return response.data;
};

// Teams/Edge SBC management API calls
export const getTeamsStatus = async (): Promise<import('../types').TeamsStatusResponse> => {
  const response = await api.get('/teams/status');
  return response.data;
};

export const getTeamsConfig = async (): Promise<import('../types').TeamsConfig> => {
  const response = await api.get('/teams/config');
  return response.data;
};

export const updateTeamsConfig = async (config: import('../types').TeamsConfig): Promise<{ success: boolean; message: string }> => {
  const response = await api.put('/teams/config', config);
  return response.data;
};

export const testTeamsConnection = async (): Promise<{ success: boolean; message: string; health_status: import('../types').TeamsHealthStatus[] }> => {
  const response = await api.post('/teams/test');
  return response.data;
};

export default api;
