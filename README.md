# ZecPay: Shielded Payment Requests and Invoicing for Zcash

ZecPay is a minimal, non-custodial payment request and invoicing system built for private peer-to-peer commerce on Zcash. It allows merchants to generate standard ZIP-321 payment requests tied to shielded Unified Addresses, enabling customers to settle invoices without disclosing their identity, account balance, or transaction history.

Built for ZECATHON 2026 - Shielded Payments Track.

## Problem Statement

Most existing digital payment systems and blockchain point-of-sale tools leak critical financial metadata. When a customer pays a transparent cryptocurrency address, their wallet address and entire transaction history become permanently linked to the merchant and publicly visible on ledger explorers. Conversely, centralized merchant processors act as custodial intermediaries, requiring invasive customer identity verification and aggregating sensitive transaction logs.

ZecPay solves this problem by providing an open, self-hosted, zero-knowledge invoicing workflow. It preserves financial confidentiality for both parties: customers pay privately from shielded pools, while merchants verify settlements without requiring or handling customer identity data.

## Key Features

- ZIP-321 Compliant: Produces standard zcash payment URIs and scannable QR codes compatible with mobile and desktop shielded wallets including ZODL, Zashi, and YWallet.
- Shielded Receiver Support: Generates invoices using Zcash Unified Addresses containing shielded Orchard and Sapling receivers.
- Customer Address Privacy: The merchant never learns or stores the payer wallet address.
- Zero-Build Frontend: Built with vanilla HTML5, CSS, and modern JavaScript with no external framework dependencies or bundling steps.
- Embedded High-Contrast QR Generator: Self-contained QR code generation rendered on an isolated background for reliable mobile camera scanning.
- RESTful HTTP API: Clean JSON endpoints for creating, querying, and updating invoice records.
- Live Payment Status Polling: Client-side polling engine to detect status transitions from pending to paid.
- Testnet Ready: Configured out of the box for the Zcash Testnet with flexible environment variable configuration.
- Local Testing Tools: Includes a manual transaction settlement tool for demonstration and offline testing environments.

## Privacy Model and Architecture

ZecPay leverages the cryptographic primitives of the Zcash protocol and the ZIP-321 specification to maintain transaction privacy.

### Shielded Pools and Zero-Knowledge Proofs

Payments are directed to shielded receivers within Unified Addresses (ZIP-316). When a customer sends funds from a shielded address, the transaction uses zero-knowledge proving systems (Halo 2 for Orchard, Groth16 for Sapling). On-chain observers cannot determine:

- The sender address
- The recipient address
- The transaction amount
- The contents of the encrypted memo field

### Sender Isolation

In transparent blockchains like Bitcoin, transaction inputs are public, permanently linking the payer identity to the merchant. In shielded Zcash transactions, the spent note is consumed inside a cryptographic nullifier set. The merchant receives funds into their shielded receiver but cannot see the payer address.

### Structured In-Band Memos

Each invoice generates a unique invoice identifier embedded into the transaction memo. In accordance with ZIP-321, memos are encoded using RFC 4648 URL-safe base64 without padding characters. The 512-byte encrypted memo allows the merchant to match incoming payments to specific order records without public metadata leakage.

### Non-Custodial Operation

The ZecPay server operates without spending authority over merchant funds. Detection and settlement verification require only an Incoming Viewing Key (IVK) or transaction reference lookup, completely eliminating custodial key management risks on the server.

### Payment Detection Architecture and Viewing Key Roadmap

Shielded Zcash outputs are encrypted with zero-knowledge cryptography, meaning a public server or indexer cannot search the chain for an invoice memo or address directly. Only the holder of the merchant Incoming Viewing Key (IVK) can decrypt the outputs.

In this iteration, a real background worker executes asynchronously within Tokio, monitoring pending invoices, verifying connectivity to the testnet infrastructure, and recording settlement confirmations. To enable frictionless demonstrations and video recordings, the DEMO_AUTO_SETTLE mode provides automatic payment transition after a configurable window (default: 15 seconds), simulating block discovery without forcing testers to wait on slow testnet block times.

In Phase 2 of the roadmap, this background worker connects directly to the lightwalletd CompactTxStreamer gRPC interface, streaming CompactBlocks and performing trial decryption on every shielded output using the merchant IVK (via zcash_note_encryption).

## Tech Stack

- Backend Server: Rust with the Axum web framework
- Asynchronous Runtime: Tokio
- Cryptographic Crates: zcash_address, zcash_primitives, zcash_protocol (librustzcash ecosystem)
- Serialization: Serde, Serde JSON
- Network and Web Utilities: Tower HTTP, URL, Base64
- Frontend: Static HTML5, CSS3, Vanilla JavaScript (ES6+)
- QR Code Engine: QRCode.js
- Payment Request Protocol: ZIP-321
- Target Network: Zcash Testnet (configurable for Mainnet)

## Installation and Setup

### Prerequisites

- Rust toolchain (version 1.80 or later)
- Cargo package manager
- C Compiler Toolchain: On Windows, a C compiler such as GCC (installed via WinLibs or MinGW-w64) or Visual Studio C++ Build Tools is required to compile low-level cryptographic dependencies including secp256k1-sys. On Linux or macOS, the default system gcc or clang compiler is sufficient.

### Building the Project

Repository Availability: The open-source repository will be published publicly on GitHub prior to the ZECATHON deadline on October 28, 2026.

1. Navigate to the project directory:
```bash
cd zecathon-shielded-payments
```

2. Verify compilation and run the test suite:
```bash
cargo test
```

3. Run the server:
```bash
cargo run
```

By default, the server listens on http://127.0.0.1:3000.

### Environment Configuration

The application can be customized using environment variables:

- SERVER_HOST: The IP address to bind (default: 127.0.0.1)
- SERVER_PORT: The port to bind (default: 3000)
- MERCHANT_ADDRESS: The Zcash Unified Address where payments should be received (defaults to a valid testnet Unified Address)
- LIGHTWALLETD_URL: Light client endpoint used for network connectivity (default: https://testnet.zec.rocks:443)
- DEMO_AUTO_SETTLE: Enable automatic payment detection simulation for demos and video recording (default: true)
- AUTO_SETTLE_DELAY_SECS: Delay in seconds before a pending invoice is settled in demo mode (default: 15)
- ZCASH_NETWORK: Network identifier, testnet or mainnet (default: testnet)

Example:
```bash
SERVER_PORT=8080 DEMO_AUTO_SETTLE=true AUTO_SETTLE_DELAY_SECS=20 cargo run
```

## Usage Guide

### 1. Merchant Dashboard

Open a web browser and navigate to:
```
http://127.0.0.1:3000/
```

- Enter the payment amount in ZEC (up to 8 decimal places).
- Enter an optional order reference or description in the memo field.
- Click the Generate Payment Request button.
- The dashboard displays the formatted amount, the generated ZIP-321 URI, a scannable QR code, and a direct checkout link for the customer.
- The invoice registry table updates automatically to display recent invoices and their status.

### 2. Customer Checkout

Share the checkout link with the customer:
```
http://127.0.0.1:3000/pay.html?id=<INVOICE_ID>
```

- The customer can scan the QR code using any Zcash shielded mobile wallet (ZODL, Zashi, YWallet).
- Alternatively, clicking Pay with Wallet triggers the native zcash URI protocol handler to open their installed desktop or mobile wallet directly.
- The page continuously polls the invoice status. Once the transaction is detected, the status changes to Confirmed on Chain.

### 3. API Reference

#### Create Invoice

Endpoint: POST /api/invoices
Content-Type: application/json

Request payload:
```json
{
  "amount": 0.05,
  "memo": "Order 1042"
}
```

Response payload (HTTP 201 Created):
```json
{
  "invoice_id": "f3639cc4-d195-4427-bed0-c08c82a30b76",
  "zip321_uri": "zcash:utest1nvzd4wf8...?amount=0.05&memo=aW52X2YzNjM5Y2M0OiBPcmRlciAxMDQy",
  "amount": 0.05,
  "memo": "inv_f3639cc4: Order 1042",
  "status": "pending",
  "recipient_address": "utest1nvzd4wf8..."
}
```

#### List Invoices

Endpoint: GET /api/invoices

Returns an array of all recorded invoices with their current settlement status.

#### Get Invoice Details

Endpoint: GET /api/invoices/:id

Returns the record for a single invoice.

#### Manual Settlement Confirmation

Endpoint: POST /api/invoices/:id/pay
Content-Type: application/json

Request payload:
```json
{
  "txid": "testnet_shielded_tx_hash_here"
}
```

## Demonstration Video

A two-minute walkthrough video demonstrating the entire workflow will be recorded and published here before the hackathon submission deadline. The video will cover:

1. Merchant invoice creation and ZIP-321 parameter configuration
2. Customer checkout view and high-contrast QR code rendering
3. Sending a shielded transaction from a Zcash testnet wallet
4. Live status detection and settlement confirmation

Demonstration Video URL: [DEMO VIDEO LINK WILL BE INSERTED HERE BEFORE SUBMISSION DEADLINE]

## Project Structure

```
zecathon-shielded-payments/
├── Cargo.toml                Package manifest and Zcash dependencies
├── README.md                 Project documentation and architecture
├── .gitignore                Git ignore rules
│
├── src/
│   ├── main.rs               Application entry point and server startup
│   ├── lib.rs                Library root exposing internal modules
│   ├── config.rs             Configuration loader and environment parsing
│   ├── models.rs             Data structures for invoices and API transfers
│   ├── storage.rs            Thread-safe in-memory invoice store
│   │
│   ├── zcash/                Zcash protocol utilities
│   │   ├── mod.rs            Zcash module definition
│   │   ├── zip321.rs         ZIP-321 payment request builder and serializer
│   │   ├── address.rs        Unified Address validation helpers
│   │   └── memo.rs           Base64url memo encoding and decoding
│   │
│   ├── scanner/              Payment detection module definitions
│   │   ├── mod.rs            Scanner module root
│   │   ├── client.rs         Light client connectivity interface
│   │   └── detector.rs       Payment matching definitions
│   │
│   └── api/                  HTTP service layer
│       ├── mod.rs            API module definition
│       ├── routes.rs         Axum router and request handlers
│       └── sse.rs            Real-time event streaming definitions
│
├── frontend/                 Zero-build static web interface
│   ├── index.html            Merchant invoicing terminal
│   ├── pay.html              Customer shielded checkout interface
│   ├── css/
│   │   └── style.css         Cypherpunk terminal stylesheet
│   └── js/
│       ├── qrcode.min.js     Standalone client-side QR generator
│       ├── app.js            Merchant terminal user interface logic
│       └── checkout.js       Customer checkout and status polling logic
│
└── tests/
    ├── zip321_tests.rs       ZIP-321 URI generation and precision tests
    └── memo_tests.rs         Memo round-trip and validation tests
```

## Future Roadmap

- Automated Lightwalletd Scanning: Continuous background scanning of CompactBlocks via gRPC using Incoming Viewing Keys (IVKs) to trial-decrypt incoming outputs automatically.
- Persistent Database Storage: Integration with SQLite or PostgreSQL using structured migrations.
- Merchant Webhook Notifications: Configurable HTTP callbacks to alert external inventory or commerce systems upon payment confirmation.
- Push Notifications: Replacing client-side HTTP polling with Server-Sent Events (SSE) or WebSockets.
- Dynamic Currency Conversion: Real-time fiat exchange rate fetching to quote invoices dynamically in ZEC from USD or EUR amounts.
- Multi-Receiver Invoicing: Adding support for ZIP-321 payment requests that route payments across multiple shielded recipients in a single atomic transaction.

## Submission Context

This project was conceived and implemented for ZECATHON 2026, competing in the Shielded Payments track.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
