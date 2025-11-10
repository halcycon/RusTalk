import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { ThemeProviderWrapper } from './theme';
import Layout from './components/Layout';
import Dashboard from './pages/Dashboard';
import Calls from './pages/Calls';
import Statistics from './pages/Statistics';
import Configuration from './pages/Configuration';

function App() {
  return (
    <ThemeProviderWrapper>
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
    </ThemeProviderWrapper>
  );
}

export default App;
