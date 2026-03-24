#!/bin/bash
set -e

# Install the Jaffle Shop Generator
# After running this, you can use `jafgen` from any directory.

if ! command -v cargo &> /dev/null; then
    echo "Rust is not installed. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

echo "Building and installing jafgen..."
cargo install --path . --locked

echo ""
echo "Done! Run 'jafgen' from any directory to start the generator."
echo "It will open http://localhost:3000 in your browser automatically."
echo "Generated CSV files will be written to ./jaffle-data/ in your current directory."
