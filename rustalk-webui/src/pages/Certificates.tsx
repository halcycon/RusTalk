import { useEffect, useState } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  Button,
  Grid,
  Alert,
  CircularProgress,
  Chip,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  Select,
  MenuItem,
  FormControl,
  InputLabel,
  IconButton,
  Tooltip,
  List,
  ListItem,
  ListItemText,
  Divider,
} from '@mui/material';
import {
  Add as AddIcon,
  Refresh as RefreshIcon,
  Security as SecurityIcon,
  Warning as WarningIcon,
  CheckCircle as CheckCircleIcon,
  Error as ErrorIcon,
} from '@mui/icons-material';
import { 
  listCertificates, 
  requestCertificate, 
  renewCertificate 
} from '../api/client';
import type { CertificateInfo, CertificateRequest } from '../types';

export default function Certificates() {
  const [certificates, setCertificates] = useState<CertificateInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [requestDialogOpen, setRequestDialogOpen] = useState(false);
  const [renewDialogOpen, setRenewDialogOpen] = useState(false);
  const [selectedDomain, setSelectedDomain] = useState<string>('');
  const [requesting, setRequesting] = useState(false);
  const [renewing, setRenewing] = useState(false);

  // Form state for new certificate request
  const [requestForm, setRequestForm] = useState<CertificateRequest>({
    domains: [''],
    email: '',
    challenge_type: 'http-01',
    use_staging: false,
  });

  useEffect(() => {
    fetchCertificates();
  }, []);

  const fetchCertificates = async () => {
    try {
      setLoading(true);
      const response = await listCertificates();
      setCertificates(response.certificates);
      setError(null);
    } catch (err) {
      setError('Failed to fetch certificates from server');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleRequestCertificate = async () => {
    // Validate form
    if (!requestForm.email || requestForm.domains.length === 0 || requestForm.domains[0] === '') {
      setError('Please provide email and at least one domain');
      return;
    }

    try {
      setRequesting(true);
      setError(null);
      setSuccess(null);
      
      await requestCertificate({
        ...requestForm,
        domains: requestForm.domains.filter(d => d.trim() !== ''),
      });
      
      setSuccess('Certificate requested successfully! This may take a few moments.');
      setRequestDialogOpen(false);
      
      // Reset form
      setRequestForm({
        domains: [''],
        email: '',
        challenge_type: 'http-01',
        use_staging: false,
      });
      
      // Refresh list after a delay
      setTimeout(() => {
        fetchCertificates();
      }, 2000);
    } catch (err: any) {
      setError(err.response?.data?.message || 'Failed to request certificate');
      console.error(err);
    } finally {
      setRequesting(false);
    }
  };

  const handleRenewCertificate = async () => {
    if (!selectedDomain) {
      setError('Please select a domain');
      return;
    }

    try {
      setRenewing(true);
      setError(null);
      setSuccess(null);
      
      await renewCertificate({ domain: selectedDomain });
      
      setSuccess(`Certificate for ${selectedDomain} renewed successfully!`);
      setRenewDialogOpen(false);
      setSelectedDomain('');
      
      // Refresh list
      fetchCertificates();
    } catch (err: any) {
      setError(err.response?.data?.message || 'Failed to renew certificate');
      console.error(err);
    } finally {
      setRenewing(false);
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'valid':
        return <CheckCircleIcon color="success" />;
      case 'expiring_soon':
        return <WarningIcon color="warning" />;
      case 'expired':
        return <ErrorIcon color="error" />;
      default:
        return <SecurityIcon />;
    }
  };

  const getStatusColor = (status: string): 'success' | 'warning' | 'error' | 'default' => {
    switch (status) {
      case 'valid':
        return 'success';
      case 'expiring_soon':
        return 'warning';
      case 'expired':
        return 'error';
      default:
        return 'default';
    }
  };

  const addDomainField = () => {
    setRequestForm({
      ...requestForm,
      domains: [...requestForm.domains, ''],
    });
  };

  const updateDomainField = (index: number, value: string) => {
    const newDomains = [...requestForm.domains];
    newDomains[index] = value;
    setRequestForm({
      ...requestForm,
      domains: newDomains,
    });
  };

  const removeDomainField = (index: number) => {
    const newDomains = requestForm.domains.filter((_, i) => i !== index);
    setRequestForm({
      ...requestForm,
      domains: newDomains.length > 0 ? newDomains : [''],
    });
  };

  if (loading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
        <CircularProgress />
      </Box>
    );
  }

  return (
    <Box>
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Typography variant="h4" component="h1">
          SSL/TLS Certificates
        </Typography>
        <Box>
          <Tooltip title="Refresh">
            <IconButton onClick={fetchCertificates} sx={{ mr: 1 }}>
              <RefreshIcon />
            </IconButton>
          </Tooltip>
          <Button
            variant="contained"
            startIcon={<AddIcon />}
            onClick={() => setRequestDialogOpen(true)}
          >
            Request Certificate
          </Button>
        </Box>
      </Box>

      {error && (
        <Alert severity="error" sx={{ mb: 2 }} onClose={() => setError(null)}>
          {error}
        </Alert>
      )}

      {success && (
        <Alert severity="success" sx={{ mb: 2 }} onClose={() => setSuccess(null)}>
          {success}
        </Alert>
      )}

      {certificates.length === 0 ? (
        <Card>
          <CardContent>
            <Box textAlign="center" py={4}>
              <SecurityIcon sx={{ fontSize: 60, color: 'text.secondary', mb: 2 }} />
              <Typography variant="h6" color="text.secondary" gutterBottom>
                No Certificates Found
              </Typography>
              <Typography variant="body2" color="text.secondary" mb={2}>
                Request a Let's Encrypt certificate to secure your SIP connections
              </Typography>
              <Button
                variant="contained"
                startIcon={<AddIcon />}
                onClick={() => setRequestDialogOpen(true)}
              >
                Request Your First Certificate
              </Button>
            </Box>
          </CardContent>
        </Card>
      ) : (
        <Grid container spacing={3}>
          {certificates.map((cert) => (
            <Grid item xs={12} md={6} key={cert.domain}>
              <Card>
                <CardContent>
                  <Box display="flex" alignItems="flex-start" justifyContent="space-between" mb={2}>
                    <Box display="flex" alignItems="center" gap={1}>
                      {getStatusIcon(cert.status)}
                      <Typography variant="h6">{cert.domain}</Typography>
                    </Box>
                    <Chip
                      label={cert.status.replace('_', ' ').toUpperCase()}
                      color={getStatusColor(cert.status)}
                      size="small"
                    />
                  </Box>

                  <Divider sx={{ my: 2 }} />

                  <List dense>
                    <ListItem>
                      <ListItemText
                        primary="Domains"
                        secondary={cert.domains.join(', ')}
                      />
                    </ListItem>
                    <ListItem>
                      <ListItemText
                        primary="Expires"
                        secondary={`${cert.expires_at} (${cert.days_until_expiry} days)`}
                      />
                    </ListItem>
                    {cert.serial && (
                      <ListItem>
                        <ListItemText
                          primary="Serial Number"
                          secondary={cert.serial}
                        />
                      </ListItem>
                    )}
                  </List>

                  {cert.needs_renewal && (
                    <Alert severity="warning" sx={{ mt: 2, mb: 1 }}>
                      This certificate should be renewed soon
                    </Alert>
                  )}

                  <Box mt={2} display="flex" gap={1}>
                    <Button
                      variant="outlined"
                      size="small"
                      fullWidth
                      disabled={!cert.needs_renewal}
                      onClick={() => {
                        setSelectedDomain(cert.domain);
                        setRenewDialogOpen(true);
                      }}
                    >
                      Renew
                    </Button>
                  </Box>
                </CardContent>
              </Card>
            </Grid>
          ))}
        </Grid>
      )}

      {/* Request Certificate Dialog */}
      <Dialog
        open={requestDialogOpen}
        onClose={() => !requesting && setRequestDialogOpen(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Request Let's Encrypt Certificate</DialogTitle>
        <DialogContent>
          <Box mt={2}>
            <Typography variant="subtitle2" gutterBottom>
              Domains
            </Typography>
            {requestForm.domains.map((domain, index) => (
              <Box key={index} display="flex" gap={1} mb={2}>
                <TextField
                  fullWidth
                  label={`Domain ${index + 1}`}
                  value={domain}
                  onChange={(e) => updateDomainField(index, e.target.value)}
                  placeholder="example.com"
                  disabled={requesting}
                />
                {requestForm.domains.length > 1 && (
                  <IconButton
                    onClick={() => removeDomainField(index)}
                    disabled={requesting}
                  >
                    <ErrorIcon />
                  </IconButton>
                )}
              </Box>
            ))}
            <Button
              size="small"
              onClick={addDomainField}
              disabled={requesting}
            >
              + Add Domain
            </Button>

            <TextField
              fullWidth
              label="Email"
              type="email"
              value={requestForm.email}
              onChange={(e) => setRequestForm({ ...requestForm, email: e.target.value })}
              placeholder="admin@example.com"
              disabled={requesting}
              sx={{ mt: 2 }}
              helperText="Used for Let's Encrypt account and notifications"
            />

            <FormControl fullWidth sx={{ mt: 2 }}>
              <InputLabel>Challenge Type</InputLabel>
              <Select
                value={requestForm.challenge_type}
                label="Challenge Type"
                onChange={(e) => setRequestForm({ ...requestForm, challenge_type: e.target.value as 'http-01' | 'dns-01' })}
                disabled={requesting}
              >
                <MenuItem value="http-01">HTTP-01 (requires port 80)</MenuItem>
                <MenuItem value="dns-01">DNS-01 (requires DNS access)</MenuItem>
              </Select>
            </FormControl>

            <Alert severity="info" sx={{ mt: 2 }}>
              <Typography variant="body2">
                <strong>HTTP-01:</strong> Requires port 80 to be accessible from the internet.
              </Typography>
              <Typography variant="body2" sx={{ mt: 1 }}>
                <strong>DNS-01:</strong> Requires adding a TXT record to your DNS.
              </Typography>
            </Alert>
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setRequestDialogOpen(false)} disabled={requesting}>
            Cancel
          </Button>
          <Button
            onClick={handleRequestCertificate}
            variant="contained"
            disabled={requesting}
          >
            {requesting ? <CircularProgress size={24} /> : 'Request Certificate'}
          </Button>
        </DialogActions>
      </Dialog>

      {/* Renew Certificate Dialog */}
      <Dialog
        open={renewDialogOpen}
        onClose={() => !renewing && setRenewDialogOpen(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Renew Certificate</DialogTitle>
        <DialogContent>
          <Typography variant="body1" sx={{ mt: 2 }}>
            Are you sure you want to renew the certificate for <strong>{selectedDomain}</strong>?
          </Typography>
          <Alert severity="info" sx={{ mt: 2 }}>
            The renewal process will request a new certificate from Let's Encrypt and replace the current one.
          </Alert>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setRenewDialogOpen(false)} disabled={renewing}>
            Cancel
          </Button>
          <Button
            onClick={handleRenewCertificate}
            variant="contained"
            disabled={renewing}
          >
            {renewing ? <CircularProgress size={24} /> : 'Renew'}
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}
