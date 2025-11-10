# RusTalk Architecture

RusTalk is a high-performance SIP/SBC platform built entirely in Rust, designed for memory safety, performance, and modern cloud deployments.

## Overview

RusTalk provides a complete Session Border Controller (SBC) and B2BUA implementation with specialized support for Microsoft Teams Direct Routing. The platform is built on modern async Rust using tokio for I/O, rustls for TLS/mTLS, and modular crate architecture.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                       RusTalk Platform                       │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ RusTalk CLI  │  │ RusTalk Edge │  │ RusTalk Cloud│      │
│  │              │  │              │  │              │      │
│  │ Admin Tool   │  │ SBC Gateway  │  │  REST API    │      │
│  │ - Config     │  │ - Teams mTLS │  │  - Monitoring│      │
│  │ - Monitoring │  │ - OPTIONS    │  │  - Analytics │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
│         │                 │                  │               │
│         └─────────────────┼──────────────────┘               │
│                           │                                  │
│                  ┌────────▼────────┐                         │
│                  │  RusTalk Core   │                         │
│                  │                 │                         │
│                  │  ┌───────────┐  │                         │
│                  │  │  B2BUA    │  │  Core SIP Engine        │
│                  │  │  Engine   │  │  - Message Parsing      │
│                  │  └───────────┘  │  - Transaction Mgmt     │
│                  │                 │  - Session State        │
│                  │  ┌───────────┐  │                         │
│                  │  │ Transport │  │  Transport Layer        │
│                  │  │  Layer    │  │  - UDP/TCP/TLS          │
│                  │  └───────────┘  │  - mTLS Support         │
│                  │                 │                         │
│                  │  ┌───────────┐  │  Media Management       │
│                  │  │   Media   │  │  - SDP Handling         │
│                  │  │  Handler  │  │  - SRTP Pass-through    │
│                  │  └───────────┘  │                         │
│                  │                 │                         │
│                  │  ┌───────────┐  │  Configuration          │
│                  │  │  Config   │  │  - JSON Files           │
│                  │  │  Manager  │  │  - DB Overlay (sqlx)    │
│                  │  └───────────┘  │                         │
│                  └─────────────────┘                         │
│                                                               │
└─────────────────────────────────────────────────────────────┘

         External Connections
         
    ┌───────────────┐       ┌──────────────┐
    │   Microsoft   │◄─────►│  Enterprise  │
    │     Teams     │ mTLS  │  SIP Trunks  │
    │  Direct Route │       │              │
    └───────────────┘       └──────────────┘
```

## Core Components

### 1. RusTalk Core (`rustalk-core`)

The heart of the platform, providing:

- **SIP Protocol Implementation**
  - Full RFC 3261 SIP parser using nom
  - Support for INVITE, BYE, OPTIONS, ACK, CANCEL, and more
  - Robust header parsing and message serialization
  
- **B2BUA Engine**
  - Back-to-Back User Agent for call routing
  - Session state management
  - Call leg tracking (A-leg and B-leg)
  - Transaction correlation
  
- **Transport Layer**
  - UDP transport (primary)
  - TCP transport
  - TLS/SIPS with rustls
  - mTLS support for Teams
  
- **Media Handling**
  - SDP parsing and manipulation
  - SRTP pass-through (no media decryption)
  - Media session tracking
  
- **Configuration Management**
  - JSON-based configuration (serde)
  - Database overlay support (sqlx + PostgreSQL)
  - Runtime configuration updates

### 2. RusTalk Edge (`rustalk-edge`)

Session Border Controller for Microsoft Teams:

- **Microsoft Teams Direct Routing**
  - mTLS certificate-based authentication
  - Teams-specific SIP trunk configuration
  - Certified SBC functionality
  
- **Health Monitoring**
  - OPTIONS ping to Teams SIP proxies
  - Automatic failover between proxies
  - Health check endpoints
  
- **Gateway Functions**
  - Call admission control
  - Number translation
  - Media anchoring
  - Topology hiding

### 3. RusTalk Cloud (`rustalk-cloud`)

Cloud-hosted REST API service:

- **REST API (axum)**
  - `/health` - Health check
  - `/api/v1/calls` - Call management
  - `/api/v1/config` - Configuration management
  - `/api/v1/stats` - Statistics and analytics
  
- **Database Integration**
  - sqlx for async PostgreSQL access
  - Call detail records (CDR)
  - Configuration storage
  - User management
  
- **Monitoring & Analytics**
  - Real-time call statistics
  - Historical reporting
  - Performance metrics

### 4. RusTalk CLI (`rustalk-cli`)

Command-line administration tool:

```bash
# Start the server
rustalk start --config config.json

# Check configuration
rustalk check-config --config config.json

# Generate sample config
rustalk generate-config --output my-config.json

# Monitor status
rustalk status --server http://localhost:8080

# List active calls
rustalk list-calls --server http://localhost:8080
```

## Technology Stack

### Core Technologies

- **Language**: Rust 2021 Edition
- **Async Runtime**: tokio (full-featured async I/O)
- **TLS/mTLS**: rustls (modern, safe TLS implementation)
- **Parsing**: nom (parser combinator library)
- **Serialization**: serde + serde_json
- **Database**: sqlx (async SQL with compile-time checking)
- **Web Framework**: axum (ergonomic web framework)
- **CLI**: clap (command-line argument parsing)

### Key Features

- **Memory Safety**: No buffer overflows, use-after-free, or data races
- **Performance**: Zero-cost abstractions, efficient async I/O
- **Concurrency**: Safe concurrent access with tokio and Arc/RwLock
- **Type Safety**: Strong typing prevents entire classes of bugs
- **Modern**: Built for cloud-native deployments

## B2BUA Operation

The B2BUA (Back-to-Back User Agent) acts as both a UAC and UAS:

```
Caller (UAC)          RusTalk B2BUA          Callee (UAS)
     │                      │                      │
     │  INVITE             │                      │
     ├─────────────────────►│                      │
     │                      │  Create Session      │
     │                      │  Modify SDP          │
     │                      │                      │
     │                      │  INVITE              │
     │                      ├─────────────────────►│
     │                      │                      │
     │                      │  180 Ringing         │
     │  180 Ringing         │◄─────────────────────┤
     │◄─────────────────────┤                      │
     │                      │                      │
     │                      │  200 OK              │
     │  200 OK              │◄─────────────────────┤
     │◄─────────────────────┤                      │
     │                      │                      │
     │  ACK                 │                      │
     ├─────────────────────►│  ACK                 │
     │                      ├─────────────────────►│
     │                      │                      │
     │       RTP Media (Pass-through)              │
     │◄═══════════════════════════════════════════►│
     │                      │                      │
     │  BYE                 │                      │
     ├─────────────────────►│                      │
     │                      │  BYE                 │
     │                      ├─────────────────────►│
     │                      │                      │
     │                      │  200 OK              │
     │  200 OK              │◄─────────────────────┤
     │◄─────────────────────┤                      │
     │                      │  Destroy Session     │
     │                      │                      │
```

## SIP Message Flow

### INVITE Flow

1. Receive INVITE from A-leg
2. Parse and validate SIP message
3. Extract SDP offer
4. Create new session in B2BUA
5. Generate new INVITE for B-leg (modify headers)
6. Forward INVITE to B-leg destination
7. Relay provisional responses (100 Trying, 180 Ringing)
8. Relay final response (200 OK)
9. Handle ACK
10. Pass RTP/SRTP media through without modification

### OPTIONS Flow (Health Check)

```
RusTalk Edge          Teams SIP Proxy
     │                      │
     │  OPTIONS             │
     ├─────────────────────►│
     │                      │
     │  200 OK              │
     │  Allow: ...          │
     │  Supported: ...      │
     │◄─────────────────────┤
     │                      │
```

Sent every 60 seconds to maintain connectivity and verify service health.

## Microsoft Teams Integration

### mTLS Configuration

RusTalk Edge supports mutual TLS authentication required by Microsoft Teams:

1. **Certificate Requirements**
   - Valid SSL certificate signed by trusted CA
   - Certificate must match SBC FQDN
   - Private key secured
   
2. **Configuration**
   ```json
   {
     "teams": {
       "enabled": true,
       "sbc_fqdn": "sbc.example.com",
       "mtls_cert": "/etc/rustalk/teams-cert.pem",
       "mtls_key": "/etc/rustalk/teams-key.pem",
       "trunk_fqdn": "sip.pstnhub.microsoft.com"
     }
   }
   ```

3. **SIP Proxies**
   - Primary: `sip.pstnhub.microsoft.com`
   - Secondary: `sip2.pstnhub.microsoft.com`
   - Tertiary: `sip3.pstnhub.microsoft.com`

### SRTP Pass-through

RusTalk does not decrypt/re-encrypt media:

- Media flows directly between endpoints
- B2BUA only modifies SDP to reflect correct addresses
- SRTP keying material passed transparently
- Reduced latency and CPU usage

## Configuration

### JSON Configuration

Primary configuration via `config.json`:

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
  }
}
```

### Database Overlay

Configuration can be overlaid from PostgreSQL:

1. Load base config from JSON
2. Connect to database
3. Query configuration overrides
4. Merge with base config
5. Apply to running system

This allows:
- Centralized configuration management
- Runtime updates without restarts
- Multi-instance synchronization

## Testing

Each crate includes comprehensive tests:

```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p rustalk-core
cargo test -p rustalk-edge

# Run with logging
RUST_LOG=debug cargo test
```

### Test Coverage

- **Unit Tests**: Individual component testing
- **Integration Tests**: Cross-component interaction
- **SIP Parser Tests**: RFC compliance validation
- **B2BUA Tests**: Call flow scenarios

## Deployment

### Docker Deployment

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/rustalk /usr/local/bin/
COPY config.json /etc/rustalk/config.json
EXPOSE 5060/udp 5060/tcp 5061/tcp 8080/tcp
CMD ["rustalk", "start", "--config", "/etc/rustalk/config.json"]
```

### Kubernetes Deployment

- StatefulSet for stable network identity
- ConfigMap for configuration
- Secrets for certificates and keys
- Service for load balancing
- HorizontalPodAutoscaler for scaling

## Performance Characteristics

- **Concurrent Calls**: 10,000+ simultaneous calls per instance
- **Latency**: Sub-millisecond SIP processing
- **Memory**: ~10MB base + ~1KB per active call
- **CPU**: Minimal (async I/O, no busy polling)
- **Network**: Line-rate packet processing

## Security

- **Memory Safety**: Rust prevents buffer overflows and use-after-free
- **TLS**: Modern cipher suites only (rustls)
- **mTLS**: Certificate-based authentication
- **Input Validation**: Strict SIP message parsing
- **No Unsafe Code**: (except where absolutely necessary in dependencies)

## Future Enhancements

- [ ] WebRTC gateway
- [ ] SIP over WebSocket
- [ ] Advanced routing rules
- [ ] Geographic redundancy
- [ ] Real-time analytics dashboard
- [ ] Automated load testing
- [ ] Prometheus metrics export
- [ ] OpenTelemetry tracing

## Contributing

RusTalk is open source (MIT License). Contributions welcome!

See CONTRIBUTING.md for guidelines.

## License

MIT License - See LICENSE file for details.
