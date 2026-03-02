# 🦀 Rust Functions – Module Notes

## 📌 Overview

Functions are fundamental in Rust.

- Declared using `fn`
- Use **snake_case** naming convention
- `main` is the program entry point
- Order of function definitions does not matter

---

# 1️⃣ Basic Function

```rust
fn main() {
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

- Functions are called using `name()`
- Curly braces define the function body

---

# 2️⃣ Parameters

Functions can take parameters.

```rust
fn print_value(x: i32) {
    println!("Value: {x}");
}
```

- Parameter types **must** be specified
- Multiple parameters separated by commas

```rust
fn print_measurement(value: i32, unit: char) {
    println!("{value}{unit}");
}
```

---

# 3️⃣ Statements vs Expressions

## 🧾 Statements

- Perform actions
- Do NOT return values

```rust
let x = 5; // statement
```

Function definitions are also statements.

---

## 🔥 Expressions

- Evaluate to a value
- No trailing semicolon

```rust
let y = {
    let x = 3;
    x + 1
};
```

If you add `;`, it becomes a statement and returns `()`.

---

# 4️⃣ Functions with Return Values

Syntax:

```rust
fn function_name() -> Type {
    expression
}
```

Example:

```rust
fn five() -> i32 {
    5
}
```

Return value = last expression.

---

## Explicit Return

```rust
fn example() -> i32 {
    return 10;
}
```

Early return supported.

---

# 5️⃣ Important Rule

Adding a semicolon changes an expression into a statement:

```rust
fn plus_one(x: i32) -> i32 {
    x + 1   // ✅ returns i32
}

fn plus_one(x: i32) -> i32 {
    x + 1;  // ❌ returns ()
}
```

Rust will throw a **mismatched types error** if return type doesn’t match.

---

# 🧠 Key Takeaways

- Functions use `fn`
- Parameters must include types
- Rust is expression-based
- Last expression = return value
- No semicolon for return expressions
- `()` is unit type (no value)

---

Functions are foundational for understanding control flow and ownership next.
