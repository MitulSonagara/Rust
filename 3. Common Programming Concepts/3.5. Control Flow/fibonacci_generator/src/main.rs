// =======================================================
// 🦀 Fibonacci Generator
// Generate the nth Fibonacci number
// =======================================================

use std::io;

fn main() {
    println!("Enter the position (n) to get nth Fibonacci number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number.");
            return;
        }
    };

    let result = fibonacci(n);

    println!("Fibonacci number at position {n} is {result}");
}

// Iterative Fibonacci (efficient)
fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}
