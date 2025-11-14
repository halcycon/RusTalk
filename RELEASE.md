# Release Process

This document describes how to create and publish releases for RusTalk.

## Overview

RusTalk uses GitHub Actions for automated building and releasing. The release workflow builds binaries for multiple platforms and creates GitHub releases with downloadable archives.

## Supported Platforms

- **Linux x86_64** (`.tar.gz`)
- **macOS x86_64** (`.tar.gz`)
- **macOS ARM64** (Apple Silicon) (`.tar.gz`)
- **Windows x86_64** (`.zip`)

## Release Types

The workflow automatically determines the release type based on the tag:

- **Stable Release**: Tags like `v1.0.0`, `v1.2.3`
- **Pre-release**: Tags containing `-rc`, `-beta`, or `-alpha` (e.g., `v1.0.0-rc1`, `v1.0.0-beta2`)

## Creating a Release

### 1. Prepare the Release

Before creating a release, ensure:

- All tests pass: `cargo test --workspace`
- Code is properly formatted: `cargo fmt --all`
- Clippy checks are addressed: `cargo clippy --workspace`
- Documentation is up to date
- CHANGELOG is updated (if you maintain one)

### 2. Update Version Numbers

Update the version in `Cargo.toml`:

```toml
[workspace.package]
version = "0.2.0"  # Update to your new version
```

Then update the lock file:

```bash
cargo update -p rustalk-core -p rustalk-edge -p rustalk-cloud -p rustalk-cli
```

### 3. Commit Version Update

```bash
git add Cargo.toml Cargo.lock
git commit -m "Bump version to 0.2.0"
git push origin main
```

### 4. Create and Push the Tag

```bash
# Create an annotated tag
git tag -a v0.2.0 -m "Release version 0.2.0"

# Push the tag to GitHub
git push origin v0.2.0
```

### 5. Monitor the Build

1. Go to the [Actions tab](https://github.com/halcycon/RusTalk/actions) on GitHub
2. Watch the "Release" workflow run
3. The workflow will:
   - Run tests on all platforms
   - Build release binaries for each platform
   - Create release archives
   - Publish a GitHub release with all artifacts

The entire process typically takes 15-30 minutes.

### 6. Verify the Release

1. Go to the [Releases page](https://github.com/halcycon/RusTalk/releases)
2. Check that the new release appears with all platform archives
3. Download and test one or more archives to verify they work

## Manual Release (if needed)

If you need to create a release manually or test the process locally:

### Build for Your Platform

```bash
# Build release binary
cargo build --release

# Create release directory
mkdir -p release/rustalk-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)
cd release/rustalk-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)

# Copy files
cp ../../target/release/rustalk .
cp ../../config.json config.example.json
cp ../../README.md ../../LICENSE .

# Create archive
cd ..
tar czf rustalk-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m).tar.gz \
    rustalk-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)
```

### Cross-Compilation (Advanced)

For cross-compiling to other platforms, use `cross`:

```bash
# Install cross
cargo install cross

# Build for Linux from macOS
cross build --release --target x86_64-unknown-linux-gnu

# Build for Windows from Linux/macOS
cross build --release --target x86_64-pc-windows-gnu
```

## Release Checklist

Before creating a release, verify:

- [ ] All tests pass on main branch
- [ ] Version numbers updated in Cargo.toml
- [ ] Cargo.lock updated with `cargo update`
- [ ] Documentation is current
- [ ] Breaking changes are documented
- [ ] Security advisories are addressed
- [ ] Tag follows semantic versioning (MAJOR.MINOR.PATCH)

After release:

- [ ] Verify release appears on GitHub
- [ ] All platform archives are present
- [ ] Download and test at least one archive
- [ ] Release notes are accurate
- [ ] Announce the release (if applicable)

## Semantic Versioning

RusTalk follows [Semantic Versioning](https://semver.org/):

- **MAJOR** version for incompatible API changes
- **MINOR** version for new functionality (backwards-compatible)
- **PATCH** version for backwards-compatible bug fixes

Examples:
- `v0.1.0` → `v0.2.0`: New features added
- `v0.2.0` → `v0.2.1`: Bug fixes
- `v0.2.1` → `v1.0.0`: First stable release
- `v1.0.0` → `v2.0.0`: Breaking changes

## Pre-releases

For testing before a stable release:

```bash
# Release candidate
git tag -a v1.0.0-rc1 -m "Release candidate 1 for version 1.0.0"
git push origin v1.0.0-rc1

# Beta release
git tag -a v1.0.0-beta1 -m "Beta 1 for version 1.0.0"
git push origin v1.0.0-beta1

# Alpha release
git tag -a v1.0.0-alpha1 -m "Alpha 1 for version 1.0.0"
git push origin v1.0.0-alpha1
```

These will be marked as pre-releases on GitHub.

## Troubleshooting

### Build Fails

If the release build fails:

1. Check the [Actions log](https://github.com/halcycon/RusTalk/actions) for errors
2. Verify tests pass locally: `cargo test --workspace`
3. Verify it builds locally: `cargo build --release`
4. Check for platform-specific issues

### Release Not Created

If the GitHub release isn't created automatically:

1. Verify the tag was pushed: `git ls-remote --tags origin`
2. Check the workflow ran: Look for the tag in the Actions tab
3. Verify the tag matches the pattern `v*`
4. Check workflow permissions in repository settings

### Missing Artifacts

If some platform archives are missing:

1. Check which build job failed in the Actions log
2. Re-run failed jobs from the Actions interface
3. If necessary, delete the release and tag, then recreate

## CI/CD Workflows

### CI Workflow

Runs on every push and pull request:
- Tests on Linux, macOS, and Windows
- Linting (rustfmt, clippy)
- Debug and release builds

### Release Workflow

Triggers on tags matching `v*`:
- Runs tests first
- Builds for all platforms in parallel
- Creates release packages
- Publishes GitHub release

## Distribution Channels

Currently, RusTalk is distributed via:

- **GitHub Releases**: Binary downloads for all platforms
- **Source**: Clone and build from the repository

Future distribution channels may include:

- **Cargo**: `cargo install rustalk`
- **Homebrew**: `brew install rustalk`
- **apt/yum**: Linux package repositories
- **Docker**: Container images

## Support

For questions about releases:
- Open an issue on [GitHub](https://github.com/halcycon/RusTalk/issues)
- Check existing [releases](https://github.com/halcycon/RusTalk/releases)
