import { useEffect, useState } from 'react';
import {
  Box,
  Card,
  CardContent,
  Grid,
  Typography,
  CircularProgress,
  Alert,
} from '@mui/material';
import {
  Phone as PhoneIcon,
  TrendingUp as TrendingUpIcon,
  Timer as TimerIcon,
} from '@mui/icons-material';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';
import { getStats, healthCheck } from '../api/client';
import type { Stats, HealthResponse } from '../types';

export default function Dashboard() {
  const [stats, setStats] = useState<Stats | null>(null);
  const [health, setHealth] = useState<HealthResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        setLoading(true);
        const [statsData, healthData] = await Promise.all([
          getStats(),
          healthCheck(),
        ]);
        setStats(statsData);
        setHealth(healthData);
        setError(null);
      } catch (err) {
        setError('Failed to fetch data from server');
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
    const interval = setInterval(fetchData, 5000); // Refresh every 5 seconds

    return () => clearInterval(interval);
  }, []);

  const formatUptime = (seconds: number): string => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    return `${hours}h ${minutes}m ${secs}s`;
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

  // Chart data should come from a historical API endpoint
  // For now, showing current active calls as the latest data point
  // TODO: Add API endpoint for historical call data (e.g., /api/v1/stats/history)
  const chartData = [
    { time: 'Now', calls: stats?.active_calls || 0 },
  ];

  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        Dashboard
      </Typography>

      <Grid container spacing={3}>
        {/* Active Calls */}
        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" mb={2}>
                <PhoneIcon color="primary" sx={{ fontSize: 40, mr: 2 }} />
                <div>
                  <Typography color="textSecondary" variant="body2">
                    Active Calls
                  </Typography>
                  <Typography variant="h4">{stats?.active_calls || 0}</Typography>
                </div>
              </Box>
            </CardContent>
          </Card>
        </Grid>

        {/* Total Calls Today */}
        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" mb={2}>
                <TrendingUpIcon color="success" sx={{ fontSize: 40, mr: 2 }} />
                <div>
                  <Typography color="textSecondary" variant="body2">
                    Calls Today
                  </Typography>
                  <Typography variant="h4">{stats?.total_calls_today || 0}</Typography>
                </div>
              </Box>
            </CardContent>
          </Card>
        </Grid>

        {/* Uptime */}
        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" mb={2}>
                <TimerIcon color="info" sx={{ fontSize: 40, mr: 2 }} />
                <div>
                  <Typography color="textSecondary" variant="body2">
                    Uptime
                  </Typography>
                  <Typography variant="h6">
                    {stats?.uptime_seconds ? formatUptime(stats.uptime_seconds) : 'N/A'}
                  </Typography>
                </div>
              </Box>
            </CardContent>
          </Card>
        </Grid>

        {/* System Status */}
        <Grid item xs={12} md={6}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                System Status
              </Typography>
              <Box mt={2}>
                <Typography variant="body1">
                  Status: <strong style={{ color: health?.status === 'healthy' ? 'green' : 'red' }}>
                    {health?.status || 'Unknown'}
                  </strong>
                </Typography>
                <Typography variant="body1">
                  Service: <strong>{health?.service || 'N/A'}</strong>
                </Typography>
                <Typography variant="body1">
                  Version: <strong>{health?.version || 'N/A'}</strong>
                </Typography>
              </Box>
            </CardContent>
          </Card>
        </Grid>

        {/* Resource Usage */}
        <Grid item xs={12} md={6}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Resource Usage
              </Typography>
              <Box mt={2}>
                <Typography variant="body1">
                  CPU: <strong>{stats?.cpu_usage ? `${stats.cpu_usage.toFixed(1)}%` : 'N/A'}</strong>
                </Typography>
                <Typography variant="body1">
                  Memory: <strong>{stats?.memory_usage ? `${stats.memory_usage.toFixed(1)}%` : 'N/A'}</strong>
                </Typography>
              </Box>
            </CardContent>
          </Card>
        </Grid>

        {/* Call Activity Chart */}
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Call Activity (Last 24 Hours)
              </Typography>
              <ResponsiveContainer width="100%" height={300}>
                <LineChart data={chartData}>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis dataKey="time" />
                  <YAxis />
                  <Tooltip />
                  <Legend />
                  <Line type="monotone" dataKey="calls" stroke="#8884d8" activeDot={{ r: 8 }} />
                </LineChart>
              </ResponsiveContainer>
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    </Box>
  );
}
