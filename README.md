# Solana Programs

A collection of Solana programs built with Anchor, demonstrating core on-chain development patterns.

## Programs

| Program | Description | Key Concepts | Status |
|---------|-------------|-------------|--------|
| `counter` | Increment/decrement counter | Anchor basics, PDA, tests | 🔜 |
| `vault` | Deposit/withdraw SOL via PDA | PDA signer, lamport transfer, events | 🔜 |
| `todo` | On-chain task manager | Multi-account relationships, `has_one` | 🔜 |
| `tip_jar` | Accept and withdraw tips | PDA vault, authority checks, events | 🔜 |
| `token_factory` | Create and manage SPL tokens | SPL Token, ATA, mint/transfer/burn/freeze | 🔜 |
| `airdrop` | Claim-based token distribution | SPL Token + PDA + claim tracking | 🔜 |

## Project Structure

Each program is now isolated in its own workspace crate:

```text
solana-programs/
├── programs/
│   ├── counter/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── vault/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── todo/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── tip_jar/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── token_factory/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── airdrop/
│       ├── Cargo.toml
│       └── src/lib.rs
├── tests/
└── Anchor.toml
```

## Build & Test

Run commands per program directory, for example:

```bash
cd programs/counter
cargo check
```

For Anchor workflows, do the same from the specific program directory when the Anchor configuration is added there.

## Devnet Deployments

| Program | Program ID |
|---------|-----------|
| counter | `TBD` |
| vault | `TBD` |
| todo | `TBD` |
| tip_jar | `TBD` |
| token_factory | `TBD` |
| airdrop | `TBD` |

## Tech Stack

- **Framework:** Anchor
- **Language:** Rust
- **Network:** Solana Devnet
- **Testing:** Anchor TS tests + Bankrun
