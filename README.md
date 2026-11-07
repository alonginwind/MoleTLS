# MoleTLS

🐭 A minimal HTTP reverse proxy written in pure Rust.

## Features

- Reverse proxy with request forwarding
- Header passthrough
- Catch-all routing

## Quick Start

```bash
# Build
cargo build

# Run
cargo run

# Test
curl http://localhost:7999/get
```

## Tech Stack

| Crate | Purpose |
|-------|---------|
| tokio | Async runtime |
| axum | Web framework |
| hyper | HTTP protocol |
| hyper-util | HTTP client |
| tracing | Logging |

## Project Structure

```
MoleTLS/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs
```

## Learning Goals

This project is built incrementally to learn Rust:

1. [x] Basic HTTP reverse proxy
2. [ ] Configurable backend
3. [ ] Multiple routing rules
4. [ ] Admin API
5. [ ] TLS/SSL support
6. [ ] Authentication
7. [ ] Tunnel (NAT traversal)

## License

MIT
