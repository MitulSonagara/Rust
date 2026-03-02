# 🦀 Hello, World! – Rust Quick Guide

## 1️⃣ Project Setup

Create a project directory:

### Linux / macOS / PowerShell

```bash
mkdir ~/projects
cd ~/projects
mkdir hello_world
cd hello_world
```

### Windows CMD

```cmd
mkdir "%USERPROFILE%\projects"
cd /d "%USERPROFILE%\projects"
mkdir hello_world
cd hello_world
```

---

## 2️⃣ Create Rust File

Create a file named:

```
main.rs
```

Rust files:

- Must end with `.rs`
- Use snake_case for file names

---

## 3️⃣ Hello World Code

```rust
fn main() {
    println!("Hello, world!");
}
```

---

## 4️⃣ Anatomy of a Rust Program

### `fn main()`

- Entry point of every Rust executable
- Runs first
- No parameters here
- Function body wrapped in `{}`

### `println!`

- `println!` is a **macro** (note the `!`)
- Prints text to the terminal
- Most Rust statements end with `;`

---

## 5️⃣ Compile & Run

### Compile:

```bash
rustc main.rs
```

This creates an executable:

- `main` (Linux/macOS)
- `main.exe` (Windows)

### Run:

Linux/macOS:

```bash
./main
```

Windows:

```bash
.\main
```

Output:

```
Hello, world!
```

---

## 6️⃣ Important Concepts

- Rust is **ahead-of-time compiled**
- You must compile before running
- Output is a standalone executable
- No Rust installation required to run the compiled binary

---

## 🧠 What You Just Learned

- How to create a Rust source file
- Structure of a basic Rust program
- What `main` does
- What a macro is (`!`)
- How compilation works

---

Next step: Use **Cargo** for real projects 🚀
