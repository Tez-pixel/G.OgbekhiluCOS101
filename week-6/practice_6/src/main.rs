fn main() {
  let n1 = "Electrical".to_string();
    let n2 = "Electronic".to_string();
      let n3 = "Engineering".to_string();
    let n4 = n1 + &n2 + &n3; // n2 & n3 reference is passed
    
    //about electrical/electronc
    println!("\nThe {} is confirmed by the aspiration to train Electrical/Electronic
        Engineering profesionals i te areas of design, building and maintenance o electrical control system,", n4);

          let w1 = "Computer".to_string();
            let w2 = "Science".to_string();
              let w3 = w1 + &w2;
              println!();
              println!("{} is aimed at developing competent, creative, innovative, entrepreneurial and ethically-minded persons, capable of creating value in the diverse fields of Computer Science", w3);  
}
