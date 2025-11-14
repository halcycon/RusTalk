import { useEffect, useState } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  TextField,
  Button,
  Grid,
  Alert,
  CircularProgress,
  Divider,
  Switch,
  FormControlLabel,
  Chip,
  Paper,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
} from '@mui/material';
import {
  Save as SaveIcon,
  Refresh as RefreshIcon,
  PlayArrow as TestIcon,
  CheckCircle as CheckCircleIcon,
  Error as ErrorIcon,
  Warning as WarningIcon,
} from '@mui/icons-material';
import { getTeamsConfig, updateTeamsConfig, getTeamsStatus, testTeamsConnection } from '../api/client';
import type { TeamsConfig, TeamsStatusResponse, TeamsHealthStatus } from '../types';
import { mockTeamsConfig, mockTeamsStatus } from '../mockData';

export default function TeamsEdge() {
  const [config, setConfig] = useState<TeamsConfig | null>(null);
  const [status, setStatus] = useState<TeamsStatusResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);
  const [testing, setTesting] = useState(false);
  const [usingMockData, setUsingMockData] = useState(false);

  const fetchData = async () => {
    try {
      setLoading(true);
      const [configData, statusData] = await Promise.all([
        getTeamsConfig(),
        getTeamsStatus(),
      ]);
      setConfig(configData);
      setStatus(statusData);
      setError(null);
      setUsingMockData(false);
    } catch (err) {
      // In development, use mock data when API is unavailable
      if (import.meta.env.DEV) {
        setConfig(mockTeamsConfig);
        setStatus(mockTeamsStatus);
        setUsingMockData(true);
        setError(null);
      } else {
        setError('Failed to fetch Teams/Edge configuration from server');
      }
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  const handleSave = async () => {
    if (!config) return;

    try {
      setSaving(true);
      setError(null);
      setSuccess(null);
      await updateTeamsConfig(config);
      setSuccess('Teams/Edge configuration updated successfully');
      await fetchData();
    } catch (err) {
      setError('Failed to update configuration');
      console.error(err);
    } finally {
      setSaving(false);
    }
  };

  const handleTest = async () => {
    try {
      setTesting(true);
      setError(null);
      setSuccess(null);
      const result = await testTeamsConnection();
      if (result.success) {
        setSuccess('Teams connection test successful');
        if (status) {
          setStatus({
            ...status,
            health_status: result.health_status,
          });
        }
      } else {
        setError(`Teams connection test failed: ${result.message}`);
      }
    } catch (err) {
      setError('Failed to test Teams connection');
      console.error(err);
    } finally {
      setTesting(false);
    }
  };

  const handleChange = (field: keyof TeamsConfig, value: string | number | boolean | string[]) => {
    if (!config) return;
    
    setConfig({
      ...config,
      [field]: value,
    });
  };

  const getHealthStatusIcon = (status: string) => {
    switch (status) {
      case 'healthy':
        return <CheckCircleIcon color="success" />;
      case 'degraded':
        return <WarningIcon color="warning" />;
      case 'down':
        return <ErrorIcon color="error" />;
      default:
        return <WarningIcon color="warning" />;
    }
  };

  const getHealthStatusColor = (status: string): 'success' | 'warning' | 'error' | 'default' => {
    switch (status) {
      case 'healthy':
        return 'success';
      case 'degraded':
        return 'warning';
      case 'down':
        return 'error';
      default:
        return 'default';
    }
  };

  if (loading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
        <CircularProgress />
      </Box>
    );
  }

  if (error && !config) {
    return <Alert severity="error">{error}</Alert>;
  }

  return (
    <Box>
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Typography variant="h4">
          Microsoft Teams / Edge SBC
        </Typography>
        <Box display="flex" gap={2}>
          <Button
            variant="outlined"
            startIcon={<RefreshIcon />}
            onClick={fetchData}
          >
            Refresh
          </Button>
          <Button
            variant="outlined"
            color="secondary"
            startIcon={<TestIcon />}
            onClick={handleTest}
            disabled={testing}
          >
            {testing ? 'Testing...' : 'Test Connection'}
          </Button>
          <Button
            variant="contained"
            color="primary"
            startIcon={<SaveIcon />}
            onClick={handleSave}
            disabled={saving}
          >
            {saving ? 'Saving...' : 'Save Changes'}
          </Button>
        </Box>
      </Box>

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
      {success && <Alert severity="success" sx={{ mb: 2 }}>{success}</Alert>}

      {usingMockData && (
        <Alert severity="info" sx={{ mb: 2 }}>
          <Typography variant="body2" fontWeight="bold">Development Mode</Typography>
          Using mock data. Start the RusTalk server to see real data.
        </Alert>
      )}

      <Grid container spacing={3}>
        {/* Status Overview */}
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Status Overview
              </Typography>
              <Divider sx={{ mb: 2 }} />
              <Grid container spacing={2}>
                <Grid item xs={12} md={3}>
                  <Paper sx={{ p: 2, textAlign: 'center' }}>
                    <Typography variant="body2" color="text.secondary">
                      Status
                    </Typography>
                    <Typography variant="h6">
                      <Chip 
                        label={status?.enabled ? 'Enabled' : 'Disabled'}
                        color={status?.enabled ? 'success' : 'default'}
                        size="small"
                      />
                    </Typography>
                  </Paper>
                </Grid>
                <Grid item xs={12} md={3}>
                  <Paper sx={{ p: 2, textAlign: 'center' }}>
                    <Typography variant="body2" color="text.secondary">
                      Total Calls
                    </Typography>
                    <Typography variant="h6">
                      {status?.total_calls || 0}
                    </Typography>
                  </Paper>
                </Grid>
                <Grid item xs={12} md={3}>
                  <Paper sx={{ p: 2, textAlign: 'center' }}>
                    <Typography variant="body2" color="text.secondary">
                      Active Calls
                    </Typography>
                    <Typography variant="h6">
                      {status?.active_calls || 0}
                    </Typography>
                  </Paper>
                </Grid>
                <Grid item xs={12} md={3}>
                  <Paper sx={{ p: 2, textAlign: 'center' }}>
                    <Typography variant="body2" color="text.secondary">
                      SIP Proxies
                    </Typography>
                    <Typography variant="h6">
                      {config?.sip_proxies.length || 0}
                    </Typography>
                  </Paper>
                </Grid>
              </Grid>
            </CardContent>
          </Card>
        </Grid>

        {/* Health Status */}
        {status?.health_status && status.health_status.length > 0 && (
          <Grid item xs={12}>
            <Card>
              <CardContent>
                <Typography variant="h6" gutterBottom>
                  SIP Proxy Health Status
                </Typography>
                <Divider sx={{ mb: 2 }} />
                <TableContainer>
                  <Table>
                    <TableHead>
                      <TableRow>
                        <TableCell>Proxy</TableCell>
                        <TableCell>Status</TableCell>
                        <TableCell>Response Time</TableCell>
                        <TableCell>Last Check</TableCell>
                        <TableCell>Error</TableCell>
                      </TableRow>
                    </TableHead>
                    <TableBody>
                      {status.health_status.map((health: TeamsHealthStatus) => (
                        <TableRow key={health.proxy}>
                          <TableCell>{health.proxy}</TableCell>
                          <TableCell>
                            <Box display="flex" alignItems="center" gap={1}>
                              {getHealthStatusIcon(health.status)}
                              <Chip 
                                label={health.status}
                                color={getHealthStatusColor(health.status)}
                                size="small"
                              />
                            </Box>
                          </TableCell>
                          <TableCell>
                            {health.response_time_ms ? `${health.response_time_ms}ms` : '-'}
                          </TableCell>
                          <TableCell>
                            {new Date(health.last_check).toLocaleString()}
                          </TableCell>
                          <TableCell>
                            {health.error || '-'}
                          </TableCell>
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
                </TableContainer>
              </CardContent>
            </Card>
          </Grid>
        )}

        {/* Teams Configuration */}
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Microsoft Teams Configuration
              </Typography>
              <Divider sx={{ mb: 2 }} />
              <Grid container spacing={2}>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="SBC FQDN"
                    value={config?.sbc_fqdn || ''}
                    onChange={(e) => handleChange('sbc_fqdn', e.target.value)}
                    helperText="Fully qualified domain name for the SBC"
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="Tenant Domain"
                    value={config?.tenant_domain || ''}
                    onChange={(e) => handleChange('tenant_domain', e.target.value)}
                    helperText="Microsoft Teams tenant domain"
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="mTLS Certificate Path"
                    value={config?.mtls_cert_path || ''}
                    onChange={(e) => handleChange('mtls_cert_path', e.target.value)}
                    helperText="Path to the mTLS certificate file"
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="mTLS Key Path"
                    value={config?.mtls_key_path || ''}
                    onChange={(e) => handleChange('mtls_key_path', e.target.value)}
                    helperText="Path to the mTLS key file"
                  />
                </Grid>
              </Grid>
            </CardContent>
          </Card>
        </Grid>

        {/* SIP Proxies */}
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Teams SIP Proxies
              </Typography>
              <Divider sx={{ mb: 2 }} />
              <Grid container spacing={2}>
                {config?.sip_proxies.map((proxy, index) => (
                  <Grid item xs={12} md={4} key={index}>
                    <TextField
                      fullWidth
                      label={`SIP Proxy ${index + 1}`}
                      value={proxy}
                      onChange={(e) => {
                        const newProxies = [...(config?.sip_proxies || [])];
                        newProxies[index] = e.target.value;
                        handleChange('sip_proxies', newProxies);
                      }}
                    />
                  </Grid>
                ))}
              </Grid>
            </CardContent>
          </Card>
        </Grid>

        {/* Health Check Configuration */}
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Health Check Configuration
              </Typography>
              <Divider sx={{ mb: 2 }} />
              <Grid container spacing={2}>
                <Grid item xs={12}>
                  <FormControlLabel
                    control={
                      <Switch
                        checked={config?.options_ping_enabled || false}
                        onChange={(e) => handleChange('options_ping_enabled', e.target.checked)}
                      />
                    }
                    label="Enable OPTIONS Ping"
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="OPTIONS Ping Interval (seconds)"
                    type="number"
                    value={config?.options_ping_interval || 60}
                    onChange={(e) => handleChange('options_ping_interval', parseInt(e.target.value))}
                    disabled={!config?.options_ping_enabled}
                    helperText="How often to send OPTIONS pings to Teams proxies"
                  />
                </Grid>
              </Grid>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      <Box mt={3} display="flex" justifyContent="flex-end" gap={2}>
        <Button
          variant="outlined"
          color="secondary"
          size="large"
          startIcon={<TestIcon />}
          onClick={handleTest}
          disabled={testing}
        >
          {testing ? 'Testing...' : 'Test Connection'}
        </Button>
        <Button
          variant="contained"
          color="primary"
          size="large"
          startIcon={<SaveIcon />}
          onClick={handleSave}
          disabled={saving}
        >
          {saving ? 'Saving...' : 'Save Changes'}
        </Button>
      </Box>
    </Box>
  );
}
