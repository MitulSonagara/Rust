# 🦀 Rust Data Types – Module Notes

## 📌 Overview

Every value in Rust has a **data type**.

Rust is **statically typed**, meaning:

- Types must be known at compile time
- Compiler usually infers types
- Sometimes explicit type annotations are required

Example:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

---

# 1️⃣ Scalar Types

Scalar types represent a **single value**.

## 🔢 Integers

Signed → `i8, i16, i32, i64, i128, isize`  
Unsigned → `u8, u16, u32, u64, u128, usize`

Default integer type → `i32`

### Integer Literals

| Format  | Example       |
| ------- | ------------- |
| Decimal | `98_222`      |
| Hex     | `0xff`        |
| Octal   | `0o77`        |
| Binary  | `0b1111_0000` |
| Byte    | `b'A'`        |

### ⚠ Integer Overflow

- Debug mode → panic
- Release mode → wrapping behavior
- Safe methods:
  - `checked_*`
  - `wrapping_*`
  - `overflowing_*`
  - `saturating_*`

---

## 🌊 Floating Point

- `f32`
- `f64` (default)

IEEE-754 standard.

---

## ➕ Numeric Operations

- `+` addition
- `-` subtraction
- `*` multiplication
- `/` division
- `%` remainder

Integer division truncates toward zero.

---

## 🔘 Boolean

- Type: `bool`
- Values: `true`, `false`
- Used in conditionals

---

## 🔤 Character

- Type: `char`
- 4 bytes
- Unicode scalar value
- Uses single quotes: `'a'`

Can store:

- Emojis
- Accented letters
- CJK characters

---

# 2️⃣ Compound Types

Group multiple values together.

---

## 📦 Tuple

- Fixed length
- Can store multiple types

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

Access:

- Destructuring
- Dot indexing (`tup.0`)

Special tuple: `()` → unit type

---

## 📚 Array

- Fixed length
- Same type elements
- Stored on stack

```rust
let a = [1, 2, 3, 4, 5];
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5]; // [3,3,3,3,3]
```

Access with indexing:

```rust
a[0]
```

### ⚠ Out-of-Bounds Access

- Causes runtime panic
- Prevents invalid memory access
- Rust guarantees memory safety

---

# 🧠 Key Takeaways

- Rust is statically typed
- Defaults: `i32`, `f64`
- Scalars: integers, floats, bool, char
- Compounds: tuple, array
- Arrays are fixed-size
- Rust prevents invalid memory access
- Type safety + memory safety by design

---

This module builds the foundation for ownership and memory concepts next.
