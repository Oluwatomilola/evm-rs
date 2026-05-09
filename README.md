# Mini EVM in Rust

A minimal Ethereum Virtual Machine (EVM) implementation written in Rust.

This project demonstrates:

- Bytecode parsing
- Stack-based execution
- Arithmetic opcodes
- Stack manipulation opcodes
- Program counter management
- Minimal EVM runtime loop

---

# Supported Opcodes

| Opcode | Hex  | Description |
|--------|------|-------------|
| STOP   | 0x00 | Halts execution |
| ADD    | 0x01 | Addition |
| MUL    | 0x02 | Multiplication |
| SUB    | 0x03 | Subtraction |
| DIV    | 0x04 | Division |
| POP    | 0x50 | Removes top stack item |
| PUSH1  | 0x60 | Pushes 1 byte to stack |
| DUP1   | 0x80 | Duplicates top stack item |
| SWAP1  | 0x90 | Swaps top two stack items |

---

# Project Structure

```text
src/
├── main.rs
└── vm/
    ├── mod.rs
    ├── evm.rs
    ├── stack.rs
    ├── opcodes.rs
    ├── executor.rs
    └── tests.rs
```

---

# Installation

Clone the repository:

```bash
git clone <repo-url>
cd evm-rs
```

Install dependencies:

```bash
cargo build
```

---

# Running the EVM

Example bytecode:

```text
0x6001600201
```

This translates to:

```text
PUSH1 0x01
PUSH1 0x02
ADD
```

Run:

```bash
cargo run -- 0x6001600201
```

Expected output:

```text
PC: 0, Opcode: 0x60
PC: 2, Opcode: 0x60
PC: 4, Opcode: 0x1

Execution Finished
Final Stack Top: 3
```

---

# Running Tests

```bash
cargo test
```

---

# EVM Architecture

The VM contains:

- Program Counter (`pc`)
- Stack
- Bytecode
- Halt flag

Execution flow:

1. Read opcode
2. Execute opcode
3. Advance program counter
4. Repeat until STOP or bytecode ends

---

# Future Improvements

- PUSH2 → PUSH32
- Memory support (`MLOAD`, `MSTORE`)
- Storage support
- Gas accounting
- JUMP/JUMPI support
- CALL opcode support
- Better error handling
- Full EVM compliance

---

# Learning Goals

This project helps understand:

- Ethereum internals
- Stack-based virtual machines
- Bytecode interpretation
- Rust systems programming
- Smart contract execution
