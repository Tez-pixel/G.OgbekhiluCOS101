use std::io;

fn main() {
    println!("APS Level Checker");

    // --- INPUT: Job title ---
    println!("Enter job title (e.g., Associate Lawyer, Teacher, Partner):");
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim().to_lowercase();

    // --- INPUT: Years of experience ---
    println!("Enter years of experience:");
    let mut years_str = String::new();
    io::stdin().read_line(&mut years_str).unwrap();
    let years: u8 = years_str.trim().parse().unwrap_or(0);

    // --- Determine APS level based on years ---
    let level_by_years = if years <= 2 {
        "APS1–2"
    } else if years <= 4 {
        "APS3–5"
    } else if years <= 6 {
        "APS5–8"
    } else if years <= 10 {
        "EL8–10"
    } else {
        "SES"
    };

    // --- Determine APS allowed level based on job title ---
    let level_by_title = if matches_any(&title, &[
        "intern", "", "paralegal", "placement" 
    ]) {
        "APS1–2"
    } else if matches_any(&title, &[
        "administrator", "research assistant", "junior associate", "classroom teacher"
    ]) {
        "APS3–5"
    } else if matches_any(&title, &[
        "senior administrator", "PhD Candidate", "associate", " Snr Teacher"
    ]) {
        "APS5–8"
    } else if matches_any(&title, &[
        "Office manager", "Post-Doc researcher", "Senior Associate1-2", "Leading teacher"
    ]) {
        "EL8–10"}
        else if matches_any(&title, &[
        "director", "senior lecturer", "Senior Associate3-4", "Deputy Principal"
    ]) {
        "EL10–13"}
     else if matches_any(&title, &[
        "ceo", "dean", "partner", "principal"
    ]) {
        "SES"
    } else {
        "Unknown"
    };

    // --- FINAL DECISION ---
    if level_by_title == "Unknown" {
        println!("Job title not found in APS table.");
    } else {
        // Staff must meet BOTH: title level + experience level
        let final_level = more_senior(level_by_title, level_by_years);

        println!("\n----- RESULT -----");
        println!("Job Title Level: {}", level_by_title);
        println!("Experience Level: {}", level_by_years);
        println!("→ Final APS Level: {}", final_level);
    }
}

fn matches_any(title: &str, list: &[&str]) -> bool {
    list.iter().any(|x| title.contains(&x.to_lowercase()))
}

// Returns the more senior of the two APS levels
fn more_senior(a: &str, b: &str) -> &'static str {
    let order = ["APS1–2", "APS3–4", "APS5–6", "EL1–2", "SES"];

    let pos_a = order.iter().position(|&x| x == a).unwrap_or(0);
    let pos_b = order.iter().position(|&x| x == b).unwrap_or(0);

    if pos_a > pos_b { a } else { b }
}