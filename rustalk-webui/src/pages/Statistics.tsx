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
  BarChart,
  Bar,
  LineChart,
  Line,
  PieChart,
  Pie,
  Cell,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts';
import { getStats } from '../api/client';
import type { Stats } from '../types';

const COLORS = ['#0088FE', '#00C49F', '#FFBB28', '#FF8042'];

export default function Statistics() {
  const [stats, setStats] = useState<Stats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchStats = async () => {
      try {
        setLoading(true);
        const data = await getStats();
        setStats(data);
        setError(null);
      } catch (err) {
        setError('Failed to fetch statistics from server');
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    fetchStats();
    const interval = setInterval(fetchStats, 5000);

    return () => clearInterval(interval);
  }, []);

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

  // Mock data for demonstration
  const hourlyCallData = [
    { hour: '00:00', inbound: 12, outbound: 8 },
    { hour: '04:00', inbound: 5, outbound: 3 },
    { hour: '08:00', inbound: 35, outbound: 28 },
    { hour: '12:00', inbound: 58, outbound: 52 },
    { hour: '16:00', inbound: 48, outbound: 42 },
    { hour: '20:00', inbound: 32, outbound: 28 },
  ];

  const callStatusData = [
    { name: 'Successful', value: stats?.total_calls_today ? Math.floor(stats.total_calls_today * 0.85) : 85 },
    { name: 'Failed', value: stats?.total_calls_today ? Math.floor(stats.total_calls_today * 0.10) : 10 },
    { name: 'Busy', value: stats?.total_calls_today ? Math.floor(stats.total_calls_today * 0.05) : 5 },
  ];

  const performanceData = [
    { metric: 'Avg Call Duration', value: 185 },
    { metric: 'Setup Time (ms)', value: 250 },
    { metric: 'Jitter (ms)', value: 12 },
    { metric: 'Packet Loss (%)', value: 0.5 },
  ];

  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        Statistics & Analytics
      </Typography>

      <Grid container spacing={3}>
        {/* Hourly Call Volume */}
        <Grid item xs={12} lg={8}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Hourly Call Volume
              </Typography>
              <ResponsiveContainer width="100%" height={350}>
                <BarChart data={hourlyCallData}>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis dataKey="hour" />
                  <YAxis />
                  <Tooltip />
                  <Legend />
                  <Bar dataKey="inbound" fill="#8884d8" />
                  <Bar dataKey="outbound" fill="#82ca9d" />
                </BarChart>
              </ResponsiveContainer>
            </CardContent>
          </Card>
        </Grid>

        {/* Call Status Distribution */}
        <Grid item xs={12} lg={4}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Call Status Distribution
              </Typography>
              <ResponsiveContainer width="100%" height={350}>
                <PieChart>
                  <Pie
                    data={callStatusData}
                    cx="50%"
                    cy="50%"
                    labelLine={false}
                    label={(entry) => `${entry.name}: ${entry.value}`}
                    outerRadius={100}
                    fill="#8884d8"
                    dataKey="value"
                  >
                    {callStatusData.map((_entry, index) => (
                      <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                    ))}
                  </Pie>
                  <Tooltip />
                </PieChart>
              </ResponsiveContainer>
            </CardContent>
          </Card>
        </Grid>

        {/* Performance Metrics */}
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Performance Metrics
              </Typography>
              <ResponsiveContainer width="100%" height={300}>
                <BarChart data={performanceData} layout="vertical">
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis type="number" />
                  <YAxis dataKey="metric" type="category" width={150} />
                  <Tooltip />
                  <Legend />
                  <Bar dataKey="value" fill="#8884d8" />
                </BarChart>
              </ResponsiveContainer>
            </CardContent>
          </Card>
        </Grid>

        {/* Resource Usage Over Time */}
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Resource Usage Trends
              </Typography>
              <ResponsiveContainer width="100%" height={300}>
                <LineChart
                  data={[
                    { time: '00:00', cpu: 15, memory: 25 },
                    { time: '04:00', cpu: 12, memory: 23 },
                    { time: '08:00', cpu: 35, memory: 40 },
                    { time: '12:00', cpu: 45, memory: 55 },
                    { time: '16:00', cpu: 38, memory: 48 },
                    { time: '20:00', cpu: 28, memory: 35 },
                    { time: '23:59', cpu: stats?.cpu_usage || 20, memory: stats?.memory_usage || 30 },
                  ]}
                >
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis dataKey="time" />
                  <YAxis />
                  <Tooltip />
                  <Legend />
                  <Line type="monotone" dataKey="cpu" stroke="#8884d8" name="CPU %" />
                  <Line type="monotone" dataKey="memory" stroke="#82ca9d" name="Memory %" />
                </LineChart>
              </ResponsiveContainer>
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    </Box>
  );
}
