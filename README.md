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

```
solana-programs/
├── programs/
│   ├── counter/
│   ├── vault/
│   ├── todo/
│   ├── tip_jar/
│   ├── token_factory/
│   └── airdrop/
├── tests/
├── Anchor.toml
└── Cargo.toml
```

## Build & Test

```bash
anchor build
anchor test
anchor deploy --provider.cluster devnet
```

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
