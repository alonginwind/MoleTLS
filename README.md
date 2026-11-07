# MoleTLS

🐭 A minimal HTTP reverse proxy with embedded WASM frontend, written in pure Rust.

## Features

- Reverse proxy with request forwarding
- Header passthrough with Host override
- Query string preservation
- Connection pooling (reuse HTTP client)
- Embedded WASM frontend (single binary deployment)
- Admin API (`/api/status`)

## Quick Start

```bash
# Build
./build.sh debug     # debug mode

# Run
./target/debug/mole-tls

# Open http://localhost:7999/
```

## Testing

```bash
# Frontend page
curl http://localhost:7999/

# Admin API
curl http://localhost:7999/api/status
# {"status":"running","backend":"httpbin.org","port":7999}

# Proxy test
curl http://localhost:7999/get
```

## Tech Stack

### Backend

| Crate | Purpose |
|-------|---------|
| tokio | Async runtime |
| axum | Web framework |
| hyper | HTTP protocol |
| hyper-util | HTTP client |
| rust-embed | Embed frontend assets |
| serde / serde_json | JSON serialization |
| tracing | Logging |

### Frontend

| Crate | Purpose |
|-------|---------|
| leptos | WASM UI framework |
| gloo-net | HTTP client for WASM |
| serde | JSON deserialization |

## Project Structure

```
MoleTLS/
├── Cargo.toml          # Workspace + backend dependencies
├── build.sh            # Build script (debug/release)
├── README.md
├── src/
│   └── main.rs         # Backend: proxy + API + asset serving
├── frontend/
│   ├── Cargo.toml      # Frontend dependencies
│   ├── index.html      # HTML entry
│   └── src/
│       └── main.rs     # Leptos component
└── target/
    ├── frontend/       # Trunk build output (embedded at compile time)
    ├── release/        # Backend binary
    └── debug/          # Debug builds
```

## Architecture

```
Single Binary (mole-tls)
├── Embedded Frontend (WASM + JS + HTML)
├── Admin API (/api/status)
└── Reverse Proxy (/* → httpbin.org)

Request Flow:
/            → Serve index.html (embedded)
/*.js, *.wasm → Serve assets (embedded)
/api/status  → Return JSON
/*           → Proxy to backend
```

## Learning Goals

This project is built incrementally to learn Rust:

1. [x] Basic HTTP reverse proxy
2. [x] Query params & Host header handling
3. [x] Connection pooling (LazyLock client)
4. [x] Admin API endpoint
5. [x] WASM frontend (Leptos)
6. [x] Single binary deployment (rust-embed)
7. [ ] Configurable backend
8. [ ] Multiple routing rules
9. [ ] TLS/SSL support
10. [ ] Authentication
11. [ ] Tunnel (NAT traversal)

## License

MIT
