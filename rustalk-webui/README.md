# RusTalk Web UI

Modern React-based admin console for RusTalk SIP B2BUA/PBX/SBC.

## Features

- **Dashboard**: Real-time system overview with statistics and call metrics
- **Call Management**: Monitor active calls with detailed information
- **Configuration**: Manage server, SIP, transport, Teams, and database settings
- **Statistics & Analytics**: Visual charts and graphs for performance monitoring

## Technology Stack

- **React 19** with TypeScript
- **Material-UI v6** for UI components
- **Recharts** for data visualization
- **Vite** for fast development and optimized builds
- **Axios** for API communication
- **React Router** for client-side routing

## Development

### Prerequisites

- Node.js 18+ and npm
- Running RusTalk backend (rustalk-cloud) on http://localhost:8080

### Install Dependencies

```bash
npm install
```

### Run Development Server

```bash
npm run dev
```

The application will start on http://localhost:3000 with hot module replacement.

### Build for Production

```bash
npm run build
```

The built files will be output to the `dist/` directory.

### Linting

```bash
npm run lint
```

## API Integration

The Web UI communicates with the RusTalk Cloud API at `/api/v1`:

- `GET /health` - Health check
- `GET /api/v1/calls` - List active calls
- `GET /api/v1/calls/:id` - Get call details
- `GET /api/v1/config` - Get configuration
- `POST /api/v1/config` - Update configuration
- `GET /api/v1/stats` - Get system statistics

API requests are proxied during development (see `vite.config.ts`).

## Project Structure

```
rustalk-webui/
├── src/
│   ├── api/          # API client and integration
│   ├── components/   # Reusable components (Layout, etc.)
│   ├── pages/        # Page components (Dashboard, Calls, etc.)
│   ├── types/        # TypeScript type definitions
│   ├── App.tsx       # Main application component
│   └── main.tsx      # Application entry point
├── public/           # Static assets
├── dist/             # Build output (gitignored)
└── package.json      # Dependencies and scripts
```

## Configuration

### Vite Configuration

The `vite.config.ts` file includes:

- API proxy configuration for development
- Build output settings
- React plugin configuration

### TypeScript Configuration

- `tsconfig.json` - Base TypeScript configuration
- `tsconfig.app.json` - Application-specific settings
- `tsconfig.node.json` - Node.js tooling settings

## Deployment

### Serving with Rust Backend

The RusTalk Cloud API server can serve the Web UI static files:

1. Build the Web UI:
   ```bash
   npm run build
   ```

2. Configure the Rust backend to serve from `rustalk-webui/dist/`:
   ```rust
   let api = CloudApi::new("0.0.0.0:8080".parse().unwrap())
       .with_webui_path("rustalk-webui/dist".to_string());
   ```

3. Access the UI at http://localhost:8080/

### Standalone Deployment

The Web UI can be deployed separately using any static file server:

- **Nginx**: Serve the `dist/` directory
- **Apache**: Configure DocumentRoot to `dist/`
- **Cloud Storage**: Upload to S3, Azure Blob, etc.
- **CDN**: Use Netlify, Vercel, or similar services

## Features

### Dashboard

- System status indicator
- Active call count
- Total calls today
- Uptime display
- Resource usage (CPU/Memory)
- Call activity chart (24-hour view)

### Calls

- Real-time call list
- Call status indicators (Active, Ringing, Ended, Failed)
- Call details (ID, From, To, Duration)
- Auto-refresh every 3 seconds

### Configuration

- Server settings (bind address, port, workers)
- SIP settings (domain, user agent, session parameters)
- Transport settings (UDP, TCP, TLS ports and certificates)
- Microsoft Teams Direct Routing (mTLS configuration)
- Database settings (PostgreSQL connection)
- Save configuration changes

### Statistics

- Hourly call volume chart (inbound/outbound)
- Call status distribution pie chart
- Performance metrics bar chart
- Resource usage trends over time

## Contributing

Contributions are welcome! Please ensure:

- Code follows the existing style
- TypeScript types are properly defined
- Components are well-documented
- UI is responsive and accessible

## License

MIT License - See LICENSE file for details
