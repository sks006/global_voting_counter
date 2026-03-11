
# Global Counter Pro (RWA-Ready Solana Engine)

A high-performance, rules-based Solana program designed for secure state management. This project implements a globally verifiable counter with administrative override capabilities, strict PDA (Program Derived Address) derivation, and a "Circuit Breaker" pause mechanism.

## 🏎️ Engineering Architecture

This program is built as a **Finite State Machine**. Every instruction must clear a series of "Security Gaskets" before the logic can "combust" (mutate state).

### The Mechanical Data Flow:

1. **Entry (`lib.rs`)**: The Solana Runtime triggers the `entrypoint!`.
2. **Gearbox (`instruction.rs`)**: Raw `&[u8]` bytes are unpacked into a `CounterInstruction` enum using Little-Endian (LE) conversion.
3. **The Engine (`processor.rs`)**:
* **Authorization**: Checks if required accounts are `Signers`.
* **Validation**: Checks if Account Owners match the `Program ID`.
* **State Check**: Verifies the `AdminConfig` is not in a `Paused` state.


4. **Combustion**: Updates the `Counter` or `Admin` state using checked arithmetic to prevent overflows.
5. **Exhaust (`events.rs`)**: Emits telemetry via `sol_log_data`.

## 📂 Project Structure

```global_voting_counter

├── Cargo.lock
├── Cargo.toml
├── readme.md
├── src
│   ├── error.rs
│   ├── instruction.rs
│   ├── lib.rs
│   ├── processor.rs
│   └── state.rs
├── target
│   ├── CACHEDIR.TAG
│   ├── debug
│   ├── flycheck0
│   └── tmp
└── tests
    └── functional.rs

```

## 🛠️ Technical Specifications

| Feature | Implementation |
| --- | --- |
| **Language** | Raw Rust (No Anchor) for maximum control |
| **Serialization** | Borsh (Binary Object Representation Serializer for Hashing) |
| **State Layout** | Strict byte-alignment (Counter: 41 bytes, Admin: 34 bytes) |
| **Math** | `checked_add` / `checked_sub` to prevent integer wrap-around |
| **Access Control** | Admin-locked PDAs with deterministic seeds |

## 🚀 Getting Started

### 1. Prerequisites

Ensure you have the Solana Tool Suite and Rust installed.

```bash
solana --version
cargo --version

```

### 2. Build the Engine

Compile the program into SBF (Solana Bytecode Format):

```bash
cd program
cargo build-sbf

```

### 3. Run the Test Bench

Execute the functional and integration tests to verify the security gaskets:

```bash
# Runs the Rust functional tests in /program/tests
cargo test-sbf

```

## 🔒 Security Invariants

* **Re-initialization Guard**: Every account starts with a `TAG_UNINITIALIZED` (0). The program will fail if you attempt to initialize an account that does not have a zero-tag.
* **Signer Verification**: Administrative actions (Pause/Resume/Transfer) require a signature from the `Admin` key stored in the `AdminConfig` account.
* **Account Ownership**: The program strictly verifies that every account passed is owned by the `Program ID` to prevent "Fake Player" attacks.

## 📊 Process Diagram

For a visual representation of the instruction routing and PDA derivation logic, refer to the [Process Architecture Diagram](https://drive.google.com/file/d/1krx4xsYAR4ZcS9SQTv_fptIVqs1fb4Ao/view?usp=sharing).

---

**Author:** Principal Engineer

**License:** MIT

**Status:** Calibrated & Ready for Devnet