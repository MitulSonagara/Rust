# 🦀 Guessing Game – Quick Revision

## 🎯 Goal

Build a game that:

- Generates a random number (1–100)
- Takes user input
- Tells if guess is:
  - Too small
  - Too big
  - Correct
- Loops until user wins

---

## 1️⃣ Create Project

```bash
cargo new guessing_game
cd guessing_game
```

Add dependency in `Cargo.toml`:

```toml
[dependencies]
rand = "0.8.5"
```

Build once to download crates:

```bash
cargo build
```

---

## 2️⃣ Core Concepts Used

### 🔹 `use`

Bring modules/traits into scope:

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;
```

---

### 🔹 Random Number

```rust
let secret_number = rand::thread_rng().gen_range(1..=100);
```

- `thread_rng()` → thread-local random generator
- `gen_range(1..=100)` → inclusive range
- Requires `Rng` trait in scope

---

### 🔹 User Input

```rust
let mut guess = String::new();

io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

- `read_line` returns `Result`
- `expect()` crashes on error

---

### 🔹 String → Number Conversion

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

- `trim()` removes newline
- `parse()` converts string to number
- Returns `Result`
- Shadowing used to reuse `guess`

---

### 🔹 Handling Invalid Input (Better Version)

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

- `Ok(num)` → valid number
- `Err(_)` → ignore and retry
- `_` = catch-all pattern
- `continue` → next loop iteration

---

### 🔹 Comparing Values

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;
    }
}
```

- `cmp()` returns `Ordering` enum
- `match` handles all possible cases
- `break` exits loop

---

## 3️⃣ Looping

```rust
loop {
    // game logic
}
```

- Infinite loop
- Stops only with `break`

---

## 4️⃣ Final Game Flow

1. Generate random number
2. Loop:
   - Ask for input
   - Parse input
   - Compare
   - Respond
   - Break if correct

---

## 🧠 Concepts Learned

- `let` and `mut`
- Shadowing
- `match`
- Enums (`Result`, `Ordering`)
- Error handling
- External crates
- Traits (`Rng`)
- Loops (`loop`, `break`, `continue`)
- Type conversion (`parse()`)

---

## 🚀 Why This Project Matters

This small game introduces:

- Rust’s strong type system
- Safe error handling
- Pattern matching
- Real-world Cargo usage
- External dependencies

This is your first real Rust program.

You’re officially past "Hello World" level 🦀
