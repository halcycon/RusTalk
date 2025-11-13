import { useEffect, useState } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Button,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  CircularProgress,
  Alert,
  IconButton,
  Switch,
  FormControlLabel,
  Grid,
  InputAdornment,
} from '@mui/material';
import {
  Add as AddIcon,
  Edit as EditIcon,
  Delete as DeleteIcon,
  Upload as UploadIcon,
} from '@mui/icons-material';
import { getRates, saveRate, deleteRate, importRates } from '../api/client';
import type { RateCard } from '../types';

export default function RatesManagement() {
  const [rates, setRates] = useState<RateCard[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [editDialogOpen, setEditDialogOpen] = useState(false);
  const [importDialogOpen, setImportDialogOpen] = useState(false);
  const [currentRate, setCurrentRate] = useState<Partial<RateCard>>({});
  const [importData, setImportData] = useState('');
  const [importFormat] = useState<'json' | 'csv'>('csv');

  useEffect(() => {
    fetchRates();
  }, []);

  const fetchRates = async () => {
    try {
      setLoading(true);
      const data = await getRates();
      setRates(data.rates);
      setError(null);
    } catch (err) {
      setError('Failed to fetch rates from server');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleAddNew = () => {
    setCurrentRate({
      name: '',
      prefix: '',
      rate_per_minute: 0,
      connection_fee: 0,
      minimum_charge_seconds: 30,
      billing_increment_seconds: 6,
      currency: 'USD',
      active: true,
    });
    setEditDialogOpen(true);
  };

  const handleEdit = (rate: RateCard) => {
    setCurrentRate(rate);
    setEditDialogOpen(true);
  };

  const handleSave = async () => {
    try {
      if (!currentRate.name || !currentRate.prefix) {
        setError('Name and prefix are required');
        return;
      }

      const rateToSave: RateCard = {
        id: currentRate.id || `rate-${Date.now()}`,
        name: currentRate.name!,
        description: currentRate.description,
        prefix: currentRate.prefix!,
        rate_per_minute: currentRate.rate_per_minute || 0,
        connection_fee: currentRate.connection_fee || 0,
        minimum_charge_seconds: currentRate.minimum_charge_seconds || 30,
        billing_increment_seconds: currentRate.billing_increment_seconds || 6,
        currency: currentRate.currency || 'USD',
        effective_date: currentRate.effective_date || Math.floor(Date.now() / 1000),
        end_date: currentRate.end_date,
        active: currentRate.active !== false,
      };

      await saveRate(rateToSave);
      setEditDialogOpen(false);
      fetchRates();
    } catch (err) {
      setError('Failed to save rate');
      console.error(err);
    }
  };

  const handleDelete = async (id: string) => {
    if (!confirm('Are you sure you want to delete this rate?')) {
      return;
    }

    try {
      await deleteRate(id);
      fetchRates();
    } catch (err) {
      setError('Failed to delete rate');
      console.error(err);
    }
  };

  const handleImport = async () => {
    try {
      const result = await importRates({
        format: importFormat,
        data: importData,
        overwrite: false,
      });

      if (result.success) {
        setImportDialogOpen(false);
        setImportData('');
        fetchRates();
        alert(`Successfully imported ${result.imported_count} rates`);
      } else {
        setError(`Import failed: ${result.errors.join(', ')}`);
      }
    } catch (err) {
      setError('Failed to import rates');
      console.error(err);
    }
  };

  const handleFileUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (e) => {
      const text = e.target?.result as string;
      setImportData(text);
    };
    reader.readAsText(file);
  };

  const formatDate = (timestamp?: number): string => {
    if (!timestamp) return 'N/A';
    return new Date(timestamp * 1000).toLocaleDateString();
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
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
        <Typography variant="h4">Rate Management</Typography>
        <Box>
          <Button
            variant="outlined"
            startIcon={<UploadIcon />}
            onClick={() => setImportDialogOpen(true)}
            sx={{ mr: 1 }}
          >
            Import Rates
          </Button>
          <Button variant="contained" startIcon={<AddIcon />} onClick={handleAddNew}>
            Add Rate
          </Button>
        </Box>
      </Box>

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}

      <Card>
        <CardContent>
          {rates.length === 0 ? (
            <Alert severity="info">No rates configured</Alert>
          ) : (
            <TableContainer component={Paper}>
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>Name</TableCell>
                    <TableCell>Prefix</TableCell>
                    <TableCell align="right">Rate/Min</TableCell>
                    <TableCell align="right">Connection Fee</TableCell>
                    <TableCell>Min. Charge</TableCell>
                    <TableCell>Billing Inc.</TableCell>
                    <TableCell>Effective Date</TableCell>
                    <TableCell>Active</TableCell>
                    <TableCell>Actions</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {rates.map((rate) => (
                    <TableRow key={rate.id}>
                      <TableCell>
                        <Typography variant="body2">{rate.name}</Typography>
                        {rate.description && (
                          <Typography variant="caption" color="text.secondary">
                            {rate.description}
                          </Typography>
                        )}
                      </TableCell>
                      <TableCell>{rate.prefix}</TableCell>
                      <TableCell align="right">
                        {rate.currency} {rate.rate_per_minute.toFixed(4)}
                      </TableCell>
                      <TableCell align="right">
                        {rate.currency} {rate.connection_fee.toFixed(4)}
                      </TableCell>
                      <TableCell>{rate.minimum_charge_seconds}s</TableCell>
                      <TableCell>{rate.billing_increment_seconds}s</TableCell>
                      <TableCell>{formatDate(rate.effective_date)}</TableCell>
                      <TableCell>
                        <Switch checked={rate.active} disabled size="small" />
                      </TableCell>
                      <TableCell>
                        <IconButton size="small" onClick={() => handleEdit(rate)}>
                          <EditIcon />
                        </IconButton>
                        <IconButton
                          size="small"
                          onClick={() => handleDelete(rate.id)}
                          color="error"
                        >
                          <DeleteIcon />
                        </IconButton>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
          )}
        </CardContent>
      </Card>

      {/* Edit/Add Rate Dialog */}
      <Dialog open={editDialogOpen} onClose={() => setEditDialogOpen(false)} maxWidth="md" fullWidth>
        <DialogTitle>{currentRate.id ? 'Edit Rate' : 'Add New Rate'}</DialogTitle>
        <DialogContent>
          <Grid container spacing={2} sx={{ mt: 1 }}>
            <Grid item xs={12} sm={6}>
              <TextField
                fullWidth
                label="Name"
                value={currentRate.name || ''}
                onChange={(e) => setCurrentRate({ ...currentRate, name: e.target.value })}
              />
            </Grid>
            <Grid item xs={12} sm={6}>
              <TextField
                fullWidth
                label="Prefix"
                value={currentRate.prefix || ''}
                onChange={(e) => setCurrentRate({ ...currentRate, prefix: e.target.value })}
              />
            </Grid>
            <Grid item xs={12}>
              <TextField
                fullWidth
                label="Description"
                value={currentRate.description || ''}
                onChange={(e) => setCurrentRate({ ...currentRate, description: e.target.value })}
                multiline
                rows={2}
              />
            </Grid>
            <Grid item xs={12} sm={6}>
              <TextField
                fullWidth
                label="Rate per Minute"
                type="number"
                value={currentRate.rate_per_minute || 0}
                onChange={(e) =>
                  setCurrentRate({ ...currentRate, rate_per_minute: parseFloat(e.target.value) })
                }
                InputProps={{
                  startAdornment: <InputAdornment position="start">$</InputAdornment>,
                }}
              />
            </Grid>
            <Grid item xs={12} sm={6}>
              <TextField
                fullWidth
                label="Connection Fee"
                type="number"
                value={currentRate.connection_fee || 0}
                onChange={(e) =>
                  setCurrentRate({ ...currentRate, connection_fee: parseFloat(e.target.value) })
                }
                InputProps={{
                  startAdornment: <InputAdornment position="start">$</InputAdornment>,
                }}
              />
            </Grid>
            <Grid item xs={12} sm={6}>
              <TextField
                fullWidth
                label="Minimum Charge (seconds)"
                type="number"
                value={currentRate.minimum_charge_seconds || 30}
                onChange={(e) =>
                  setCurrentRate({
                    ...currentRate,
                    minimum_charge_seconds: parseInt(e.target.value),
                  })
                }
              />
            </Grid>
            <Grid item xs={12} sm={6}>
              <TextField
                fullWidth
                label="Billing Increment (seconds)"
                type="number"
                value={currentRate.billing_increment_seconds || 6}
                onChange={(e) =>
                  setCurrentRate({
                    ...currentRate,
                    billing_increment_seconds: parseInt(e.target.value),
                  })
                }
              />
            </Grid>
            <Grid item xs={12} sm={6}>
              <TextField
                fullWidth
                label="Currency"
                value={currentRate.currency || 'USD'}
                onChange={(e) => setCurrentRate({ ...currentRate, currency: e.target.value })}
              />
            </Grid>
            <Grid item xs={12} sm={6}>
              <FormControlLabel
                control={
                  <Switch
                    checked={currentRate.active !== false}
                    onChange={(e) => setCurrentRate({ ...currentRate, active: e.target.checked })}
                  />
                }
                label="Active"
              />
            </Grid>
          </Grid>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setEditDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleSave} variant="contained">
            Save
          </Button>
        </DialogActions>
      </Dialog>

      {/* Import Dialog */}
      <Dialog open={importDialogOpen} onClose={() => setImportDialogOpen(false)} maxWidth="md" fullWidth>
        <DialogTitle>Import Rates</DialogTitle>
        <DialogContent>
          <Box sx={{ mt: 2 }}>
            <Typography variant="body2" gutterBottom>
              Upload a CSV or JSON file containing rate information.
            </Typography>
            <Box sx={{ mt: 2 }}>
              <input
                accept=".csv,.json"
                style={{ display: 'none' }}
                id="rate-file-upload"
                type="file"
                onChange={handleFileUpload}
              />
              <label htmlFor="rate-file-upload">
                <Button variant="outlined" component="span" startIcon={<UploadIcon />}>
                  Choose File
                </Button>
              </label>
            </Box>
            <TextField
              fullWidth
              multiline
              rows={10}
              label="Rate Data"
              value={importData}
              onChange={(e) => setImportData(e.target.value)}
              sx={{ mt: 2 }}
              helperText="Paste your CSV or JSON data here, or upload a file"
            />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setImportDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleImport} variant="contained" disabled={!importData}>
            Import
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}
