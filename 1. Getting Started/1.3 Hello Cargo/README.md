# 🦀 Hello, Cargo! – Quick Revision

## 1️⃣ What is Cargo?

Cargo is Rust’s:

- 📦 Package manager
- 🛠 Build system
- 🔄 Dependency manager

It:

- Builds your code
- Downloads dependencies (crates)
- Manages versions
- Organizes project structure

Installed automatically with Rust (via rustup).

Check installation:

```bash
cargo --version
```

---

## 2️⃣ Create a New Project

```bash
cargo new hello_cargo
cd hello_cargo
```

Cargo creates:

```
hello_cargo/
 ├── Cargo.toml
 └── src/
     └── main.rs
```

It also initializes a Git repository.

---

## 3️⃣ Cargo.toml

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

- `[package]` → project metadata
- `[dependencies]` → add external crates here
- Uses TOML format

---

## 4️⃣ Build & Run

### 🔨 Build

```bash
cargo build
```

- Output → `target/debug/`
- Creates `Cargo.lock`

### ▶ Run

```bash
cargo run
```

- Builds (if needed) + runs in one step

### ⚡ Fast Compile Check

```bash
cargo check
```

- Checks code
- No executable generated
- Faster than `cargo build`

---

## 5️⃣ Release Build (Optimized)

```bash
cargo build --release
```

- Output → `target/release/`
- Optimized for performance
- Slower compile time

Use this for production builds or benchmarking.

---

## 6️⃣ Key Commands Summary

| Command                 | Purpose             |
| ----------------------- | ------------------- |
| `cargo new`             | Create project      |
| `cargo build`           | Compile project     |
| `cargo run`             | Compile + run       |
| `cargo check`           | Quick compile check |
| `cargo build --release` | Optimized build     |

---

## 🧠 Why Cargo Matters

- Standard tool for all Rust projects
- Cross-platform commands
- Handles multi-file projects easily
- Makes dependency management simple
- Required for real-world Rust development

---

From now on, use **Cargo instead of rustc directly** 🚀
