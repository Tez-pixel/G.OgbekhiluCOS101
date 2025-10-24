use std::io;

fn main() {
    // Get user input for a, b, c
    let mut input = String::new();

    println!("Enter value for a: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let a: f64 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    println!("Enter value for b: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let b: f64 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    println!("Enter value for c: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let c: f64 = input.trim().parse().expect("Please enter a valid number");

    // Compute the discriminant
    let discriminant = b * b - 4.0 * a * c;

    // Check discriminant to determine root nature
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {:.2} and {:.2}", root1, root2);
    } else if discriminant == 0.0
    {
        let root = -b / (2.0 * a);
        println!("One real root: {:.2}", root);
    } else {
        println!("No real roots (discriminant is negative).");
    }
}