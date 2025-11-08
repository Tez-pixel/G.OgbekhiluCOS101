use std::io;

fn main() {
    

 println!("  Letter         Menu                            Price");
  println!(" P          Poundo Yam / Edinkaiko Soup       #3,200");
 println!(" F        Fried Rice & Chicken                 #3,000");
 println!(" A      Amala & Ewedu Soup                     #2,500");
  println!("E     Eba & Egusi Soup                        #2,000");
   println!("W     White Rice & Stew         #2,500");



let mut food = String::new();
        println!("Select which food between P, F, A ,E");
        io::stdin().read_line(&mut food).expect("Invalid Input");
        let food = food.trim().to_uppercase();

        println!("Enter quantity");
let mut quant = String::new();
io::stdin().read_line(&mut quant).expect("Read invaid input");
let quant:i32 = quant.trim().parse().expect("Invalid Input");

let price = match food.as_str() {
    "P" => 3200,
    "F" => 3000,
    "A" => 2500,
    "E" => 2000,
    "W" => 2500,
    _=> {
        println!("Invalid food entered");
        return;
    }

};
let total = price * quant;
let mut final_total = total as f64;

if total > 10_000 {
    let discount = 0.05 * final_total;
    final_total -= discount;
    println!("A 5% dscount has been added");
}       

   println!("\nTotal amount to pay: #{}", final_total); 

}