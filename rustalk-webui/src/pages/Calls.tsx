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
  AlertTitle,
} from '@mui/material';
import { getCalls } from '../api/client';
import type { CallInfo } from '../types';
import { mockCalls } from '../mockData';

const getStatusColor = (status: string): 'default' | 'primary' | 'success' | 'error' | 'warning' => {
  switch (status) {
    case 'active':
      return 'success';
    case 'ringing':
      return 'warning';
    case 'ended':
      return 'default';
    case 'failed':
      return 'error';
    default:
      return 'default';
  }
};

export default function Calls() {
  const [calls, setCalls] = useState<CallInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [usingMockData, setUsingMockData] = useState(false);

  useEffect(() => {
    const fetchCalls = async () => {
      try {
        setLoading(true);
        const data = await getCalls();
        setCalls(data);
        setError(null);
        setUsingMockData(false);
      } catch (err) {
        // In development, use mock data when API is unavailable
        if (import.meta.env.DEV) {
          setCalls(mockCalls);
          setUsingMockData(true);
          setError(null);
        } else {
          setError('Failed to fetch calls from server');
        }
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    fetchCalls();
    const interval = setInterval(fetchCalls, 3000); // Refresh every 3 seconds

    return () => clearInterval(interval);
  }, []);

  const formatDateTime = (timestamp: number): string => {
    return new Date(timestamp * 1000).toLocaleString();
  };

  const formatDuration = (duration?: number): string => {
    if (!duration) return 'N/A';
    const minutes = Math.floor(duration / 60);
    const seconds = duration % 60;
    return `${minutes}:${seconds.toString().padStart(2, '0')}`;
  };

  if (loading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
        <CircularProgress />
      </Box>
    );
  }

  if (error) {
    return <Alert severity="error">{error}</Alert>;
  }

  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        Active Calls
      </Typography>

      {usingMockData && (
        <Alert severity="info" sx={{ mb: 3 }}>
          <AlertTitle>Development Mode</AlertTitle>
          Using mock data. Start the RusTalk server to see real calls.
        </Alert>
      )}

      <Card>
        <CardContent>
          {calls.length === 0 ? (
            <Alert severity="info">No active calls</Alert>
          ) : (
            <TableContainer component={Paper}>
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>Call ID</TableCell>
                    <TableCell>From</TableCell>
                    <TableCell>To</TableCell>
                    <TableCell>Status</TableCell>
                    <TableCell>Start Time</TableCell>
                    <TableCell>Duration</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {calls.map((call) => (
                    <TableRow key={call.id}>
                      <TableCell>
                        <Typography variant="body2" fontFamily="monospace">
                          {call.id}
                        </Typography>
                      </TableCell>
                      <TableCell>{call.from}</TableCell>
                      <TableCell>{call.to}</TableCell>
                      <TableCell>
                        <Chip
                          label={call.status}
                          color={getStatusColor(call.status)}
                          size="small"
                        />
                      </TableCell>
                      <TableCell>{formatDateTime(call.start_time)}</TableCell>
                      <TableCell>{formatDuration(call.duration)}</TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
          )}
        </CardContent>
      </Card>
    </Box>
  );
}
