import React from 'react';
import {
  AppBar,
  Box,
  CssBaseline,
  Drawer,
  IconButton,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Toolbar,
  Typography,
  Tooltip,
  Accordion,
  AccordionSummary,
  AccordionDetails,
} from '@mui/material';
import {
  Menu as MenuIcon,
  Dashboard as DashboardIcon,
  Phone as PhoneIcon,
  Settings as SettingsIcon,
  BarChart as BarChartIcon,
  Brightness4 as DarkModeIcon,
  Brightness7 as LightModeIcon,
  Security as SecurityIcon,
  History as HistoryIcon,
  AttachMoney as AttachMoneyIcon,
  GraphicEq as CodecIcon,
  Contacts as ContactsIcon,
  PhoneForwarded as TrunkIcon,
  Groups as GroupIcon,
  Route as RouteIcon,
  Dialpad as DialpadIcon,
  ExpandMore as ExpandMoreIcon,
  Cloud as CloudIcon,
} from '@mui/icons-material';
import { useNavigate, useLocation } from 'react-router-dom';
import { useThemeMode } from '../theme';

const drawerWidth = 240;

interface LayoutProps {
  children: React.ReactNode;
}

interface MenuItem {
  text: string;
  icon: React.ReactElement;
  path: string;
}

interface MenuGroup {
  title: string;
  icon: React.ReactElement;
  items: MenuItem[];
}

// Standalone menu items (not in groups)
const standaloneMenuItems: MenuItem[] = [
  { text: 'Dashboard', icon: <DashboardIcon />, path: '/' },
];

// Grouped menu items
const menuGroups: MenuGroup[] = [
  {
    title: 'Call Management',
    icon: <PhoneIcon />,
    items: [
      { text: 'Active Calls', icon: <PhoneIcon />, path: '/calls' },
      { text: 'Call Logs', icon: <HistoryIcon />, path: '/call-logs' },
    ],
  },
  {
    title: 'PBX Features',
    icon: <ContactsIcon />,
    items: [
      { text: 'Extensions', icon: <ContactsIcon />, path: '/extensions' },
      { text: 'DIDs', icon: <DialpadIcon />, path: '/dids' },
      { text: 'Ring Groups', icon: <GroupIcon />, path: '/ring-groups' },
      { text: 'Trunks', icon: <TrunkIcon />, path: '/trunks' },
    ],
  },
  {
    title: 'Routing',
    icon: <RouteIcon />,
    items: [
      { text: 'Routes', icon: <RouteIcon />, path: '/routes' },
      { text: 'SIP Profiles', icon: <SettingsIcon />, path: '/sip-profiles' },
    ],
  },
  {
    title: 'Teams/SBC',
    icon: <CloudIcon />,
    items: [
      { text: 'Teams Edge', icon: <CloudIcon />, path: '/teams-edge' },
    ],
  },
  {
    title: 'Media',
    icon: <CodecIcon />,
    items: [
      { text: 'Codecs', icon: <CodecIcon />, path: '/codecs' },
    ],
  },
  {
    title: 'Billing',
    icon: <AttachMoneyIcon />,
    items: [
      { text: 'Rates', icon: <AttachMoneyIcon />, path: '/rates' },
    ],
  },
  {
    title: 'Security',
    icon: <SecurityIcon />,
    items: [
      { text: 'Certificates', icon: <SecurityIcon />, path: '/certificates' },
    ],
  },
  {
    title: 'System',
    icon: <SettingsIcon />,
    items: [
      { text: 'Configuration', icon: <SettingsIcon />, path: '/config' },
      { text: 'Statistics', icon: <BarChartIcon />, path: '/stats' },
    ],
  },
];

export default function Layout({ children }: LayoutProps) {
  const [mobileOpen, setMobileOpen] = React.useState(false);
  const [expandedGroups, setExpandedGroups] = React.useState<string[]>([]);
  const navigate = useNavigate();
  const location = useLocation();
  const { mode, toggleTheme } = useThemeMode();

  // Auto-expand the group containing the current path
  React.useEffect(() => {
    const currentGroup = menuGroups.find(group =>
      group.items.some(item => item.path === location.pathname)
    );
    if (currentGroup) {
      setExpandedGroups(prev => 
        prev.includes(currentGroup.title) ? prev : [...prev, currentGroup.title]
      );
    }
  }, [location.pathname]);

  const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen);
  };

  const handleAccordionChange = (groupTitle: string) => {
    setExpandedGroups(prev =>
      prev.includes(groupTitle)
        ? prev.filter(title => title !== groupTitle)
        : [...prev, groupTitle]
    );
  };

  const drawer = (
    <div>
      <Toolbar>
        <Box display="flex" alignItems="center" width="100%" gap={1}>
          <img 
            src="https://github.com/user-attachments/assets/e096d831-7060-4a74-bc72-b52a49cecc8b" 
            alt="RusTalk Logo" 
            style={{ height: '40px', width: '40px', objectFit: 'contain' }}
          />
          <Typography variant="h6" noWrap component="div">
            RusTalk
          </Typography>
        </Box>
      </Toolbar>
      
      {/* Standalone menu items */}
      <List>
        {standaloneMenuItems.map((item) => (
          <ListItem key={item.text} disablePadding>
            <ListItemButton
              selected={location.pathname === item.path}
              onClick={() => navigate(item.path)}
            >
              <ListItemIcon>{item.icon}</ListItemIcon>
              <ListItemText primary={item.text} />
            </ListItemButton>
          </ListItem>
        ))}
      </List>

      {/* Grouped menu items with accordions */}
      <Box sx={{ px: 1 }}>
        {menuGroups.map((group) => (
          <Accordion
            key={group.title}
            expanded={expandedGroups.includes(group.title)}
            onChange={() => handleAccordionChange(group.title)}
            disableGutters
            elevation={0}
            sx={{
              '&:before': { display: 'none' },
              '&.Mui-expanded': { margin: 0 },
            }}
          >
            <AccordionSummary
              expandIcon={<ExpandMoreIcon />}
              sx={{
                minHeight: 48,
                '&.Mui-expanded': { minHeight: 48 },
                px: 2,
              }}
            >
              <Box display="flex" alignItems="center" gap={1}>
                {group.icon}
                <Typography>{group.title}</Typography>
              </Box>
            </AccordionSummary>
            <AccordionDetails sx={{ p: 0 }}>
              <List disablePadding>
                {group.items.map((item) => (
                  <ListItem key={item.text} disablePadding>
                    <ListItemButton
                      selected={location.pathname === item.path}
                      onClick={() => navigate(item.path)}
                      sx={{ pl: 4 }}
                    >
                      <ListItemIcon>{item.icon}</ListItemIcon>
                      <ListItemText primary={item.text} />
                    </ListItemButton>
                  </ListItem>
                ))}
              </List>
            </AccordionDetails>
          </Accordion>
        ))}
      </Box>
    </div>
  );

  return (
    <Box sx={{ display: 'flex' }}>
      <CssBaseline />
      <AppBar
        position="fixed"
        sx={{
          width: { sm: `calc(100% - ${drawerWidth}px)` },
          ml: { sm: `${drawerWidth}px` },
        }}
      >
        <Toolbar>
          <IconButton
            color="inherit"
            aria-label="open drawer"
            edge="start"
            onClick={handleDrawerToggle}
            sx={{ mr: 2, display: { sm: 'none' } }}
          >
            <MenuIcon />
          </IconButton>
          <Typography variant="h6" noWrap component="div" sx={{ flexGrow: 1 }}>
            RusTalk Admin Console
          </Typography>
          <Tooltip title={`Switch to ${mode === 'light' ? 'dark' : 'light'} mode`}>
            <IconButton
              color="inherit"
              onClick={toggleTheme}
              sx={{ ml: 1 }}
            >
              {mode === 'light' ? <DarkModeIcon /> : <LightModeIcon />}
            </IconButton>
          </Tooltip>
        </Toolbar>
      </AppBar>
      <Box
        component="nav"
        sx={{ width: { sm: drawerWidth }, flexShrink: { sm: 0 } }}
      >
        <Drawer
          variant="temporary"
          open={mobileOpen}
          onClose={handleDrawerToggle}
          ModalProps={{
            keepMounted: true,
          }}
          sx={{
            display: { xs: 'block', sm: 'none' },
            '& .MuiDrawer-paper': { boxSizing: 'border-box', width: drawerWidth },
          }}
        >
          {drawer}
        </Drawer>
        <Drawer
          variant="permanent"
          sx={{
            display: { xs: 'none', sm: 'block' },
            '& .MuiDrawer-paper': { boxSizing: 'border-box', width: drawerWidth },
          }}
          open
        >
          {drawer}
        </Drawer>
      </Box>
      <Box
        component="main"
        sx={{
          flexGrow: 1,
          p: 3,
          width: { sm: `calc(100% - ${drawerWidth}px)` },
        }}
      >
        <Toolbar />
        {children}
      </Box>
    </Box>
  );
}
