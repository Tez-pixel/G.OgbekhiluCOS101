fn main() {
    let fullname = "Pan-Antlantic University";
    println!();
    println!("Name{}", fullname);
    println!();
    println!("Before trim");
    println!("length is {}", fullname.len());
    println!();
    println!("After trim");
    println!("lenght is {}", fullname.trim().len());
}
