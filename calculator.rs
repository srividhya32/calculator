use std::io;

fn main() {
    loop {
        println!("Choose an operation:");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        if choice == 5 {
            println!("Exiting the calculator.");
            break;
        }

        println!("Enter the first number:");
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Failed to read line");
        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        println!("Enter the second number:");
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Failed to read line");
        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        match choice {
            1 => perform_operation(&num1, &num2, |a, b| a + b, "addition"),
            2 => perform_operation(&num1, &num2, |a, b| a - b, "subtraction"),
            3 => perform_operation(&num1, &num2, |a, b| a * b, "multiplication"),
            4 => perform_operation(&num1, &num2, |a, b| a / b, "division"),
            _ => println!("Invalid choice. Please select a valid operation."),
        }
    }
}

fn perform_operation<F>(num1: &f64, num2: &f64, operation: F, operation_name: &str)
where
    F: Fn(f64, f64) -> f64,
{
    let result = operation(*num1, *num2);
    println!("Result of {} is: {}", operation_name, result);
}
