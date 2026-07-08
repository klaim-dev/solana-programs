# solana-programs

Anchor programs: counter, vault, SPL tooling, CPI patterns, Token-2022. Written from scratch, tested per error branch.

**Stack:** Rust · Anchor · TypeScript · Solana CLI

**Status:** work in progress - updated daily.

## Goal

Build a compact but serious Anchor workspace that demonstrates Solana fundamentals through small, testable programs. Each program is intended to show one concrete pattern: accounts, PDAs, constraints, custom errors, SPL Token flows, CPI, staking, Token-2022 extensions, and security checks.

This repository is the main training ground for on-chain Solana development.

## Workspace Layout

```text
programs/   Anchor programs and on-chain Rust code
scripts/    Rust scripts for Solana CLI-style workflows
client/     TypeScript client examples
tests/      Integration and behavior tests
```

## Planned Programs

- `counter`: account initialization, mutation, constraints
- `vault`: PDAs, deposits, withdrawals, custom errors
- `todo`: account modeling and Anchor tests
- `token_factory`: SPL token creation and minting flows
- `tip_jar`: transfers, owners, and signer constraints
- `airdrop`: controlled distribution patterns
- `cpi_*`: cross-program invocation examples
- `staking`: reward accounting and time-based state
- `token_2022`: extension-focused Token-2022 experiments
- `fee_vault`: fee collection and treasury flows

## Quality Bar

- Tests cover success paths and expected failures
- Custom errors are named and asserted
- Seeds and bumps are documented near account constraints
- No keypairs, `.env`, or generated build artifacts in git

## Local Development

```bash
anchor build
anchor test
```

## Keywords

Solana, Anchor, Rust, SPL Token, Token-2022, CPI, PDA, smart contracts, Web3, blockchain security.
