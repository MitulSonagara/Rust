# 🦀 Rust Basics – Quick Revision

## 1️⃣ Variables (Immutable by Default)

- Variables in Rust are **immutable by default**
- Once assigned, the value cannot change
- Prevents accidental state changes
- Improves safety & code predictability

To allow changes, use `mut`.

---

## 2️⃣ Mutable Variables (`mut`)

- `mut` allows reassignment
- Still cannot change the variable’s type
- Use only when mutation is actually needed

✔ Changes value  
❌ Cannot change type

---

## 3️⃣ Constants (`const`)

- Declared using `const` (not `let`)
- Must include a **type annotation**
- Must be assigned a **compile-time constant expression**
- Always immutable
- Naming convention: `SCREAMING_SNAKE_CASE`
- Can be declared in global scope

Use constants for shared, fixed values.

---

## 4️⃣ Shadowing

- Declare a new variable with the same name using `let`
- New variable overrides the previous one
- Can change the variable’s **type**
- Scope-based behavior (inner scope shadowing disappears after scope ends)

✔ Creates new binding  
✔ Can change type  
✔ Keeps immutability after transformation

---

## 5️⃣ `mut` vs Shadowing

| `mut`                 | Shadowing            |
| --------------------- | -------------------- |
| Changes same variable | Creates new variable |
| Cannot change type    | Can change type      |
| Requires `mut`        | Requires `let` again |

---

## 🧠 Core Idea

Rust encourages:

- Immutability by default
- Explicit mutation
- Predictable state
- Safer code

Compiler errors are guardrails, not punishment.
