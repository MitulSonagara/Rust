// =====================================================
// 🦀 Rust Data Types – Full Demonstration Module
// =====================================================

use std::io;

fn main() {
    println!("--- SCALAR TYPES ---");

    // ===============================
    // 1️⃣ Integer Types
    // ===============================

    // Default integer type is i32
    let _integer_default = 42;

    // Explicit signed and unsigned integers
    let signed: i32 = -10;
    let unsigned: u32 = 10;

    // Integer literals in different formats
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // u8 only

    println!("Integer examples: {}, {}, {}", signed, unsigned, decimal);
    println!(
        "Hex: {}, Octal: {}, Binary: {}, Byte: {}",
        hex, octal, binary, byte
    );

    // Integer overflow example (checked safely)
    let x: u8 = 255;
    println!("Checked add overflow: {:?}", x.checked_add(1)); // None

    // ===============================
    // 2️⃣ Floating-Point Types
    // ===============================

    // Default float type is f64
    let float_default = 2.0; // f64
    let float_explicit: f32 = 3.14;

    println!("Floats: {}, {}", float_default, float_explicit);

    // ===============================
    // 3️⃣ Numeric Operations
    // ===============================

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // integer division
    let remainder = 43 % 5;

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Truncated division: {}", truncated);
    println!("Remainder: {}", remainder);

    // ===============================
    // 4️⃣ Boolean Type
    // ===============================

    let t = true;
    let f: bool = false;

    println!("Booleans: {} and {}", t, f);

    // ===============================
    // 5️⃣ Character Type
    // ===============================

    let c = 'z';
    let unicode_char: char = 'ℤ';
    let emoji = '😻';

    println!("Characters: {}, {}, {}", c, unicode_char, emoji);

    println!("\n--- COMPOUND TYPES ---");

    // ===============================
    // 6️⃣ Tuple
    // ===============================

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring
    let (x, y, z) = tup;
    println!("Tuple destructured: {}, {}, {}", x, y, z);

    // Index access
    println!("Tuple via index: {}, {}, {}", tup.0, tup.1, tup.2);

    // Unit type
    let unit_value = ();
    println!("Unit value: {:?}", unit_value);

    // ===============================
    // 7️⃣ Array
    // ===============================

    let array = [1, 2, 3, 4, 5];

    // Explicit type
    let typed_array: [i32; 5] = [10, 20, 30, 40, 50];

    // Repeated value initialization
    let repeated = [3; 5];

    println!("Array first element: {}", array[0]);
    println!("Typed array: {:?}", typed_array);
    println!("Repeated array: {:?}", repeated);

    // ===============================
    // 8️⃣ Safe Array Access Example
    // ===============================

    println!("\nEnter an array index (0-4):");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let index: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Exiting safely.");
            return;
        }
    };

    if index < array.len() {
        println!("Element at index {} is {}", index, array[index]);
    } else {
        println!("Index out of bounds. Prevented panic.");
    }

    println!("\n--- End of Data Types Module ---");
}
