# 🦀 Rust Module: Ownership

This module explores one of Rust’s most important and unique features: **Ownership**.

Ownership is Rust’s system for managing memory safely without a garbage collector.

---

## 📚 What You’ll Learn

- What ownership is and why it exists
- Stack vs Heap memory
- The 3 Ownership Rules
- Move semantics
- Clone vs Copy
- Ownership in functions
- Returning ownership
- Why Rust prevents double-free errors

---

## 🧠 The Three Ownership Rules

1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.

---

## 🗂 Stack vs Heap

| Stack                     | Heap                       |
| ------------------------- | -------------------------- |
| Fixed size                | Dynamic size               |
| Fast                      | Slower (allocation needed) |
| LIFO (Last In, First Out) | Arbitrary memory layout    |
| Stores values directly    | Stores pointer to data     |

Types like `i32`, `bool`, `char` → stored on stack  
Types like `String` → stored on heap

---

## 🚚 Move Semantics

When assigning a `String`:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

Ownership moves from `s1` to `s2`.

`s1` becomes invalid.

This prevents **double free errors**.

---

## 🧬 Clone (Deep Copy)

If you need both values:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

This copies heap data.

⚠️ `clone()` can be expensive.

---

## 📦 Copy Trait (Stack-only types)

Simple types implement `Copy`:

- All integers (`i32`, `u32`, etc.)
- `bool`
- `char`
- `f32`, `f64`
- Tuples (if all elements implement `Copy`)

Example:

```rust
let x = 5;
let y = x; // copied, not moved
```

---

## 🏃 Ownership and Functions

Passing a `String` to a function moves it:

```rust
fn takes_ownership(s: String) {
    println!("{s}");
}
```

Passing an `i32` copies it instead.

---

## 🔁 Returning Ownership

Ownership can be transferred back via return values:

```rust
fn gives_ownership() -> String {
    String::from("hello")
}
```

---

## 🧨 Why This Matters

Ownership prevents:

- Double free errors
- Dangling pointers
- Memory leaks
- Data races

All at **compile time**.

No runtime cost.  
No garbage collector.

---

## 🏁 Summary

Ownership is Rust’s solution to safe memory management.

- Move prevents double free
- Clone enables deep copy
- Copy enables cheap stack duplication
- Drop runs automatically at end of scope

Understanding ownership unlocks the rest of Rust.
