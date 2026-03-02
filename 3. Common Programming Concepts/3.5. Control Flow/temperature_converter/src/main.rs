// =======================================================
// 🦀 Temperature Converter
// Convert between Fahrenheit and Celsius
// =======================================================

use std::io;

fn main() {
    println!("Temperature Converter");
    println!("Type 'C' to convert Celsius → Fahrenheit");
    println!("Type 'F' to convert Fahrenheit → Celsius");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice = choice.trim().to_uppercase();

    println!("Enter temperature value:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read temperature");

    let temp: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number entered.");
            return;
        }
    };

    if choice == "C" {
        let fahrenheit = celsius_to_fahrenheit(temp);
        println!("{temp}°C = {fahrenheit}°F");
    } else if choice == "F" {
        let celsius = fahrenheit_to_celsius(temp);
        println!("{temp}°F = {celsius}°C");
    } else {
        println!("Invalid choice.");
    }
}

// Convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

// Convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}
