# GitHub Copilot Instructions for RusTalk

## How to Use These Instructions

These instructions help GitHub Copilot understand the RusTalk project and generate appropriate code. When working on tasks:

1. **Always run quality checks** before finalizing changes (build, test, clippy, fmt)
2. **Make minimal changes** - modify only what's necessary to accomplish the task
3. **Follow existing patterns** - maintain consistency with the current codebase
4. **Test incrementally** - build and test after each meaningful change
5. **Document your changes** - update relevant documentation when adding features

### Task Scope Guidelines

**Well-suited tasks for automation:**
- Bug fixes with clear reproduction steps
- Adding unit tests for existing code
- Updating documentation
- Implementing new SIP methods based on RFC specifications
- Adding new CLI commands
- Adding REST API endpoints
- Refactoring within a single module
- Fixing clippy warnings

**Tasks that need careful consideration:**
- Large-scale refactoring across multiple crates
- Changes to core B2BUA call flow logic
- Modifications to Teams integration (requires testing with actual Teams)
- Performance optimization (requires profiling and benchmarking)
- Security-critical code (TLS/mTLS configuration, certificate handling)

**When to ask for clarification:**
- The task requirements are ambiguous
- Multiple valid approaches exist and the preference isn't clear
- The task might break backward compatibility
- External dependencies or services are involved
- The scope seems too large for a single change

## Project Overview

RusTalk is a high-performance SIP B2BUA (Back-to-Back User Agent), PBX, and Microsoft Teams-compatible Session Border Controller (SBC) built entirely in Rust. The project emphasizes memory safety, performance, and modern cloud-native architecture.

**Key Technologies:**

**Backend:**
- Language: Rust 2021 Edition
- Async Runtime: tokio (full async I/O)
- TLS/mTLS: rustls (modern, safe TLS)
- Parser: nom (parser combinators)
- Serialization: serde + serde_json
- Database: sqlx (async PostgreSQL)
- Web Framework: axum
- CLI: clap

**Frontend:**
- Framework: React 19 with TypeScript
- UI Library: Material-UI v6
- Charts: Recharts
- Build Tool: Vite
- Router: React Router

## Project Structure

This is a Cargo workspace with four main crates plus a React-based Web UI:

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

### 5. `rustalk-webui/` - Web Admin Console
Modern React-based administration interface:
- **Dashboard**: Real-time system overview with metrics (`src/pages/Dashboard.tsx`)
- **Call Management**: Monitor active calls (`src/pages/Calls.tsx`)
- **Configuration**: Manage all settings (`src/pages/Configuration.tsx`)
- **Statistics**: Visual analytics with charts (`src/pages/Statistics.tsx`)
- **API Client**: Integration with REST API (`src/api/client.ts`)
- **Components**: Reusable UI components (`src/components/`)
- **TypeScript Types**: Shared type definitions (`src/types/`)

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

## Quality Gates and Development Workflow

### Required Quality Checks
Before submitting any code changes, ALL of the following must pass:

1. **Build**: `cargo build --workspace` must complete without errors
2. **Tests**: `cargo test --workspace` must pass with 0 failures
3. **Linting**: `cargo clippy --workspace` must pass (warnings are acceptable but should be minimized)
4. **Formatting**: Code should be formatted with `cargo fmt --all`

### Development Workflow

#### 1. Initial Setup
```bash
# Clone and build the project
git clone https://github.com/halcycon/RusTalk.git
cd RusTalk
cargo build --workspace
cargo test --workspace
```

#### 2. Making Changes
```bash
# Create a feature branch
git checkout -b feature/your-feature-name

# Make your changes iteratively
# After each meaningful change:
cargo build --workspace          # Verify it builds
cargo test --workspace           # Verify tests pass
cargo clippy --workspace         # Check for issues
cargo fmt --all                  # Format code
```

#### 3. Testing Your Changes
```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p rustalk-core

# Run tests with logging enabled
RUST_LOG=debug cargo test

# Run a specific test
cargo test test_name -- --nocapture
```

#### 4. Code Quality Checks
```bash
# Run clippy for all crates
cargo clippy --workspace

# Run clippy with strict warnings (recommended)
cargo clippy --workspace -- -D warnings

# Format all code
cargo fmt --all

# Check formatting without changing files
cargo fmt --all -- --check
```

#### 5. Running the Application
```bash
# Generate a sample configuration
cargo run -p rustalk-cli -- generate-config > my-config.json

# Check configuration validity
cargo run -p rustalk-cli -- check-config my-config.json

# Start the server
cargo run -p rustalk-cli -- start my-config.json

# Use the interactive console
cargo run -p rustalk-cli -- console
```

### Build Commands Reference
```bash
cargo build                    # Debug build (faster, includes debug symbols)
cargo build --release          # Release build (optimized, slower to build)
cargo build -p rustalk-core    # Build specific crate only
cargo build --all-features     # Build with all features enabled
```

### Test Commands Reference
```bash
cargo test --workspace         # Run all tests in all crates
cargo test -p rustalk-core     # Test specific crate
RUST_LOG=debug cargo test      # Test with logging enabled
cargo test -- --test-threads=1 # Run tests serially (for debugging)
cargo test -- --nocapture      # Show println! output
```

### Linting and Formatting
```bash
cargo clippy --workspace       # Run linter
cargo clippy --fix             # Auto-fix some issues
cargo fmt --all                # Format code
cargo fmt --all -- --check     # Check formatting without modifying
```

## Common Development Scenarios

### Scenario 1: Adding a New Configuration Option
1. Add the field to the appropriate struct in `rustalk-core/src/config/mod.rs`
2. Update the `Default` implementation if needed
3. Add serialization/deserialization tests
4. Update the example `config.json` in the repository root
5. Document the new option in code comments

### Scenario 2: Adding a New SIP Header Parser
1. Add the header struct in `rustalk-core/src/sip/message.rs`
2. Add the parser function in `rustalk-core/src/sip/parser.rs` using nom
3. Add unit tests for valid and invalid header formats
4. Update the message parsing logic to include the new header
5. Document which RFC the header is from

### Scenario 3: Adding a New CLI Command
1. Define the command in `rustalk-cli/src/main.rs` using clap's derive macros
2. Implement the command handler function
3. Add error handling with anyhow::Context
4. Add integration test in the CLI tests
5. Update help text with examples

### Scenario 4: Adding a New REST API Endpoint
1. Define the route in `rustalk-cloud/src/api/mod.rs`
2. Create the handler in `rustalk-cloud/src/handlers/mod.rs`
3. Define request/response models in `rustalk-cloud/src/models/`
4. Add input validation
5. Add tests for the endpoint
6. Document the endpoint behavior in code comments

### Scenario 5: Debugging Test Failures
1. Run tests with logging: `RUST_LOG=debug cargo test -- --nocapture`
2. Run specific test: `cargo test test_name -- --nocapture`
3. Add additional debug tracing statements temporarily
4. Check if external dependencies (PostgreSQL) are needed
5. Verify test data setup and cleanup

## Troubleshooting

### Common Issues
1. **"future cannot be sent between threads safely"**
   - Ensure all types in async functions implement Send
   - Use Arc<Mutex<T>> or Arc<RwLock<T>> for shared mutable state
   - Avoid holding locks across await points

2. **"database connection failed"**
   - PostgreSQL is optional - code should work without database
   - Check database URL in config.json
   - Verify PostgreSQL is running if database features are used

3. **"certificate verification failed"**
   - Ensure certificate paths are correct in config
   - Verify certificates are in PEM format
   - Check that certificate matches the expected domain

4. **"unused import" or "unused variable" warnings**
   - Remove unused imports to keep code clean
   - Prefix intentionally unused variables with underscore: `_variable`
   - Run `cargo clippy --fix` to auto-fix some issues

5. **Test failures in CI but passes locally**
   - Check for timing-dependent tests
   - Verify all dependencies are specified in Cargo.toml
   - Check for platform-specific behavior

## Best Practices for This Project

### Code Quality
1. **Always use tracing macros** (`debug!`, `info!`, `warn!`, `error!`) instead of `println!`
2. **Prefer &str over String** for function parameters when ownership not needed
3. **Use workspace dependencies** - don't add version directly in crate Cargo.toml
4. **Avoid `unsafe` code** - only use in audited dependencies, never in project code
5. **Handle errors with anyhow** - use `?` operator and `.context()` for error propagation
6. **Keep functions small** - aim for functions under 50 lines when possible

### Testing and Validation
7. **Write tests for B2BUA logic** - call flows are complex and need validation
8. **Test both success and error paths** - ensure error handling works correctly
9. **Mock external dependencies** - don't rely on PostgreSQL or external services in unit tests
10. **Run tests frequently** - test after each meaningful change

### Domain-Specific (SIP/Telecom)
11. **Document SIP RFC compliance** - note which RFCs are implemented
12. **Handle OPTIONS gracefully** - used for health checks by Teams and SIP trunks
13. **Validate all SIP input** - malformed messages can cause issues
14. **Keep session state minimal** - memory per call should be ~1KB
15. **Log important events** - call setup, teardown, errors, Teams connectivity

### Performance and Async
16. **Never block async code** - use tokio::spawn_blocking for blocking operations
17. **Use Arc for shared immutable data** - avoid unnecessary cloning
18. **Prefer RwLock read locks** - write locks should be held briefly
19. **Zero-copy when possible** - use &str and &[u8] over owned types
20. **Profile before optimizing** - measure first, optimize second

## Security Considerations

1. **Input Validation**: All SIP messages are untrusted input
2. **Certificate Handling**: Never expose private keys in logs
3. **Memory Safety**: Rust prevents most memory vulnerabilities
4. **TLS Configuration**: Use modern cipher suites only
5. **mTLS for Teams**: Required for production Teams integration

## Working with the Web UI

### Development Workflow
```bash
cd rustalk-webui
npm install              # Install dependencies
npm run dev             # Start dev server on http://localhost:3000
npm run build           # Build for production
npm run lint            # Lint TypeScript/React code
```

### Web UI Architecture
- **React 19** with functional components and hooks
- **Material-UI v6** for consistent UI design
- **TypeScript** for type safety
- **Vite** for fast builds and HMR
- **Recharts** for data visualization
- **React Router** for client-side routing

### Key Files
- `src/App.tsx` - Main app with routing
- `src/components/Layout.tsx` - Sidebar navigation layout
- `src/pages/*.tsx` - Page components for each route
- `src/api/client.ts` - Axios client for REST API
- `src/types/index.ts` - TypeScript type definitions
- `vite.config.ts` - Vite configuration with API proxy

### Best Practices for Web UI
1. **Use Material-UI components** - Don't create custom styled components unnecessarily
2. **Keep API calls in pages** - Components should receive props
3. **Use TypeScript types** - Always type props, state, and API responses
4. **Follow React hooks rules** - Don't call hooks conditionally
5. **Handle loading states** - Show CircularProgress while fetching data
6. **Handle error states** - Display Alert components for errors
7. **Auto-refresh data** - Use setInterval for real-time updates
8. **Responsive design** - Use Material-UI Grid with xs/md/lg breakpoints

### Integrating Frontend Changes
When the backend API changes:
1. Update `src/types/index.ts` with new types
2. Update `src/api/client.ts` with new endpoints
3. Update relevant page components to use new data
4. Test with the development proxy

### Serving Web UI from Rust
The `rustalk-cloud` API server serves the built Web UI:
```rust
let api = CloudApi::new("0.0.0.0:8080".parse().unwrap())
    .with_webui_path("rustalk-webui/dist".to_string());
```

## VS Code Extensions Recommended

**Rust Development:**
- rust-analyzer: Rust language support
- CodeLLDB: Debugging support
- Even Better TOML: Cargo.toml editing
- Error Lens: Inline error display

**Web UI Development:**
- ESLint: JavaScript/TypeScript linting
- Prettier: Code formatting
- TypeScript Vue Plugin: TypeScript support
- ES7+ React/Redux/React-Native snippets: React snippets

**General:**
- GitLens: Git integration

## Tools and Resources

### Development Tools
- **rust-analyzer**: Essential VS Code extension for Rust development
- **cargo-watch**: Auto-rebuild on file changes: `cargo install cargo-watch`
- **cargo-edit**: Manage dependencies: `cargo install cargo-edit`
- **cargo-expand**: See macro expansions: `cargo install cargo-expand`

### Useful Cargo Commands
```bash
# Watch and rebuild on changes
cargo watch -x build

# Watch and run tests
cargo watch -x test

# Check without building (faster)
cargo check --workspace

# Build documentation
cargo doc --open --no-deps

# View dependency tree
cargo tree -p rustalk-core
```

### Debugging Tips
1. Use `RUST_LOG=trace` for maximum logging verbosity
2. Use `dbg!()` macro for quick debugging (remember to remove it)
3. Use `cargo expand` to see what macros generate
4. Use `--nocapture` with tests to see println! output
5. Add `#[ignore]` to skip long-running tests during development

### Additional Resources

#### SIP and VoIP Specifications
- [RFC 3261](https://datatracker.ietf.org/doc/html/rfc3261) - SIP Protocol (essential)
- [RFC 4566](https://datatracker.ietf.org/doc/html/rfc4566) - SDP
- [RFC 3711](https://datatracker.ietf.org/doc/html/rfc3711) - SRTP
- [RFC 3262](https://datatracker.ietf.org/doc/html/rfc3262) - Reliability of Provisional Responses (PRACK)
- [RFC 3264](https://datatracker.ietf.org/doc/html/rfc3264) - Offer/Answer Model with SDP

#### Microsoft Teams Integration
- [Microsoft Teams Direct Routing](https://learn.microsoft.com/en-us/microsoftteams/direct-routing-landing-page)
- [Plan Direct Routing](https://learn.microsoft.com/en-us/microsoftteams/direct-routing-plan)
- [Configure SBC for Direct Routing](https://learn.microsoft.com/en-us/microsoftteams/direct-routing-configure)

#### Project Documentation
- See ARCHITECTURE.md for detailed architecture documentation
- See README.md for quick start guide
- See individual crate README files (if present) for crate-specific details

#### Rust Learning Resources
- [The Rust Book](https://doc.rust-lang.org/book/) - Official Rust documentation
- [Rust Async Book](https://rust-lang.github.io/async-book/) - Understanding async/await
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial) - Async runtime documentation

## Optional: Custom Agents

Custom agents are specialized Copilot configurations for specific types of tasks. They are optional but can be useful for complex or repetitive workflows. If needed in the future, custom agents can be added to `.github/agents/` directory.

### Potential Custom Agents for RusTalk

**Example: SIP Protocol Expert**
Could handle complex SIP message parsing, RFC compliance validation, and protocol-specific implementations.

**Example: Async Rust Expert**
Could specialize in tokio patterns, async/await optimization, and concurrent programming patterns.

**Example: Test Generator**
Could focus on generating comprehensive test suites, including unit tests, integration tests, and edge cases.

### How to Add Custom Agents

To add a custom agent:
1. Create `.github/agents/` directory
2. Add a markdown file for each agent (e.g., `sip-expert.md`)
3. Include agent metadata and instructions:
```markdown
---
name: sip-expert
description: Expert in SIP protocol implementation and RFC compliance
---

You are a specialized agent for SIP protocol implementation...
[Detailed instructions for the agent]
```

For more information, see:
- [GitHub Docs: Custom Agents](https://docs.github.com/en/copilot/customizing-copilot/adding-custom-instructions-for-github-copilot)
- [Best practices for custom agents](https://github.blog/ai-and-ml/github-copilot/from-chaos-to-clarity-using-github-copilot-agents-to-improve-developer-workflows/)

**Note:** The current comprehensive instructions should be sufficient for most development tasks. Custom agents should only be added if there's a clear need for specialized expertise in specific domains.
