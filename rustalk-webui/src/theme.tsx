import { createContext, useContext, useState, useMemo, useEffect } from 'react';
import type { ReactNode } from 'react';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import type { PaletteMode } from '@mui/material/styles';
import { CssBaseline } from '@mui/material';

interface ThemeContextType {
  mode: PaletteMode;
  toggleTheme: () => void;
}

const ThemeContext = createContext<ThemeContextType>({
  mode: 'light',
  toggleTheme: () => {},
});

// eslint-disable-next-line react-refresh/only-export-components
export const useThemeMode = () => useContext(ThemeContext);

interface ThemeProviderWrapperProps {
  children: ReactNode;
}

export function ThemeProviderWrapper({ children }: ThemeProviderWrapperProps) {
  // Get initial theme from localStorage or default to light
  const [mode, setMode] = useState<PaletteMode>(() => {
    const savedMode = localStorage.getItem('themeMode');
    return (savedMode === 'dark' || savedMode === 'light') ? savedMode : 'light';
  });

  // Save theme preference to localStorage whenever it changes
  useEffect(() => {
    localStorage.setItem('themeMode', mode);
  }, [mode]);

  const toggleTheme = () => {
    setMode((prevMode) => (prevMode === 'light' ? 'dark' : 'light'));
  };

  const theme = useMemo(
    () =>
      createTheme({
        palette: {
          mode,
          primary: {
            main: mode === 'light' ? '#6366f1' : '#818cf8',
            light: mode === 'light' ? '#818cf8' : '#a5b4fc',
            dark: mode === 'light' ? '#4f46e5' : '#6366f1',
            contrastText: '#ffffff',
          },
          secondary: {
            main: mode === 'light' ? '#ec4899' : '#f472b6',
            light: mode === 'light' ? '#f472b6' : '#f9a8d4',
            dark: mode === 'light' ? '#db2777' : '#ec4899',
            contrastText: '#ffffff',
          },
          success: {
            main: mode === 'light' ? '#10b981' : '#34d399',
            light: mode === 'light' ? '#34d399' : '#6ee7b7',
            dark: mode === 'light' ? '#059669' : '#10b981',
          },
          warning: {
            main: mode === 'light' ? '#f59e0b' : '#fbbf24',
            light: mode === 'light' ? '#fbbf24' : '#fcd34d',
            dark: mode === 'light' ? '#d97706' : '#f59e0b',
          },
          error: {
            main: mode === 'light' ? '#ef4444' : '#f87171',
            light: mode === 'light' ? '#f87171' : '#fca5a5',
            dark: mode === 'light' ? '#dc2626' : '#ef4444',
          },
          info: {
            main: mode === 'light' ? '#3b82f6' : '#60a5fa',
            light: mode === 'light' ? '#60a5fa' : '#93c5fd',
            dark: mode === 'light' ? '#2563eb' : '#3b82f6',
          },
          background: {
            default: mode === 'light' ? '#f8fafc' : '#0f172a',
            paper: mode === 'light' ? '#ffffff' : '#1e293b',
          },
          text: {
            primary: mode === 'light' ? '#0f172a' : '#f1f5f9',
            secondary: mode === 'light' ? '#475569' : '#cbd5e1',
          },
        },
        typography: {
          fontFamily: '"Inter", "Roboto", "Helvetica", "Arial", sans-serif',
          h4: {
            fontWeight: 700,
            letterSpacing: '-0.02em',
          },
          h6: {
            fontWeight: 600,
            letterSpacing: '-0.01em',
          },
        },
        shape: {
          borderRadius: 12,
        },
        components: {
          MuiCard: {
            styleOverrides: {
              root: {
                boxShadow: mode === 'light' 
                  ? '0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)'
                  : '0 1px 3px 0 rgb(0 0 0 / 0.3), 0 1px 2px -1px rgb(0 0 0 / 0.5)',
                transition: 'all 0.3s ease-in-out',
                '&:hover': {
                  transform: 'translateY(-2px)',
                  boxShadow: mode === 'light'
                    ? '0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)'
                    : '0 10px 15px -3px rgb(0 0 0 / 0.3), 0 4px 6px -4px rgb(0 0 0 / 0.5)',
                },
              },
            },
          },
          MuiButton: {
            styleOverrides: {
              root: {
                textTransform: 'none',
                fontWeight: 600,
                borderRadius: 8,
                padding: '8px 16px',
              },
            },
          },
          MuiChip: {
            styleOverrides: {
              root: {
                fontWeight: 500,
              },
            },
          },
          MuiDrawer: {
            styleOverrides: {
              paper: {
                background: mode === 'light' 
                  ? 'linear-gradient(180deg, #ffffff 0%, #f8fafc 100%)'
                  : 'linear-gradient(180deg, #1e293b 0%, #0f172a 100%)',
                borderRight: mode === 'light' ? '1px solid #e2e8f0' : '1px solid #334155',
              },
            },
          },
          MuiAppBar: {
            styleOverrides: {
              root: {
                background: mode === 'light'
                  ? 'linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%)'
                  : 'linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%)',
                boxShadow: mode === 'light'
                  ? '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)'
                  : '0 4px 6px -1px rgb(0 0 0 / 0.3), 0 2px 4px -2px rgb(0 0 0 / 0.5)',
              },
            },
          },
          MuiTableHead: {
            styleOverrides: {
              root: {
                background: mode === 'light' ? '#f8fafc' : '#1e293b',
                '& .MuiTableCell-head': {
                  fontWeight: 600,
                  color: mode === 'light' ? '#334155' : '#cbd5e1',
                },
              },
            },
          },
        },
      }),
    [mode]
  );

  return (
    <ThemeContext.Provider value={{ mode, toggleTheme }}>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        {children}
      </ThemeProvider>
    </ThemeContext.Provider>
  );
}
