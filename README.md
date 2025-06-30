# Solana Axum Server

A simple REST API server built with Rust and Axum that provides basic Solana blockchain operations on devnet.

## Features

- Get SOL balance for any Solana address
- Request SOL airdrops on devnet
- Get account details
- Built with Axum for high performance
- Connects to Solana devnet

## Prerequisites

- Rust (latest stable version)
- Cargo

## Installation

1. Clone this repository
2. Install dependencies:
   ```bash
   cargo build
   ```

## Running the Server

```bash
cargo run
```

The server will start on `http://0.0.0.0:8080`

## Live Demo

The server is deployed and running at: **https://rust-axum-solana-backend.onrender.com/**

You can test the endpoints directly using this live URL instead of running locally.

## API Endpoints

### GET `/`
Returns a simple hello world message.

**Example:**
```bash
# Local
curl http://localhost:8080/

# Live demo
curl https://rust-axum-solana-backend.onrender.com/
```

### GET `/balance/:pubkey`
Get the SOL balance for a given public key.

**Parameters:**
- `pubkey` - Solana public key (base58 encoded)

**Example:**
```bash
# Local
curl http://localhost:8080/balance/YOUR_PUBKEY_HERE

# Live demo
curl https://rust-axum-solana-backend.onrender.com/balance/YOUR_PUBKEY_HERE
```

### GET `/airdrop/:pubkey`
Request a 1 SOL airdrop to the specified public key (devnet only).

**Parameters:**
- `pubkey` - Solana public key (base58 encoded)

**Example:**
```bash
# Local
curl http://localhost:8080/airdrop/YOUR_PUBKEY_HERE

# Live demo
curl https://rust-axum-solana-backend.onrender.com/airdrop/YOUR_PUBKEY_HERE
```

### GET `/details/:pubkey`
Get detailed account information including balance and public key.

**Parameters:**
- `pubkey` - Solana public key (base58 encoded)

**Example:**
```bash
# Local
curl http://localhost:8080/details/YOUR_PUBKEY_HERE

# Live demo
curl https://rust-axum-solana-backend.onrender.com/details/YOUR_PUBKEY_HERE
```

## Dependencies

- `solana-client` - Solana RPC client
- `solana-sdk` - Solana SDK for types and utilities
- `axum` - Web framework
- `serde_json` - JSON serialization
- `tokio` - Async runtime

## Network

This server connects to Solana devnet (`https://api.devnet.solana.com`). All operations are performed on the devnet, so no real SOL is involved.

## License

MIT 