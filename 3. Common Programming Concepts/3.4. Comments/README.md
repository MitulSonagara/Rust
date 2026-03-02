# 🦀 Rust Comments – Module Notes

## 📌 Overview

Comments are used to explain code.

- Ignored by the compiler
- Meant for humans reading the code
- Improve clarity and maintainability

---

# 1️⃣ Single-Line Comments

Rust uses `//` for standard comments.

```rust
// hello, world
```

Everything after `//` on that line is ignored.

---

# 2️⃣ Multi-Line Comments (Using Multiple `//`)

Rust does not require a special multi-line syntax.
You simply use `//` on each line:

```rust
// This is a longer explanation
// that spans multiple lines
// for clarity.
```

This is the idiomatic Rust style.

---

# 3️⃣ End-of-Line Comments

Comments can appear after code:

```rust
let lucky_number = 7; // I'm feeling lucky today
```

Used for short explanations.

---

# 4️⃣ Above-Code Comments (Preferred Style)

More commonly, comments are written above the code they describe:

```rust
// I'm feeling lucky today
let lucky_number = 7;
```

✔ Cleaner  
✔ Easier to read  
✔ Preferred in Rust projects

---

# 5️⃣ Documentation Comments (Brief Mention)

Rust also supports **documentation comments**, which:

- Are written using `///`
- Generate documentation via `cargo doc`

These are covered later when publishing crates.

---

# 🧠 Key Takeaways

- `//` starts a comment
- Comments improve readability
- Compiler ignores comments
- Prefer placing comments above code
- Documentation comments (`///`) are different from regular comments

---

Clear code > clever code.  
Comments should explain _why_, not just _what_.
