# 🦀 Rust Control Flow – Module Notes

## 📌 Overview

Control flow allows you to decide:

- Which code runs
- How many times it runs
- When to stop execution

Rust provides:

- `if` / `else`
- `loop`
- `while`
- `for`
- `break`
- `continue`

---

# 1️⃣ if / else

```rust
if condition {
    // code
} else {
    // code
}
```

Rules:

- Condition must be `bool`
- No parentheses required
- `if` is an expression (can return a value)

Example:

```rust
let number = 5;

if number > 0 {
    println!("Positive");
}
```

---

## if as an Expression

```rust
let number = 5;

let result = if number > 0 {
    1
} else {
    -1
};
```

Both branches must return the same type.

---

# 2️⃣ loop (Infinite Loop)

```rust
loop {
    // code
}
```

Stops with:

- `break`
- Program exit

Example:

```rust
loop {
    break;
}
```

---

## loop Returning Value

```rust
let result = loop {
    break 10;
};
```

---

# 3️⃣ while Loop

```rust
while condition {
    // code
}
```

Runs while condition is `true`.

---

# 4️⃣ for Loop

Best way to iterate collections.

```rust
for element in collection {
    // code
}
```

Example:

```rust
let arr = [1, 2, 3];

for num in arr {
    println!("{num}");
}
```

---

## Range Loop

```rust
for number in 1..=5 {
    println!("{number}");
}
```

`1..=5` → inclusive range

---

# 5️⃣ break and continue

- `break` → exit loop
- `continue` → skip to next iteration

---

# 🧠 Key Takeaways

- `if` requires boolean
- `if` is an expression
- `loop` can return values
- Prefer `for` for iteration
- Rust prevents unsafe looping errors

---

Control flow is the foundation of real program logic.
