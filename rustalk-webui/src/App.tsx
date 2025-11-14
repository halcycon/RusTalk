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
import Codecs from './pages/Codecs';
import DIDs from './pages/DIDs';
import Extensions from './pages/Extensions';
import Trunks from './pages/Trunks';
import RingGroups from './pages/RingGroups';
import RoutesPage from './pages/Routes';
import SipProfiles from './pages/SipProfiles';

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
            <Route path="/codecs" element={<Codecs />} />
            <Route path="/dids" element={<DIDs />} />
            <Route path="/extensions" element={<Extensions />} />
            <Route path="/trunks" element={<Trunks />} />
            <Route path="/ring-groups" element={<RingGroups />} />
            <Route path="/routes" element={<RoutesPage />} />
            <Route path="/sip-profiles" element={<SipProfiles />} />
          </Routes>
        </Layout>
      </Router>
    </ThemeProviderWrapper>
  );
}

export default App;
