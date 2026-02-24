#!/usr/bin/env bash
set -euo pipefail

REPO="tailwindlabs/heroicons"
BRANCH="master"

echo "Fetching latest commit from $REPO/$BRANCH..."
TARBALL_URL="https://github.com/$REPO/archive/refs/heads/$BRANCH.tar.gz"

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

echo "Downloading tarball..."
curl -sL "$TARBALL_URL" -o "$TMPDIR/heroicons.tar.gz"

echo "Extracting optimized SVGs..."
tar -xzf "$TMPDIR/heroicons.tar.gz" -C "$TMPDIR"

EXTRACTED="heroicons-$BRANCH"

SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)

rm -rf "$SCRIPT_DIR/optimized"
mv "$TMPDIR/$EXTRACTED/optimized" "$SCRIPT_DIR/optimized"

COUNT=$(find "$SCRIPT_DIR/optimized" -name '*.svg' | wc -l | tr -d ' ')
echo "Updated $COUNT SVGs in optimized/"
