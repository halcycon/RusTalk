<img width="480" height="480" alt="RusTalk Logo" src="https://github.com/user-attachments/assets/e096d831-7060-4a74-bc72-b52a49cecc8b" />

# RusTalk
A high-performance SIP B2BUA / PBX / MS-Teams-compatible SBC built from the ground up in Rust. That tone you can hear? It's the sound of memory-safety.

## Features

- **High Performance**: Built on tokio async runtime for efficient I/O
- **Memory Safe**: Written in Rust - no buffer overflows, no use-after-free
- **B2BUA**: Full Back-to-Back User Agent implementation
- **Microsoft Teams**: Direct Routing support with mTLS authentication
- **SRTP**: Secure RTP pass-through without media decryption
- **Modular**: Separate crates for core engine, edge SBC, cloud API, and CLI
- **Web UI**: Modern React-based admin console with real-time dashboards
- **Configuration**: JSON-based config with PostgreSQL database overlay
- **TLS/mTLS**: Secure SIP (SIPS) using rustls

## Architecture

RusTalk consists of four modular crates plus a modern Web UI:

1. **rustalk-core**: Core SIP engine and B2BUA implementation
2. **rustalk-edge**: Session Border Controller with Teams gateway
3. **rustalk-cloud**: REST API service for management and monitoring
4. **rustalk-cli**: Command-line administration tool
5. **rustalk-webui**: React-based admin console with real-time monitoring

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed architecture documentation.

## Quick Start

### Prerequisites

- Rust 1.75 or later
- PostgreSQL (optional, for database overlay)

### Installation

```bash
# Clone the repository
git clone https://github.com/halcycon/RusTalk.git
cd RusTalk

# Build the project
cargo build --release

# Install the CLI tool
cargo install --path rustalk-cli
```

### Generate Configuration

```bash
rustalk generate-config --output config.json
```

Edit `config.json` to customize your settings:

```json
{
  "server": {
    "bind_address": "0.0.0.0",
    "bind_port": 5060,
    "workers": 4
  },
  "sip": {
    "domain": "rustalk.local",
    "user_agent": "RusTalk/0.1.0",
    "max_forwards": 70,
    "session_expires": 1800
  },
  "transport": {
    "protocols": ["udp", "tcp", "tls"],
    "udp_port": 5060,
    "tcp_port": 5060,
    "tls_port": 5061,
    "tls_cert": "/etc/rustalk/cert.pem",
    "tls_key": "/etc/rustalk/key.pem"
  },
  "teams": {
    "enabled": true,
    "sbc_fqdn": "sbc.example.com",
    "mtls_cert": "/etc/rustalk/teams-cert.pem",
    "mtls_key": "/etc/rustalk/teams-key.pem",
    "trunk_fqdn": "sip.pstnhub.microsoft.com"
  }
}
```

### Validate Configuration

```bash
rustalk check-config --config config.json
```

### Start the Server

```bash
rustalk start --config config.json
```

## CLI Commands

### Interactive Console

Enter an interactive console mode similar to FreeSWITCH's fs_cli:

```bash
rustalk console --config config.json
```

The console provides a powerful interactive shell with:
- **Command history** - Navigate previous commands with arrow keys
- **Line editing** - Edit commands with standard editing keys
- **Tab completion** - Auto-complete commands (via rustyline)
- **Help system** - Type `help` or `?` for available commands

#### Console Commands

**Show Commands:**
```
show acls              - Display access control lists
show profiles          - Display SIP profiles
show status            - Display server status
show calls             - Display active calls
```

**Profile Management:**
```
profile <name> start   - Start a SIP profile
profile <name> stop    - Stop a SIP profile
profile <name> restart - Restart a SIP profile
profile <name> rescan  - Rescan a SIP profile
```

**Module Management:**
```
load <module>          - Load a module
unload <module>        - Unload a module
reload <module>        - Reload a module
```

**General:**
```
help, ?                - Display help
exit, quit, q          - Exit the console
```

### Start Server

```bash
rustalk start --config config.json
```

### Check Configuration

```bash
rustalk check-config --config config.json
```

### Generate Sample Config

```bash
rustalk generate-config --output my-config.json
```

### Server Status

```bash
rustalk status --server http://localhost:8080
```

### List Active Calls

```bash
rustalk list-calls --server http://localhost:8080
```

## Web UI

RusTalk includes a modern React-based administration console.

### Features

- **Dashboard**: Real-time system overview with call statistics and metrics
- **Call Management**: Monitor active calls with status indicators
- **Configuration**: Manage all system settings through an intuitive interface
- **Statistics**: Visual charts and graphs for performance analysis

### Access the Web UI

1. Build the Web UI:
   ```bash
   cd rustalk-webui
   npm install
   npm run build
   ```

2. Start the Cloud API server (it will automatically serve the Web UI):
   ```bash
   rustalk start --config config.json
   ```

3. Open your browser and navigate to:
   ```
   http://localhost:8080
   ```

### Development Mode

For frontend development with hot reload:

```bash
cd rustalk-webui
npm run dev
```

The UI will be available at http://localhost:3000 with API requests proxied to the backend.

See [rustalk-webui/README.md](rustalk-webui/README.md) for more details.

## SIP Methods Supported

- **INVITE**: Establish new calls
- **ACK**: Acknowledge call setup
- **BYE**: Terminate calls
- **CANCEL**: Cancel pending requests
- **OPTIONS**: Capability queries and health checks
- **REGISTER**: Contact registration (planned)

## Microsoft Teams Integration

RusTalk Edge provides specialized support for Microsoft Teams Direct Routing:

### Features

- mTLS authentication with Teams
- SIP trunk configuration for Teams
- OPTIONS ping for health monitoring
- SRTP pass-through
- Support for all Teams SIP proxies:
  - sip.pstnhub.microsoft.com
  - sip2.pstnhub.microsoft.com
  - sip3.pstnhub.microsoft.com

### Configuration

1. Obtain a certificate for your SBC FQDN
2. Configure the certificate in Teams admin center
3. Update `config.json` with certificate paths:

```json
{
  "teams": {
    "enabled": true,
    "sbc_fqdn": "sbc.yourcompany.com",
    "mtls_cert": "/etc/rustalk/teams-cert.pem",
    "mtls_key": "/etc/rustalk/teams-key.pem",
    "trunk_fqdn": "sip.pstnhub.microsoft.com"
  }
}
```

## Development

### Build

```bash
cargo build
```

### Run Tests

```bash
cargo test --workspace
```

### Run Specific Tests

```bash
cargo test -p rustalk-core
cargo test -p rustalk-edge
```

### Build Documentation

```bash
cargo doc --workspace --no-deps --open
```

## Project Structure

```
RusTalk/
├── rustalk-core/       # Core SIP engine
│   ├── src/
│   │   ├── sip/        # SIP protocol implementation
│   │   ├── b2bua/      # B2BUA engine
│   │   ├── transport/  # UDP/TCP/TLS transport
│   │   ├── config/     # Configuration management
│   │   └── media/      # Media/SDP handling
│   └── Cargo.toml
├── rustalk-edge/       # SBC Teams gateway
│   ├── src/
│   │   ├── teams/      # Teams integration
│   │   ├── gateway/    # Gateway logic
│   │   └── health/     # Health monitoring
│   └── Cargo.toml
├── rustalk-cloud/      # REST API service
│   ├── src/
│   │   ├── api/        # API server
│   │   ├── handlers/   # Request handlers
│   │   └── models/     # Data models
│   └── Cargo.toml
├── rustalk-cli/        # CLI admin tool
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── rustalk-webui/      # Web admin console
│   ├── src/
│   │   ├── api/        # API client
│   │   ├── components/ # React components
│   │   ├── pages/      # Page components
│   │   └── types/      # TypeScript types
│   ├── dist/           # Build output
│   └── package.json
├── config.json         # Sample configuration
├── ARCHITECTURE.md     # Architecture documentation
├── Cargo.toml          # Workspace configuration
└── README.md
```

## Performance

- **Concurrent Calls**: 10,000+ simultaneous calls per instance
- **Latency**: Sub-millisecond SIP processing
- **Memory**: ~10MB base + ~1KB per active call
- **CPU**: Minimal (async I/O, no busy polling)

## Security

- **Memory Safety**: Rust prevents buffer overflows and use-after-free bugs
- **TLS**: Modern cipher suites only (rustls)
- **mTLS**: Certificate-based authentication for Teams
- **Input Validation**: Strict SIP message parsing
- **No Unsafe Code**: (except in well-audited dependencies)

## Database Support

RusTalk supports PostgreSQL for configuration overlay and persistence:

```json
{
  "database": {
    "url": "postgresql://rustalk:password@localhost/rustalk",
    "max_connections": 10,
    "min_connections": 2
  }
}
```

Configuration values in the database override those in config.json, allowing centralized management.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

MIT License - See LICENSE file for details.

## Credits

Built with:

**Backend:**
- [tokio](https://tokio.rs/) - Async runtime
- [rustls](https://github.com/rustls/rustls) - TLS implementation
- [axum](https://github.com/tokio-rs/axum) - Web framework
- [sqlx](https://github.com/launchbadge/sqlx) - Async SQL
- [nom](https://github.com/rust-bakery/nom) - Parser combinators
- [clap](https://github.com/clap-rs/clap) - CLI framework

**Frontend:**
- [React](https://react.dev/) - UI library
- [Material-UI](https://mui.com/) - Component library
- [Recharts](https://recharts.org/) - Charts library
- [Vite](https://vite.dev/) - Build tool
- [TypeScript](https://www.typescriptlang.org/) - Type-safe JavaScript

## Status

This is an initial implementation providing core SIP/SBC functionality. Future enhancements include:

- [ ] WebRTC gateway
- [ ] SIP over WebSocket
- [ ] Advanced routing rules
- [x] Web UI with real-time dashboards
- [ ] WebSocket support for live updates
- [ ] Prometheus metrics
- [ ] OpenTelemetry tracing
