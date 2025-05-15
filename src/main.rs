use std::io::{self, Write};

#[derive(Debug)]
enum Unit {
    Length,
    Weight,
    Temperature,
}

fn main() {
    println!("Welcome to Unit Converter!");
    println!("Available conversions:");
    println!("1. Length (meters <-> feet)");
    println!("2. Weight (kilograms <-> pounds)");
    println!("3. Temperature (Celsius <-> Fahrenheit)");

    loop {
        print!("\nSelect conversion type (1-5) or 'q' to quit: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        if choice == "q" {
            println!("Goodbye!");
            break;
        }

        let unit = match choice {
            "1" => Unit::Length,
            "2" => Unit::Weight,
            "3" => Unit::Temperature,
            _ => {
                println!("Invalid choice. Please select 1-3 or 'q' to quit.");
                continue;
            }
        };

        convert_units(unit);
    }
}

fn convert_units(unit: Unit) {
    print!("Enter value to convert: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let value: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    match unit {
        Unit::Length => {
            println!("{} meters = {} feet", value, value * 3.28084);
            println!("{} feet = {} meters", value, value / 3.28084);
        },
        Unit::Weight => {
            println!("{} kilograms = {} pounds", value, value * 2.20462);
            println!("{} pounds = {} kilograms", value, value / 2.20462);
        },
        Unit::Temperature => {
            let fahrenheit = (value * 9.0/5.0) + 32.0;
            let celsius = (value - 32.0) * 5.0/9.0;
            println!("{} Celsius = {} Fahrenheit", value, fahrenheit);
            println!("{} Fahrenheit = {} Celsius", value, celsius);
        },
    }
}
