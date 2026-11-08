# MoleTLS

🐭 A minimal HTTP reverse proxy with embedded WASM frontend, written in pure Rust.

## Features

- Dynamic reverse proxy with configurable routing rules
- Domain-based virtual hosting (match by Host header)
- CRUD API for proxy configuration (`/api/services`)
- Header passthrough with Host override
- Query string preservation
- Connection pooling (reuse HTTP client)
- Embedded WASM frontend (single binary deployment)
- i18n support (Chinese / English)

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

# List proxy configs
curl http://localhost:7999/api/services
# []

# Add a proxy rule
curl -X POST http://localhost:7999/api/services \
  -H 'Content-Type: application/json' \
  -d '{"name":"test","domain":"test.local","target_url":"http://httpbin.org"}'

# Delete a proxy rule
curl -X DELETE http://localhost:7999/api/services/test
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
│   ├── Trunk.toml      # Trunk build config
│   ├── index.html      # HTML entry
│   ├── style.css       # Global styles
│   └── src/
│       ├── main.rs     # App entry, routing, sidebar
│       ├── services.rs # Proxy config management page
│       ├── settings.rs # Settings page
│       ├── status.rs   # Status page
│       ├── i18n.rs     # i18n loader
│       └── lang/
│           ├── mod.rs
│           ├── zh.rs   # Chinese translations
│           └── en.rs   # English translations
└── target/
    ├── frontend/       # Trunk build output (embedded at compile time)
    ├── release/        # Backend binary
    └── debug/          # Debug builds
```

## Architecture

```
Single Binary (mole-tls)
├── Embedded Frontend (WASM + JS + HTML)
├── Admin API
│   ├── GET    /api/status       → Server status
│   ├── GET    /api/services     → List proxy configs
│   ├── POST   /api/services     → Add proxy config
│   └── DELETE /api/services/:name → Delete proxy config
└── Dynamic Reverse Proxy (domain-based virtual hosting)

Request Flow:
localhost:7999/         → Serve index.html (embedded)
localhost:7999/*.js,wasm → Serve assets (embedded)
<domain>:7999/*         → Proxy to matched target_url
* (no match)            → 404
```

## Learning Goals

This project is built incrementally to learn Rust:

1. [x] Basic HTTP reverse proxy
2. [x] Query params & Host header handling
3. [x] Connection pooling (LazyLock client)
4. [x] Admin API endpoint
5. [x] WASM frontend (Leptos)
6. [x] Single binary deployment (rust-embed)
7. [x] Configurable backend (dynamic proxy rules)
8. [x] Multiple routing rules (domain-based virtual hosting)
9. [ ] TLS/SSL support
10. [ ] Authentication
11. [ ] Tunnel (NAT traversal)

## License

MIT
