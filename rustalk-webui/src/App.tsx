import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { ThemeProviderWrapper } from './theme';
import Layout from './components/Layout';
import Dashboard from './pages/Dashboard';
import Calls from './pages/Calls';
import CallLogs from './pages/CallLogs';
import Statistics from './pages/Statistics';
import Configuration from './pages/Configuration';
import Certificates from './pages/Certificates';
import RatesManagement from './pages/RatesManagement';

function App() {
  return (
    <ThemeProviderWrapper>
      <Router>
        <Layout>
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/calls" element={<Calls />} />
            <Route path="/call-logs" element={<CallLogs />} />
            <Route path="/stats" element={<Statistics />} />
            <Route path="/config" element={<Configuration />} />
            <Route path="/certificates" element={<Certificates />} />
            <Route path="/rates" element={<RatesManagement />} />
          </Routes>
        </Layout>
      </Router>
    </ThemeProviderWrapper>
  );
}

export default App;
