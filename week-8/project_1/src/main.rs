use std::io;

fn main() {
    // === OCCUPATION GROUPS ===
    let public_servant = vec!["intern", "senior administrator", "director", "ceo"];
    let administrator = vec!["administrator", "office manager"];
    let academic = vec!["research assistant", "phd candidate", "post-doc researcher", "senior lecturer", "dean"];
    let lawyer = vec!["paralegal", "junior associate", "associate", "senior associate"];
    let teacher = vec!["classroom teacher", "snr teacher", "leading teacher", "principal"];

    //  ASK USER FOR INPUT 
    let mut job = String::new();
    let mut years = String::new();

    println!("Enter your occupation (e.g., Associate, Lawyer, Administrator):");
    io::stdin().read_line(&mut job).unwrap();
    let job = job.trim().to_lowercase();

    println!("Enter your years of experience:");
    io::stdin().read_line(&mut years).unwrap();
    let years: i32 = years.trim().parse().unwrap();

    // DETERMINE OCCUPATION GROUP 
    let group = if public_servant.contains(&job.as_str()) {
        "Public Servant"
    } else if administrator.contains(&job.as_str()) {
        "Administrator"
    } else if academic.contains(&job.as_str()) {
        "Academic"
    } else if lawyer.contains(&job.as_str()) {
        "Lawyer"
    } else if teacher.contains(&job.as_str()) {
        "Teacher"
    } else {
        "Unknown Occupation"
    };

    //  DETERMINE APS LEVEL 
    let staff_level = if years >= 1 && years <= 2 {
        "APS 1-2"
    } else if years >= 3 && years <= 5 {
        "APS 3-5"
    } else if years >= 5 && years <= 8 {
        "APS 5-8"
    } else if years >= 8 && years <= 10 {
        "EL 1 (8–10)"
    } else if years >= 10 && years <= 13 {
        "EL 2 (10–13)"
    } else if years > 13 {
        "SES"
    } else {
        "Invalid years"
    };

    //  OUTPUT RESULT
    println!("\n=== RESULT ===");
    println!("Occupation Group: {}", group);
    println!("Staff Level Based on Experience: {}", staff_level);
}
