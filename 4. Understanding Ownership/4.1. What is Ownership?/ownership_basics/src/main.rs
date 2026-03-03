fn main() {
    println!("--- Ownership Basics Demo ---");

    // ===============================
    // 1️⃣ Move Semantics
    // ===============================

    let s1 = String::from("hello");
    let s2 = s1; // ownership moved

    // println!("{s1}"); ❌ This would cause compile error
    println!("s2 owns the value: {s2}");

    // ===============================
    // 2️⃣ Clone (Deep Copy)
    // ===============================

    let s3 = String::from("world");
    let s4 = s3.clone(); // deep copy of heap data

    println!("s3 = {s3}, s4 = {s4}");

    // ===============================
    // 3️⃣ Copy Trait (Stack Data)
    // ===============================

    let x = 10;
    let y = x; // copied, not moved

    println!("x = {x}, y = {y}");

    // ===============================
    // 4️⃣ Ownership in Functions
    // ===============================

    let s5 = String::from("ownership");
    takes_ownership(s5);
    // println!("{s5}"); ❌ moved

    let num = 42;
    makes_copy(num);
    println!("num is still usable: {num}");

    // ===============================
    // 5️⃣ Return Ownership
    // ===============================

    let s6 = gives_ownership();
    println!("Received ownership: {s6}");

    let s7 = String::from("rust");
    let s8 = takes_and_returns(s7);
    println!("Returned ownership: {s8}");
}

// Takes ownership of a String
fn takes_ownership(s: String) {
    println!("Function took ownership: {s}");
}

// Copies integer (Copy trait)
fn makes_copy(n: i32) {
    println!("Function received copy: {n}");
}

// Gives ownership to caller
fn gives_ownership() -> String {
    String::from("returned value")
}

// Takes and returns ownership
fn takes_and_returns(s: String) -> String {
    println!("Taking and returning: {s}");
    s
}
