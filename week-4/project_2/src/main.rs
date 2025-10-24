use std::io;

fn main() {
    let mut experience_input = String::new();
    let mut age_input = String::new();

    // Get experience input
    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");

    let experience = experience_input.trim().to_lowercase();

    // Get age input
    println!("Enter the employee's age: ");
    io::stdin()
    .read_line(&mut age_input)
    .expect("Failed to read input");
    let age:u32 = age_input.trim().parse().expect("Please enter a valid number");

    // Determine incentive
    let incentive:u32;

    if experience == "yes" {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age > 30 && age < 40 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            incentive = 1_480_000; // Optional default for ages 28–30 if desired
        }
    } else {
        incentive = 100_000;
    }

    println!("The annual incentive is: ₦{}", incentive);
}