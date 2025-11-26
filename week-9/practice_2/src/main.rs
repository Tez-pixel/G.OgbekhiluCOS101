use std::io::Write;
use std::io::Read;
fn main() {
    let announce = "Week 9 - Rust File Input & Output\n";
    let dept = "Department of computer science";

    let mut file = std::fs::File::create("message.txt").expect("create failed");
    file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    println!("\nData written to file.");

 








    let mut file = std::fs::File::open("Welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
