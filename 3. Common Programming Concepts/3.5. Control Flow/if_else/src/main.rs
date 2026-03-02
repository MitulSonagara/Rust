// =======================================================
// 🦀 Rust Control Flow – IF / ELSE Module
// =======================================================

fn main() {
    println!("--- IF / ELSE EXAMPLES ---");

    let number = 7;

    // ============================================
    // 1️⃣ Basic if / else
    // ============================================

    if number > 0 {
        println!("Number is positive");
    } else {
        println!("Number is not positive");
    }

    // ============================================
    // 2️⃣ Multiple Conditions
    // ============================================

    if number % 4 == 0 {
        println!("Divisible by 4");
    } else if number % 3 == 0 {
        println!("Divisible by 3");
    } else {
        println!("Not divisible by 4 or 3");
    }

    // ============================================
    // 3️⃣ if as Expression
    // ============================================

    let condition = true;

    let value = if condition {
        5 // No semicolon → expression
    } else {
        6
    };

    println!("Value from if expression: {value}");

    // ============================================
    // Important Rule
    // Both branches must return same type
    // ============================================

    // let value = if condition { 5 } else { "six" };
    // ❌ This would fail (mismatched types)
}
