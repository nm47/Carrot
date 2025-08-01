#!/bin/bash

# Development server with hot reload for Carrot

PORT=8001
echo "Starting Carrot development server on http://localhost:$PORT"
echo "Press Ctrl+C to stop"

# Function to rebuild WASM when Rust files change
rebuild_wasm() {
    echo "Rust files changed, rebuilding WASM..."
    wasm-pack build --target web --dev
    echo "WASM rebuild complete!"
}

# Function to handle cleanup
cleanup() {
    echo "Stopping development server..."
    kill $SERVER_PID 2>/dev/null
    kill $WATCHER_PID 2>/dev/null
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Build WASM initially
echo "Building WASM package..."
wasm-pack build --target web --dev
echo "Initial WASM build complete!"

# Start the proxy server in background
python3 proxy_server.py &
SERVER_PID=$!

# Watch for changes in Rust files and rebuild WASM
if command -v inotifywait &> /dev/null; then
    echo "Watching for Rust file changes..."
    while file=$(inotifywait -e modify --format '%w%f' src/ Cargo.toml 2>/dev/null); do
        echo "File changed: $file"
        # Skip vim temporary files and swap files
        if [[ "$file" == *".swp"* ]] || [[ "$file" == *".tmp"* ]] || [[ "$file" == *"~"* ]] || [[ "$file" == *".swo"* ]]; then
            echo "Skipping temporary file: $file"
            continue
        fi
        sleep 0.5  # Longer debounce for vim
        rebuild_wasm
    done &
    WATCHER_PID=$!
else
    echo "No file watcher found (inotifywait). Manual rebuild required."
    echo "Run 'wasm-pack build --target web --dev' after making changes to Rust code."
fi

echo "Development server started! Open http://localhost:$PORT in your browser"
echo "The server will automatically rebuild WASM when Rust files change"

# Wait for the server process
wait $SERVER_PID
