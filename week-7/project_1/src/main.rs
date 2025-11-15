use std::io;


fn read_input_f64(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read line");
            
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        }
    }
}




fn calculate_trapezium_area() -> f64 {
    println!("\n--- Area of Trapezium ---");
    let height = read_input_f64("Enter the height:");
    let base1 = read_input_f64("Enter the length of base 1:");
    let base2 = read_input_f64("Enter the length of base 2:");

    (height / 2.0) * (base1 + base2)
}


fn calculate_rhombus_area() -> f64 {
    println!("\n--- Area of Rhombus ---");
    let diagonal1 = read_input_f64("Enter the length of diagonal 1:");
    let diagonal2 = read_input_f64("Enter the length of diagonal 2:");

    0.5 * diagonal1 * diagonal2
}


fn calculate_parallelogram_area() -> f64 {
    println!("\n--- Area of Parallelogram ---");
    let base = read_input_f64("Enter the length of the base:");
    let altitude = read_input_f64("Enter the altitude (height):");

    base * altitude
}


fn calculate_cube_area() -> f64 {
    println!("\n--- Surface Area of Cube ---");
    let side_length = read_input_f64("Enter the length of one side:");

    6.0 * side_length.powf(2.0)
}

fn calculate_cylinder_volume() -> f64 {
    println!("\n--- Volume of Cylinder ---");
    let radius = read_input_f64("Enter the radius:");
    let height = read_input_f64("Enter the height:");
    let pi = 3.142;
    pi * radius.powf(2.0) * height
}


fn main() {
    println!("*MTH 101 -  Calculator in Rust*");

    loop {
        // 1. Prompt the user to select an equation
        println!("\n--- Select an Equation ---");
        println!("1: Area of Trapezium");
        println!("2: Area of Rhombus");
        println!("3: Area of Parallelogram");
        println!("4: Surface Area of Cube");
        println!("5: Volume of Cylinder");
        println!("6: Leave");
        print!("Enter your choice (1-6): ");

        let mut decision = String::new();
        io::stdin().read_line(&mut decision).expect("Failed to read line");
            
            

        let decision: u32 = match decision.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nInvalid decision. Please enter a number between 1 and 6.");
                continue;
            }
        };

        
        let result = match decision {
            1 => calculate_trapezium_area(),
            2 => calculate_rhombus_area(),
            3 => calculate_parallelogram_area(),
            4 => calculate_cube_area(),
            5 => calculate_cylinder_volume(),
            6 => {
                println!("\nExiting program. Goodbye!");
                break; 
            }
            _ => {
                println!("\nInvalid decision. Please select an option from 1 to 6.");
                continue; 
            }
        };

    
        println!("\n Result: The calculated value is {:.2}", result);
    }
}