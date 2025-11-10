# GitHub Copilot Instructions for RusTalk

## Project Overview

RusTalk is a high-performance SIP B2BUA (Back-to-Back User Agent), PBX, and Microsoft Teams-compatible Session Border Controller (SBC) built entirely in Rust. The project emphasizes memory safety, performance, and modern cloud-native architecture.

**Key Technologies:**
- Language: Rust 2021 Edition
- Async Runtime: tokio (full async I/O)
- TLS/mTLS: rustls (modern, safe TLS)
- Parser: nom (parser combinators)
- Serialization: serde + serde_json
- Database: sqlx (async PostgreSQL)
- Web Framework: axum
- CLI: clap

## Project Structure

This is a Cargo workspace with four main crates:

### 1. `rustalk-core/` - Core SIP Engine
The heart of the platform, providing:
- **SIP Protocol Implementation**: Full RFC 3261 SIP parser using nom
  - Supports: INVITE, BYE, OPTIONS, ACK, CANCEL
  - Location: `src/sip/`
- **B2BUA Engine**: Back-to-Back User Agent for call routing
  - Session state management
  - Call leg tracking (A-leg and B-leg)
  - Location: `src/b2bua/`
- **Transport Layer**: UDP/TCP/TLS/mTLS support
  - Location: `src/transport/`
- **Media Handling**: SDP parsing and SRTP pass-through
  - Location: `src/media/`
- **Configuration Management**: JSON-based config with database overlay
  - Location: `src/config/`

### 2. `rustalk-edge/` - Session Border Controller
Microsoft Teams Direct Routing support:
- **Teams Integration**: mTLS authentication with Teams (`src/teams/`)
- **Gateway Functions**: Call admission control, topology hiding (`src/gateway/`)
- **Health Monitoring**: OPTIONS ping to Teams SIP proxies (`src/health/`)

### 3. `rustalk-cloud/` - REST API Service
Cloud management and monitoring:
- **REST API**: axum-based HTTP server (`src/api/`)
- **Request Handlers**: API endpoint implementations (`src/handlers/`)
- **Data Models**: API and database models (`src/models/`)
- Endpoints: `/health`, `/api/v1/calls`, `/api/v1/config`, `/api/v1/stats`

### 4. `rustalk-cli/` - Command-Line Tool
Administration tool with commands:
- `start` - Start the server
- `check-config` - Validate configuration
- `generate-config` - Generate sample config
- `console` - Interactive console mode
- `status` - Monitor server status
- `list-calls` - Show active calls

## Architecture Principles

### Memory Safety First
- No `unsafe` code in project code (only in audited dependencies)
- Leverage Rust's ownership system for safety
- Use Arc/RwLock for safe concurrent access
- Avoid raw pointers and manual memory management

### Async Everything
- All I/O operations use tokio async
- Use async/await syntax consistently
- Avoid blocking operations in async contexts
- Use tokio::spawn for concurrent tasks

### Error Handling
- Use `anyhow::Result` for application errors
- Use `anyhow::Context` for error context
- Propagate errors with `?` operator
- Log errors with tracing macros

### Modular Design
- Each crate has a specific responsibility
- Minimize dependencies between crates
- Use workspace dependencies for consistency
- Keep public APIs minimal and well-documented

## Coding Conventions

### Naming
- Use snake_case for functions, variables, modules
- Use PascalCase for types, traits, enums
- Use SCREAMING_SNAKE_CASE for constants
- Prefix async functions with meaningful verbs (e.g., `handle_`, `process_`, `send_`)

### Code Organization
- Group related functionality in modules
- Keep files under 500 lines when possible
- Use `mod.rs` for module public exports
- Place tests in same file using `#[cfg(test)]` modules

### Documentation
- Add doc comments (`///`) for public APIs
- Include examples in doc comments where helpful
- Document complex algorithms inline
- Keep comments up-to-date with code changes

### Testing
- Write unit tests for core logic
- Use descriptive test names: `test_<scenario>_<expected_behavior>`
- Test both success and error paths
- Mock external dependencies when possible

## SIP Protocol Knowledge

### Message Flow (B2BUA)
When handling SIP messages:
1. Parse incoming message with nom parser (`rustalk-core/src/sip/parser.rs`)
2. Validate headers and body
3. Extract relevant information (Call-ID, From, To, etc.)
4. Create/update session state in B2BUA
5. Modify headers for outbound leg
6. Forward to destination
7. Relay responses back to originator

### Key SIP Concepts
- **Dialog**: A peer-to-peer SIP relationship (identified by Call-ID, From-tag, To-tag)
- **Transaction**: A request and its responses
- **B2BUA**: Acts as both client and server, creates two call legs
- **SDP**: Session Description Protocol for media negotiation
- **SRTP**: Secure RTP for encrypted media

## Common Tasks

### Adding a New SIP Method
1. Add parser in `rustalk-core/src/sip/parser.rs`
2. Add method handler in `rustalk-core/src/sip/mod.rs`
3. Update B2BUA logic in `rustalk-core/src/b2bua/`
4. Add tests for the new method
5. Update documentation

### Adding a New API Endpoint
1. Define route in `rustalk-cloud/src/api/mod.rs`
2. Implement handler in `rustalk-cloud/src/handlers/`
3. Add data model in `rustalk-cloud/src/models/` if needed
4. Update API documentation

### Adding a New CLI Command
1. Add command definition in `rustalk-cli/src/main.rs` using clap
2. Implement command handler
3. Add help text and examples
4. Update README.md

## Configuration

### config.json Structure
The main configuration file includes:
- `server`: Bind address, port, workers
- `sip`: Domain, user agent, session settings
- `transport`: Protocol configuration (UDP/TCP/TLS)
- `teams`: Microsoft Teams integration settings
- `database`: PostgreSQL connection (optional)

### Database Overlay
Configuration can be overlaid from PostgreSQL:
- Base config loaded from JSON
- Database overrides merged at runtime
- Allows centralized management across instances

## Microsoft Teams Integration

### mTLS Requirements
- Valid SSL certificate signed by trusted CA
- Certificate must match SBC FQDN
- Configure paths in `teams.mtls_cert` and `teams.mtls_key`
- Teams SIP proxies: sip.pstnhub.microsoft.com (+ sip2, sip3)

### SRTP Pass-through
RusTalk does not decrypt media:
- Media flows directly between endpoints
- B2BUA only modifies SDP addresses
- SRTP keying material passed transparently
- Reduced latency and CPU usage

## Performance Considerations

### Target Metrics
- 10,000+ simultaneous calls per instance
- Sub-millisecond SIP processing latency
- ~10MB base + ~1KB per active call memory
- Minimal CPU usage (async I/O, no busy polling)

### Optimization Guidelines
- Use zero-copy parsing where possible
- Avoid unnecessary allocations
- Prefer borrowed types (&str, &[u8]) over owned (String, Vec)
- Use Arc for shared immutable data
- Use RwLock for shared mutable data (prefer read locks)

## Dependencies

### Core Dependencies
- `tokio`: Async runtime - use for all async operations
- `rustls`: TLS implementation - use for secure connections
- `nom`: Parser combinators - use for SIP message parsing
- `serde`/`serde_json`: Serialization - use for config and API
- `sqlx`: Database access - use for PostgreSQL queries
- `anyhow`: Error handling - use for application errors
- `tracing`: Logging - use instead of println!
- `axum`: Web framework - use for REST API
- `clap`: CLI parsing - use for command-line interface

### Adding New Dependencies
- Prefer async-compatible crates
- Check security advisories before adding
- Use workspace dependencies for consistency
- Document reason for new dependency

## Building and Testing

### Build Commands
```bash
cargo build                    # Debug build
cargo build --release          # Release build
cargo build -p rustalk-core    # Build specific crate
```

### Test Commands
```bash
cargo test --workspace         # Run all tests
cargo test -p rustalk-core     # Test specific crate
RUST_LOG=debug cargo test      # Test with logging
```

### Linting
```bash
cargo clippy --workspace       # Run linter
cargo fmt --all                # Format code
```

## Troubleshooting

### Common Issues
1. **"future cannot be sent between threads safely"**
   - Ensure all types in async functions implement Send
   - Use Arc<Mutex<T>> or Arc<RwLock<T>> for shared mutable state

2. **"database connection failed"**
   - PostgreSQL is optional - code should work without database
   - Check database URL in config.json

3. **"certificate verification failed"**
   - Ensure certificate paths are correct in config
   - Verify certificates are in PEM format

## Best Practices for This Project

1. **Always use tracing macros** (`debug!`, `info!`, `warn!`, `error!`) instead of `println!`
2. **Prefer &str over String** for function parameters when ownership not needed
3. **Use workspace dependencies** - don't add version directly in crate Cargo.toml
4. **Write tests for B2BUA logic** - call flows are complex and need validation
5. **Document SIP RFC compliance** - note which RFCs are implemented
6. **Handle OPTIONS gracefully** - used for health checks by Teams and SIP trunks
7. **Never block async code** - use tokio::spawn_blocking for blocking operations
8. **Validate all SIP input** - malformed messages can cause issues
9. **Keep session state minimal** - memory per call should be ~1KB
10. **Log important events** - call setup, teardown, errors, Teams connectivity

## Security Considerations

1. **Input Validation**: All SIP messages are untrusted input
2. **Certificate Handling**: Never expose private keys in logs
3. **Memory Safety**: Rust prevents most memory vulnerabilities
4. **TLS Configuration**: Use modern cipher suites only
5. **mTLS for Teams**: Required for production Teams integration

## VS Code Extensions Recommended

- rust-analyzer: Rust language support
- CodeLLDB: Debugging support
- Even Better TOML: Cargo.toml editing
- Error Lens: Inline error display
- GitLens: Git integration

## Additional Resources

- [RFC 3261](https://datatracker.ietf.org/doc/html/rfc3261) - SIP Protocol
- [RFC 4566](https://datatracker.ietf.org/doc/html/rfc4566) - SDP
- [RFC 3711](https://datatracker.ietf.org/doc/html/rfc3711) - SRTP
- [Microsoft Teams Direct Routing](https://learn.microsoft.com/en-us/microsoftteams/direct-routing-landing-page)
- See ARCHITECTURE.md for detailed architecture documentation
- See README.md for quick start guide
