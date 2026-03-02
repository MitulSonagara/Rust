# 🦀 Rust Installation – Quick Guide

## 1️⃣ Install Rust (via rustup)

Rust is installed using **rustup**, the official toolchain manager.

### 🔹 Linux / macOS

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

After installation, restart your terminal.

If linker errors appear:

- macOS → `xcode-select --install`
- Ubuntu → install `build-essential`
- Other Linux → install GCC or Clang

---

### 🔹 Windows

1. Go to: https://www.rust-lang.org/tools/install
2. Download and run the installer
3. Install Visual Studio components if prompted (required for linker)

---

## 2️⃣ Verify Installation

```bash
rustc --version
```

Expected output:

```
rustc x.y.z (commit-hash date)
```

If command not found → check your `PATH` variable.

---

## 3️⃣ Update Rust

```bash
rustup update
```

---

## 4️⃣ Uninstall Rust

```bash
rustup self uninstall
```

---

## 5️⃣ Open Offline Documentation

```bash
rustup doc
```

---

## 6️⃣ (Optional) Cache Dependencies for Offline Use

```bash
cargo new get-dependencies
cd get-dependencies
cargo add rand@0.8.5 trpl@0.2.0
```

After this, you can use:

```bash
cargo build --offline
```

---

## 🧠 Key Notes

- rustup installs the latest **stable** Rust version
- Cargo (Rust’s package manager) is installed automatically
- Works in cmd, PowerShell, macOS, and Linux terminals
- Most editors/IDEs support Rust (VS Code, IntelliJ, etc.)

---

You're ready to build 🦀
