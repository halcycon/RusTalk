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
} from '@mui/material';
import { Save as SaveIcon } from '@mui/icons-material';
import { getConfig, updateConfig } from '../api/client';
import type { Config } from '../types';

export default function Configuration() {
  const [config, setConfig] = useState<Config | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    const fetchConfig = async () => {
      try {
        setLoading(true);
        const data = await getConfig();
        setConfig(data);
        setError(null);
      } catch (err) {
        setError('Failed to fetch configuration from server');
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    fetchConfig();
  }, []);

  const handleSave = async () => {
    if (!config) return;

    try {
      setSaving(true);
      setError(null);
      setSuccess(null);
      await updateConfig(config);
      setSuccess('Configuration updated successfully');
    } catch (err) {
      setError('Failed to update configuration');
      console.error(err);
    } finally {
      setSaving(false);
    }
  };

  const handleChange = (section: keyof Config, field: string, value: string | number | boolean) => {
    if (!config) return;
    
    setConfig({
      ...config,
      [section]: {
        ...config[section as keyof typeof config],
        [field]: value,
      },
    });
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
          Configuration
        </Typography>
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

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
      {success && <Alert severity="success" sx={{ mb: 2 }}>{success}</Alert>}

      <Grid container spacing={3}>
        {/* Server Configuration */}
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Server Configuration
              </Typography>
              <Divider sx={{ mb: 2 }} />
              <Grid container spacing={2}>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="Bind Address"
                    value={config?.server?.bind_address || ''}
                    onChange={(e) => handleChange('server', 'bind_address', e.target.value)}
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="Bind Port"
                    type="number"
                    value={config?.server?.bind_port || ''}
                    onChange={(e) => handleChange('server', 'bind_port', parseInt(e.target.value))}
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="Workers"
                    type="number"
                    value={config?.server?.workers || ''}
                    onChange={(e) => handleChange('server', 'workers', parseInt(e.target.value))}
                  />
                </Grid>
              </Grid>
            </CardContent>
          </Card>
        </Grid>

        {/* SIP Configuration */}
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                SIP Configuration
              </Typography>
              <Divider sx={{ mb: 2 }} />
              <Grid container spacing={2}>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="Domain"
                    value={config?.sip?.domain || ''}
                    onChange={(e) => handleChange('sip', 'domain', e.target.value)}
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="User Agent"
                    value={config?.sip?.user_agent || ''}
                    onChange={(e) => handleChange('sip', 'user_agent', e.target.value)}
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="Max Forwards"
                    type="number"
                    value={config?.sip?.max_forwards || ''}
                    onChange={(e) => handleChange('sip', 'max_forwards', parseInt(e.target.value))}
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="Session Expires (seconds)"
                    type="number"
                    value={config?.sip?.session_expires || ''}
                    onChange={(e) => handleChange('sip', 'session_expires', parseInt(e.target.value))}
                  />
                </Grid>
              </Grid>
            </CardContent>
          </Card>
        </Grid>

        {/* Transport Configuration */}
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Transport Configuration
              </Typography>
              <Divider sx={{ mb: 2 }} />
              <Grid container spacing={2}>
                <Grid item xs={12} md={4}>
                  <TextField
                    fullWidth
                    label="UDP Port"
                    type="number"
                    value={config?.transport?.udp_port || ''}
                    onChange={(e) => handleChange('transport', 'udp_port', parseInt(e.target.value))}
                  />
                </Grid>
                <Grid item xs={12} md={4}>
                  <TextField
                    fullWidth
                    label="TCP Port"
                    type="number"
                    value={config?.transport?.tcp_port || ''}
                    onChange={(e) => handleChange('transport', 'tcp_port', parseInt(e.target.value))}
                  />
                </Grid>
                <Grid item xs={12} md={4}>
                  <TextField
                    fullWidth
                    label="TLS Port"
                    type="number"
                    value={config?.transport?.tls_port || ''}
                    onChange={(e) => handleChange('transport', 'tls_port', parseInt(e.target.value))}
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="TLS Certificate Path"
                    value={config?.transport?.tls_cert || ''}
                    onChange={(e) => handleChange('transport', 'tls_cert', e.target.value)}
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="TLS Key Path"
                    value={config?.transport?.tls_key || ''}
                    onChange={(e) => handleChange('transport', 'tls_key', e.target.value)}
                  />
                </Grid>
              </Grid>
            </CardContent>
          </Card>
        </Grid>

        {/* Microsoft Teams Configuration */}
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Microsoft Teams Direct Routing
              </Typography>
              <Divider sx={{ mb: 2 }} />
              <Grid container spacing={2}>
                <Grid item xs={12}>
                  <FormControlLabel
                    control={
                      <Switch
                        checked={config?.teams?.enabled || false}
                        onChange={(e) => handleChange('teams', 'enabled', e.target.checked)}
                      />
                    }
                    label="Enable Teams Integration"
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="SBC FQDN"
                    value={config?.teams?.sbc_fqdn || ''}
                    onChange={(e) => handleChange('teams', 'sbc_fqdn', e.target.value)}
                    disabled={!config?.teams?.enabled}
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="Trunk FQDN"
                    value={config?.teams?.trunk_fqdn || ''}
                    onChange={(e) => handleChange('teams', 'trunk_fqdn', e.target.value)}
                    disabled={!config?.teams?.enabled}
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="mTLS Certificate Path"
                    value={config?.teams?.mtls_cert || ''}
                    onChange={(e) => handleChange('teams', 'mtls_cert', e.target.value)}
                    disabled={!config?.teams?.enabled}
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="mTLS Key Path"
                    value={config?.teams?.mtls_key || ''}
                    onChange={(e) => handleChange('teams', 'mtls_key', e.target.value)}
                    disabled={!config?.teams?.enabled}
                  />
                </Grid>
              </Grid>
            </CardContent>
          </Card>
        </Grid>

        {/* Database Configuration */}
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Database Configuration
              </Typography>
              <Divider sx={{ mb: 2 }} />
              <Grid container spacing={2}>
                <Grid item xs={12}>
                  <TextField
                    fullWidth
                    label="Database URL"
                    value={config?.database?.url || ''}
                    onChange={(e) => handleChange('database', 'url', e.target.value)}
                    placeholder="postgresql://user:password@localhost/rustalk"
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="Max Connections"
                    type="number"
                    value={config?.database?.max_connections || ''}
                    onChange={(e) => handleChange('database', 'max_connections', parseInt(e.target.value))}
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TextField
                    fullWidth
                    label="Min Connections"
                    type="number"
                    value={config?.database?.min_connections || ''}
                    onChange={(e) => handleChange('database', 'min_connections', parseInt(e.target.value))}
                  />
                </Grid>
              </Grid>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      <Box mt={3} display="flex" justifyContent="flex-end">
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
