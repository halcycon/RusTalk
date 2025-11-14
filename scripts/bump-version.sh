#!/bin/bash
# Script to bump version numbers for RusTalk release

set -e

if [ $# -ne 1 ]; then
    echo "Usage: $0 <new-version>"
    echo "Example: $0 0.2.0"
    exit 1
fi

NEW_VERSION=$1

# Validate version format (basic semver check)
if ! [[ $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+)?$ ]]; then
    echo "Error: Invalid version format. Use format: MAJOR.MINOR.PATCH (e.g., 1.2.3 or 1.2.3-rc1)"
    exit 1
fi

echo "Bumping version to $NEW_VERSION..."

# Update workspace Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Update Cargo.lock
cargo update -p rustalk-core -p rustalk-edge -p rustalk-cloud -p rustalk-cli

# Remove backup file
rm -f Cargo.toml.bak

echo "✓ Version updated to $NEW_VERSION in Cargo.toml"
echo "✓ Cargo.lock updated"
echo ""
echo "Next steps:"
echo "  1. Review the changes: git diff"
echo "  2. Commit: git add Cargo.toml Cargo.lock && git commit -m 'Bump version to $NEW_VERSION'"
echo "  3. Tag: git tag -a v$NEW_VERSION -m 'Release version $NEW_VERSION'"
echo "  4. Push: git push origin main && git push origin v$NEW_VERSION"
