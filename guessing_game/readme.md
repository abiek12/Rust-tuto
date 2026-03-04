# Chapter 2 — Guessing Game

A hands-on project covering core Rust concepts.

---

## Full Code

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
```

---

## Variables & Mutability

```rust
let foo = 5;        // immutable by default
let mut foo = 5;    // mutable — needs `mut` keyword
```

Rust variables are **immutable by default**. You have to explicitly opt in to mutation with `mut`.

---

## String Input

```rust
let mut guess = String::new();
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

- `String::new()` creates an empty growable string
- `read_line` appends user input into the string
- `&mut guess` passes a **mutable reference** — the function can modify it
- `.expect()` crashes with a message if something goes wrong (basic error handling)

---

## Using an External Crate

Add `rand` to `Cargo.toml`:

```toml
[dependencies]
rand = "0.8.5"
```

Run `cargo build` — Cargo downloads it automatically from [crates.io](https://crates.io).

```rust
use rand::Rng;
let secret_number = rand::thread_rng().gen_range(1..=100);
//                                               1..=100 → inclusive range
```

---

## Type Conversion & Shadowing

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_)  => continue,
};
```

- `guess` is redeclared as `u32` — this is **shadowing** (reusing same name with a new type)
- `.trim()` removes the newline `\n` left by Enter key
- `.parse()` converts string → number
- `Ok` / `Err` are the two arms of `Result` — Rust's way of handling errors without exceptions

---

## Comparing with `match`

```rust
match guess.cmp(&secret_number) {
    Ordering::Less    => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal   => println!("You win!"),
}
```

- `match` is like a switch but **exhaustive** — every possible case must be handled
- `cmp` returns an `Ordering` enum: `Less`, `Greater`, or `Equal`

---

## Loop and Break

```rust
loop {
    // runs forever until...
    break; // explicitly told to stop
}
```

`continue` skips the rest of the current iteration and goes back to the top of the loop.

---

## Key Takeaways

| Concept   | What it means                                  |
| --------- | ---------------------------------------------- |
| `let`     | immutable variable                             |
| `let mut` | mutable variable                               |
| `&`       | reference                                      |
| `&mut`    | mutable reference                              |
| Shadowing | redeclare same variable name with new type     |
| `match`   | exhaustive pattern matching                    |
| `Result`  | `Ok(value)` or `Err(e)` — no try/catch in Rust |
| Crates    | external packages managed via `Cargo.toml`     |
| `loop`    | infinite loop, exit with `break`               |