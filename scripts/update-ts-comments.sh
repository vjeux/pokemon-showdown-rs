#!/bin/bash

# Convenience script to add TypeScript source comments to all Rust methods
# Run this inside the Docker container

set -e

cd /home/builder/workspace

echo "================================================"
echo "Adding TypeScript Source Comments to Rust Files"
echo "================================================"
echo ""

# Check if pokemon-showdown repository exists
if [ ! -d "/home/builder/pokemon-showdown" ]; then
    echo "❌ Pokemon Showdown repository not found!"
    echo "   Cloning from GitHub..."
    cd /home/builder
    git clone https://github.com/smogon/pokemon-showdown.git
    echo "✓ Repository cloned"
    echo ""
fi

cd /home/builder/workspace

# Run the script
echo "Running comment addition script..."
node scripts/add-all-js-comments.js

echo ""
echo "================================================"
echo "✓ Done!"
echo "================================================"
echo ""
echo "To verify changes:"
echo "  git diff src/"
echo ""
echo "To see comment count:"
echo "  grep -r '// TypeScript source:' src/ | wc -l"
echo ""
