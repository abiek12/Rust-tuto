# Rust Learning Notes

Personal notes while learning Rust from [The Rust Book](https://doc.rust-lang.org/book/).

---

## Installation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Check version:
```bash
rustc --version
cargo --version
```

---

## How Rust Runs

Rust is a **compiled** language. You write `.rs` files, compile them to a binary, then run the binary.

```
source code (.rs)  →  compiler (rustc)  →  binary  →  run
```

---

## Two Ways to Run

### 1. Using `rustc` directly (simple single files)

```bash
rustc main.rs    # compiles to a binary
./main           # run the binary
```

### 2. Using `cargo` (recommended)

```bash
cargo new my_project     # create a new project
cd my_project
cargo run                # compile + run in one step
cargo build              # just compile
cargo build --release    # optimized build for production
cargo check              # check for errors without producing a binary (fast)
```

---

## Cargo Project Structure

```
my_project/
├── Cargo.toml      # project config + dependencies (like package.json)
├── Cargo.lock      # locked dependency versions (auto-generated)
└── src/
    └── main.rs     # entry point
```

---

## First Program

```rust
fn main() {
    println!("Hello, world!");
}
```

- `fn main()` — entry point of every Rust program
- `println!` — a macro (the `!` means it's a macro, not a function)
- Statements end with `;`

---

## Key Things Noted So Far

- `rustc` is the compiler, `cargo` is the build tool + package manager
- `cargo run` is the most used command day to day
- `cargo check` is fast — good for catching errors while writing
- Dependencies are called **crates**, added in `Cargo.toml`
- Rust files use `.rs` extension

---

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust Playground](https://play.rust-lang.org/) — try Rust in the browser