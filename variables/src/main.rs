// ==========================================================
// 🦀 Rust Basics: Variables, Mutability, Constants & Shadowing
// ==========================================================


// ----------------------------------------------------------
// 1️⃣ Immutable Variable (Default Behavior in Rust)
// ----------------------------------------------------------
// By default, variables in Rust are immutable.
// Once assigned, their value cannot be changed.

/*
fn main() {
    let x = 5; // `x` is immutable
    println!("The value of x is: {x}");

    x = 6; 
    // ❌ Compile-time error:
    // error[E0384]: cannot assign twice to immutable variable `x`
}
*/


// ----------------------------------------------------------
// 2️⃣ Mutable Variable
// ----------------------------------------------------------
// Use the `mut` keyword to allow mutation.

/*
fn main() {
    let mut x = 5; // `x` is mutable
    println!("The value of x is: {x}");

    x = 6; // ✅ Allowed
    println!("The value of x is: {x}");
}
*/


// ----------------------------------------------------------
// 3️⃣ Constants
// ----------------------------------------------------------
// Constants:
// - Declared using `const`
// - MUST include a type annotation
// - Must be assigned a value at compile time
// - Cannot be mutable
// - Naming convention: SCREAMING_SNAKE_CASE

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


// ----------------------------------------------------------
// 4️⃣ Shadowing
// ----------------------------------------------------------
// Shadowing allows you to declare a new variable with the
// same name as a previous variable.
//
// The new variable "shadows" (overrides) the previous one.
// This is DIFFERENT from `mut`.
//
// With shadowing:
// - You can change the type of a variable
// - Each `let` creates a completely new variable
// - Scope rules apply

fn main() {
    println!(
        "Constant value: THREE_HOURS_IN_SECONDS = {}",
        THREE_HOURS_IN_SECONDS
    );

    // Initial immutable variable
    let x = 5;
    println!("Initial value of x: {x}");

    // Shadowing: new variable `x`
    let x = x + 1;
    println!("After shadowing (x + 1): {x}");

    {
        // Inner scope shadowing
        let x = x * 2;
        println!("Value of x in inner scope: {x}");
    }

    // Back to outer scope
    println!("Value of x in outer scope: {x}");
}