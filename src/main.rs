use std::io;

mod prime;
mod calc;
mod trapdoor;

fn main() {
    println!("Type 2 natural numbers for homomorphic addition:");
    let mut input_1 = String::new();
    let mut input_2 = String::new();
    
    io::stdin().read_line(&mut input_1).expect("Failed to read input");
    io::stdin().read_line(&mut input_2).expect("Failed to read input");

    let mut a = input_1.trim().parse::<i64>().expect("Please enter a valid number");
    let mut b = input_2.trim().parse::<i64>().expect("Please enter a valid number");

    println!("Performing homomorphic Encryption...");
    trapdoor::trapdoor(a,b);
}
