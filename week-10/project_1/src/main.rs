struct Laptop {
    hp:i32,
    ibm:i32,
    toshiba:i32,
    dell:i32,
}
impl Laptop {
    fn total_cost(&self, quant:i32) ->i32 {
        (self.hp * quant) + (self.ibm * quant) + (self.toshiba * quant) + (self.dell * quant)
    }
}

fn main() {
 let prize = Laptop {
    hp:650000,
    ibm:755000,
    toshiba:550000,
    dell:850000,
 };
 let quant = 3;
 let total = prize.total_cost(quant);

 println!("Total cost for buying 3 laptops from each brand is therefore: {}", total);
}
