# Off-Chain Governance Simulator (Rust CLI)

A simple command-line tool written in Rust to simulate a basic governance system with users, proposals, and weighted voting.  
The project was created as a learning exercise to better understand Rust, modular code organization, and state management.

## Features

- Add users with roles (`User`, `Validator`, `Admin`)
- Each role has different vote weight
- Create proposals with titles
- Vote “yes” or “no” on open proposals
- Close proposals to finalize results
- View results at any time
- Export/import full voting state via JSON
- Unit tests included for core logic

## How to run

```bash
git clone https://github.com/yourusername/offchain-governance-sim.git
cd offchain-governance-sim
cargo run
```

## How to test

```bash
cargo test
```

## Project structure

```
src/
├── main.rs         # CLI interface
├── governance.rs   # Voting state and coordination logic
├── proposal.rs     # Proposal data and status handling
└── user.rs         # User roles and vote weights
```

## Notes

```
This project was created independently as part of my learning path with Rust.
It’s intentionally simple and focuses on correctness, structure, and clean logic.
```
