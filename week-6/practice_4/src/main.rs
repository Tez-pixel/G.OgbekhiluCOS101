fn main() {
    let fullname = "Chibudum John Umeh";
    let department = "Computer Science";
    let uni = "Pan Atlantic University";

    let mut school = "School of Science".to_string();
    // push string
    school.push_str("and Technology");

    println!("My name is:{}",fullname);
    // check lengt
    println!("The lengt of my fullname is: {}",fullname.len());
    println!("I am a student of department: {}",department);
    println!("{}",school);
    println!("{}",uni);
  


}
