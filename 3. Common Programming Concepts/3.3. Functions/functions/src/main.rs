// =======================================================
// 🦀 Rust Functions – Full Demonstration Module
// =======================================================

fn main() {
    println!("--- BASIC FUNCTION CALL ---");

    // Calling a function
    another_function();

    println!("\n--- FUNCTION WITH PARAMETERS ---");

    // Passing arguments
    print_value(10);
    print_measurement(5, 'h');

    println!("\n--- EXPRESSIONS & BLOCKS ---");

    // Block expression example
    let result = {
        let x = 3;
        x + 1 // No semicolon → expression returns 4
    };

    println!("Block expression result: {result}");

    println!("\n--- FUNCTION WITH RETURN VALUE ---");

    let five_value = five();
    println!("five() returned: {five_value}");

    let plus = plus_one(5);
    println!("plus_one(5) returned: {plus}");

    println!("\n--- EARLY RETURN EXAMPLE ---");

    let check = check_number(7);
    println!("check_number(7) returned: {check}");

    println!("\n--- END OF FUNCTIONS MODULE ---");
}

// =======================================================
// 1️⃣ Basic Function
// =======================================================

fn another_function() {
    println!("Another function executed.");
}

// =======================================================
// 2️⃣ Function with Single Parameter
// =======================================================

fn print_value(x: i32) {
    // x must have a declared type
    println!("Value received: {x}");
}

// =======================================================
// 3️⃣ Function with Multiple Parameters
// =======================================================

fn print_measurement(value: i32, unit_label: char) {
    println!("Measurement: {value}{unit_label}");
}

// =======================================================
// 4️⃣ Function Returning a Value
// =======================================================

fn five() -> i32 {
    // No semicolon → this expression is returned
    5
}

// =======================================================
// 5️⃣ Function Returning Computed Value
// =======================================================

fn plus_one(x: i32) -> i32 {
    // Expression-based return
    x + 1
}

// =======================================================
// 6️⃣ Early Return Example
// =======================================================

fn check_number(x: i32) -> i32 {
    if x > 5 {
        return x; // Explicit early return
    }

    // Implicit return (no semicolon)
    0
}
