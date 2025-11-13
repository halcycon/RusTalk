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
  Chip,
  CircularProgress,
  Alert,
  Button,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Divider,
  Grid,
  IconButton,
  Menu,
  MenuItem,
} from '@mui/material';
import {
  Download as DownloadIcon,
  FileDownload as FileDownloadIcon,
  MoreVert as MoreVertIcon,
} from '@mui/icons-material';
import { getCallLogs, getCallLog, exportCallLogs } from '../api/client';
import type { CallLog, CallLogDetail } from '../types';

const getStatusColor = (status: string): 'default' | 'success' | 'error' | 'warning' => {
  switch (status.toLowerCase()) {
    case 'completed':
      return 'success';
    case 'failed':
      return 'error';
    case 'ringing':
    case 'in-progress':
      return 'warning';
    default:
      return 'default';
  }
};

export default function CallLogs() {
  const [logs, setLogs] = useState<CallLog[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedLog, setSelectedLog] = useState<CallLogDetail | null>(null);
  const [detailLoading, setDetailLoading] = useState(false);
  const [page] = useState(1);
  const [total, setTotal] = useState(0);
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);

  useEffect(() => {
    fetchLogs();
    const interval = setInterval(fetchLogs, 10000); // Refresh every 10 seconds
    return () => clearInterval(interval);
  }, [page]);

  const fetchLogs = async () => {
    try {
      setLoading(true);
      const data = await getCallLogs({ page, per_page: 50 });
      setLogs(data.logs);
      setTotal(data.total);
      setError(null);
    } catch (err) {
      setError('Failed to fetch call logs from server');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleRowClick = async (log: CallLog) => {
    try {
      setDetailLoading(true);
      const detail = await getCallLog(log.id);
      setSelectedLog(detail);
    } catch (err) {
      console.error('Failed to fetch call log details:', err);
      setError('Failed to fetch call log details');
    } finally {
      setDetailLoading(false);
    }
  };

  const handleCloseDetail = () => {
    setSelectedLog(null);
  };

  const handleExportClick = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleExportClose = () => {
    setAnchorEl(null);
  };

  const handleExport = async (format: 'json' | 'csv' | 'pdf') => {
    handleExportClose();
    try {
      const result = await exportCallLogs({
        format,
        include_charges: true,
      });
      
      // Handle the export result based on format
      if (format === 'json' || format === 'csv') {
        const blob = new Blob([result.data], { 
          type: format === 'json' ? 'application/json' : 'text/csv' 
        });
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `call-logs.${format}`;
        a.click();
        window.URL.revokeObjectURL(url);
      } else if (format === 'pdf' && result.url) {
        window.open(result.url, '_blank');
      }
    } catch (err) {
      console.error('Export failed:', err);
      setError('Failed to export call logs');
    }
  };

  const formatDateTime = (timestamp: number): string => {
    return new Date(timestamp * 1000).toLocaleString();
  };

  const formatDuration = (duration?: number): string => {
    if (!duration) return 'N/A';
    const minutes = Math.floor(duration / 60);
    const seconds = duration % 60;
    return `${minutes}:${seconds.toString().padStart(2, '0')}`;
  };

  const formatCost = (cost?: number): string => {
    if (cost === undefined || cost === null) return 'N/A';
    return `$${cost.toFixed(4)}`;
  };

  const formatNumber = (user: string, domain: string): string => {
    return `${user}@${domain}`;
  };

  if (loading && logs.length === 0) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
        <CircularProgress />
      </Box>
    );
  }

  return (
    <Box>
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
        <Typography variant="h4">Call Logs</Typography>
        <Button
          variant="contained"
          startIcon={<DownloadIcon />}
          onClick={handleExportClick}
        >
          Export
        </Button>
        <Menu
          anchorEl={anchorEl}
          open={Boolean(anchorEl)}
          onClose={handleExportClose}
        >
          <MenuItem onClick={() => handleExport('csv')}>Export as CSV</MenuItem>
          <MenuItem onClick={() => handleExport('json')}>Export as JSON</MenuItem>
          <MenuItem onClick={() => handleExport('pdf')}>Export as PDF</MenuItem>
        </Menu>
      </Box>

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}

      <Card>
        <CardContent>
          {logs.length === 0 ? (
            <Alert severity="info">No call logs available</Alert>
          ) : (
            <>
              <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
                Showing {logs.length} of {total} call logs
              </Typography>
              <TableContainer component={Paper}>
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>Start Time</TableCell>
                    <TableCell>From</TableCell>
                    <TableCell>To</TableCell>
                    <TableCell>Duration</TableCell>
                    <TableCell>Status</TableCell>
                    <TableCell>Cost</TableCell>
                    <TableCell>Actions</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {logs.map((log) => (
                    <TableRow 
                      key={log.id}
                      hover
                      onClick={() => handleRowClick(log)}
                      sx={{ cursor: 'pointer' }}
                    >
                      <TableCell>{formatDateTime(log.start_time)}</TableCell>
                      <TableCell>{formatNumber(log.from_user, log.from_domain)}</TableCell>
                      <TableCell>{formatNumber(log.to_user, log.to_domain)}</TableCell>
                      <TableCell>{formatDuration(log.duration_seconds)}</TableCell>
                      <TableCell>
                        <Chip
                          label={log.status}
                          color={getStatusColor(log.status)}
                          size="small"
                        />
                      </TableCell>
                      <TableCell>{formatCost(log.cost)}</TableCell>
                      <TableCell>
                        <IconButton
                          size="small"
                          onClick={(e) => {
                            e.stopPropagation();
                            handleRowClick(log);
                          }}
                        >
                          <MoreVertIcon />
                        </IconButton>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
            </>
          )}
        </CardContent>
      </Card>

      {/* Call Log Detail Dialog */}
      <Dialog
        open={selectedLog !== null}
        onClose={handleCloseDetail}
        maxWidth="md"
        fullWidth
      >
        {detailLoading ? (
          <Box display="flex" justifyContent="center" p={4}>
            <CircularProgress />
          </Box>
        ) : selectedLog ? (
          <>
            <DialogTitle>
              Call Details
              <Typography variant="caption" display="block" color="text.secondary">
                Call ID: {selectedLog.call_id}
              </Typography>
            </DialogTitle>
            <DialogContent dividers>
              <Grid container spacing={2}>
                <Grid item xs={12}>
                  <Typography variant="h6" gutterBottom>
                    Call Information
                  </Typography>
                </Grid>
                <Grid item xs={6}>
                  <Typography variant="body2" color="text.secondary">
                    From
                  </Typography>
                  <Typography>
                    {formatNumber(selectedLog.from_user, selectedLog.from_domain)}
                  </Typography>
                </Grid>
                <Grid item xs={6}>
                  <Typography variant="body2" color="text.secondary">
                    To
                  </Typography>
                  <Typography>
                    {formatNumber(selectedLog.to_user, selectedLog.to_domain)}
                  </Typography>
                </Grid>
                <Grid item xs={6}>
                  <Typography variant="body2" color="text.secondary">
                    Start Time
                  </Typography>
                  <Typography>{formatDateTime(selectedLog.start_time)}</Typography>
                </Grid>
                <Grid item xs={6}>
                  <Typography variant="body2" color="text.secondary">
                    Duration
                  </Typography>
                  <Typography>{formatDuration(selectedLog.duration_seconds)}</Typography>
                </Grid>
                <Grid item xs={6}>
                  <Typography variant="body2" color="text.secondary">
                    Status
                  </Typography>
                  <Chip
                    label={selectedLog.status}
                    color={getStatusColor(selectedLog.status)}
                    size="small"
                  />
                </Grid>
                <Grid item xs={6}>
                  <Typography variant="body2" color="text.secondary">
                    Termination Reason
                  </Typography>
                  <Typography>{selectedLog.termination_reason || 'N/A'}</Typography>
                </Grid>

                <Grid item xs={12}>
                  <Divider sx={{ my: 2 }} />
                  <Typography variant="h6" gutterBottom>
                    SIP Session Details
                  </Typography>
                </Grid>
                <Grid item xs={12}>
                  <Typography variant="body2" color="text.secondary">
                    SIP Call-ID
                  </Typography>
                  <Typography fontFamily="monospace" fontSize="0.875rem">
                    {selectedLog.sip_call_id}
                  </Typography>
                </Grid>
                <Grid item xs={6}>
                  <Typography variant="body2" color="text.secondary">
                    From Tag
                  </Typography>
                  <Typography fontFamily="monospace" fontSize="0.875rem">
                    {selectedLog.from_tag || 'N/A'}
                  </Typography>
                </Grid>
                <Grid item xs={6}>
                  <Typography variant="body2" color="text.secondary">
                    To Tag
                  </Typography>
                  <Typography fontFamily="monospace" fontSize="0.875rem">
                    {selectedLog.to_tag || 'N/A'}
                  </Typography>
                </Grid>
                <Grid item xs={6}>
                  <Typography variant="body2" color="text.secondary">
                    A-Leg Codec
                  </Typography>
                  <Typography>{selectedLog.a_leg_codec || 'N/A'}</Typography>
                </Grid>
                <Grid item xs={6}>
                  <Typography variant="body2" color="text.secondary">
                    B-Leg Codec
                  </Typography>
                  <Typography>{selectedLog.b_leg_codec || 'N/A'}</Typography>
                </Grid>

                {selectedLog.charge_breakdown && selectedLog.charge_breakdown.length > 0 && (
                  <>
                    <Grid item xs={12}>
                      <Divider sx={{ my: 2 }} />
                      <Typography variant="h6" gutterBottom>
                        Charge Breakdown
                      </Typography>
                    </Grid>
                    <Grid item xs={12}>
                      <TableContainer>
                        <Table size="small">
                          <TableHead>
                            <TableRow>
                              <TableCell>Description</TableCell>
                              <TableCell align="right">Rate</TableCell>
                              <TableCell align="right">Quantity</TableCell>
                              <TableCell align="right">Amount</TableCell>
                            </TableRow>
                          </TableHead>
                          <TableBody>
                            {selectedLog.charge_breakdown.map((charge, index) => (
                              <TableRow key={index}>
                                <TableCell>{charge.description}</TableCell>
                                <TableCell align="right">
                                  ${charge.rate.toFixed(4)}/{charge.unit}
                                </TableCell>
                                <TableCell align="right">
                                  {charge.quantity.toFixed(2)} {charge.unit}
                                </TableCell>
                                <TableCell align="right">
                                  ${charge.amount.toFixed(4)}
                                </TableCell>
                              </TableRow>
                            ))}
                            <TableRow>
                              <TableCell colSpan={3} align="right">
                                <strong>Total</strong>
                              </TableCell>
                              <TableCell align="right">
                                <strong>${selectedLog.total_cost?.toFixed(4) || '0.0000'}</strong>
                              </TableCell>
                            </TableRow>
                          </TableBody>
                        </Table>
                      </TableContainer>
                    </Grid>
                  </>
                )}

                {selectedLog.recording_path && (
                  <>
                    <Grid item xs={12}>
                      <Divider sx={{ my: 2 }} />
                      <Typography variant="h6" gutterBottom>
                        Recording
                      </Typography>
                    </Grid>
                    <Grid item xs={12}>
                      <Button
                        variant="outlined"
                        startIcon={<FileDownloadIcon />}
                        onClick={() => {
                          window.open(selectedLog.recording_path!, '_blank');
                        }}
                      >
                        Download Recording
                      </Button>
                    </Grid>
                  </>
                )}
              </Grid>
            </DialogContent>
            <DialogActions>
              <Button
                startIcon={<FileDownloadIcon />}
                onClick={() => {
                  // Download this specific call log
                  const data = JSON.stringify(selectedLog, null, 2);
                  const blob = new Blob([data], { type: 'application/json' });
                  const url = window.URL.createObjectURL(blob);
                  const a = document.createElement('a');
                  a.href = url;
                  a.download = `call-log-${selectedLog.id}.json`;
                  a.click();
                  window.URL.revokeObjectURL(url);
                }}
              >
                Download Details
              </Button>
              <Button onClick={handleCloseDetail}>Close</Button>
            </DialogActions>
          </>
        ) : null}
      </Dialog>
    </Box>
  );
}
