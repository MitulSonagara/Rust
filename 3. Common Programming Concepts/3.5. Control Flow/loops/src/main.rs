// =======================================================
// 🦀 Rust Control Flow – LOOPS Module
// =======================================================

fn main() {
    println!("--- LOOP EXAMPLES ---");

    // ============================================
    // 1️⃣ Infinite loop with break
    // ============================================

    let mut counter = 0;

    loop {
        counter += 1;

        if counter == 3 {
            break; // Exit loop
        }
    }

    println!("Counter stopped at {counter}");

    // ============================================
    // 2️⃣ loop returning value
    // ============================================

    let result = loop {
        break 42; // loop returns 42
    };

    println!("Loop returned {result}");

    // ============================================
    // 3️⃣ while loop
    // ============================================

    let mut number = 3;

    while number > 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!");

    // ============================================
    // 4️⃣ for loop over array
    // ============================================

    let array = [10, 20, 30];

    for element in array {
        println!("Value: {element}");
    }

    // ============================================
    // 5️⃣ for loop with range
    // ============================================

    for number in 1..=5 {
        println!("Range value: {number}");
    }

    // ============================================
    // 6️⃣ continue example
    // ============================================

    for number in 1..=5 {
        if number == 3 {
            continue; // Skip 3
        }
        println!("Continue example: {number}");
    }
}
