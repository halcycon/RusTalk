import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import Layout from './components/Layout';
import Dashboard from './pages/Dashboard';
import Calls from './pages/Calls';
import Statistics from './pages/Statistics';
import Configuration from './pages/Configuration';

const theme = createTheme({
  palette: {
    mode: 'light',
    primary: {
      main: '#1976d2',
    },
    secondary: {
      main: '#dc004e',
    },
  },
});

function App() {
  return (
    <ThemeProvider theme={theme}>
      <Router>
        <Layout>
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/calls" element={<Calls />} />
            <Route path="/stats" element={<Statistics />} />
            <Route path="/config" element={<Configuration />} />
          </Routes>
        </Layout>
      </Router>
    </ThemeProvider>
  );
}

export default App;
