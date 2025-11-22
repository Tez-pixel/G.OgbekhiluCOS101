#[derive(Debug)]
struct Developer {
    name: String,
    years_experience: u32,
}

fn main() {
    // A list (vector) of developers
    let developers = vec![
        Developer {
            name: String::from("Steve Johnson"),
            years_experience: 15,
        },
        Developer {
            name: String::from("Michael Ademide"),
            years_experience: 8,
        },
        Developer {
            name: String::from("Chinwe Isaac"),
            years_experience: 3,
        },
        Developer {
            name: String::from("Bola Adebimpe"),
            years_experience: 12,
        },
    ];

    // Find the developer with the highest years of experience
    let most_experienced = developers
        .iter()
        .max_by_key(|dev| dev.years_experience)
        .unwrap();

    println!(
        "The applicant with the highest experience is {} with {} years.",
        most_experienced.name, most_experienced.years_experience
    );
}