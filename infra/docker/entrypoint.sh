#!/bin/bash
set -e

cleanup() {
    echo "Shutting down..."
    kill "$SERVER_PID" "$WEB_PID" "$NGINX_PID" 2>/dev/null || true
    wait "$SERVER_PID" "$WEB_PID" "$NGINX_PID" 2>/dev/null || true
    exit 0
}

trap cleanup TERM INT

# Start the Rust API server
amaterasu-server &
SERVER_PID=$!

# Start the SvelteKit Bun server
cd /app/web
PORT=3001 ORIGIN=http://127.0.0.1:3000 API_ORIGIN=http://127.0.0.1:8080 bun run index.js &
WEB_PID=$!
cd /

# Start nginx in foreground mode (but backgrounded so we can wait)
nginx -g 'daemon off;' &
NGINX_PID=$!

# Wait for any process to exit — if one dies, bring down the container
wait -n "$SERVER_PID" "$WEB_PID" "$NGINX_PID" 2>/dev/null
EXIT_CODE=$?

echo "Process exited with code $EXIT_CODE, shutting down..."
cleanup
exit $EXIT_CODE
