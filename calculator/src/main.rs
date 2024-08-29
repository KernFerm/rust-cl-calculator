use std::io::{self, Write};

fn main() {
    println!("Welcome to the Rust Command-Line Calculator!");
    println!("===========================================\n");

    loop {
        // Prompt for the first number
        print!("Enter the first number (or 'q' to quit): ");
        io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately
        let mut num1_str = String::new();
        io::stdin().read_line(&mut num1_str).expect("Failed to read line");
        if num1_str.trim().to_lowercase() == "q" {
            println!("\nThank you for using the calculator! Created by Bubbles The Dev , Goodbye.");
            break;
        }
        let num1: f64 = match num1_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️  Please enter a valid number.\n");
                continue;
            }
        };

        // Prompt for the operator
        println!("\nOperators:");
        println!("  + : Addition");
        println!("  - : Subtraction");
        println!("  * : Multiplication");
        println!("  / : Division");
        println!("  % : Modulus");
        println!("  ^ : Power");
        println!(" sqrt : Square Root");
        print!("\nEnter an operator: ");
        io::stdout().flush().unwrap();
        let mut operator = String::new();
        io::stdin().read_line(&mut operator).expect("Failed to read line");
        let operator = operator.trim();

        // Prompt for the second number if not using sqrt
        let num2: f64;
        if operator != "sqrt" {
            print!("Enter the second number: ");
            io::stdout().flush().unwrap();
            let mut num2_str = String::new();
            io::stdin().read_line(&mut num2_str).expect("Failed to read line");
            num2 = match num2_str.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("⚠️  Please enter a valid number.\n");
                    continue;
                }
            };
        } else {
            num2 = 0.0; // Not used in sqrt operation
        }

        // Perform the calculation
        let result: f64 = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    println!("⚠️  Error: Division by zero!\n");
                    continue;
                }
            }
            "%" => {
                if num2 != 0.0 {
                    (num1 as i64 % num2 as i64) as f64
                } else {
                    println!("⚠️  Error: Modulus by zero!\n");
                    continue;
                }
            }
            "^" => num1.powf(num2),
            "sqrt" => {
                if num1 >= 0.0 {
                    num1.sqrt()
                } else {
                    println!("⚠️  Error: Square root of a negative number!\n");
                    continue;
                }
            }
            _ => {
                println!("⚠️  Invalid operator!\n");
                continue;
            }
        };

        // Display the result
        println!("\n===============================");
        println!("  The result is: {:.2}", result);
        println!("===============================\n");
    }
}
