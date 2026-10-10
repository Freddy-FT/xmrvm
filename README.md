# XMRVM

**Status: early prototype.** Most components are not implemented yet. The goal
right now is to learn and figure out good ways to create such a chain.

**WE ARE EARLY IN DEVELOPMENT.** This project is an experimental sidechain for Monero, and many components are still in the early stages of implementation. Use it at your own risk.

## Vision

A separate chain based on PoW that enables people to create smart contracts.
The chain would have rotating guards that every node can be, and that enable
on/off ramping.

## Repository layout

A Cargo workspace with the chain split into its building blocks:

| Crate                     | Purpose                                     |
| ------------------------- | ------------------------------------------- |
| `crates/core`             | Shared primitives: `Account`, `Transaction` |
| `crates/ledger`           | State and balance accounting                |
| `crates/vm`               | Smart contract execution                    |
| `crates/crypto`           | Hashing and chain cryptography             |
| `crates/node`             | Node binary                                 |
| `contracts/example_transfer` | Example contract to execute in the VM     |

## Building

```sh
cargo build
```

```sh
cargo test
```
