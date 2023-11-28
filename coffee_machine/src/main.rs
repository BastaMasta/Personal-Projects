use coffee_machine::coffee_machine::{CoffeeMachine, CoffeeType};
use std::io;


fn main() {
    let mut coffee_machine = CoffeeMachine::new();
    loop {

        println!("What would you like? (espresso/latte/cappuccino):");
        let mut inp_coffee = String::new();
        io::stdin().read_line(&mut inp_coffee).expect("Failed to read input!");

        match inp_coffee.trim().to_lowercase().as_str() {
            "espresso" => coffee_machine.make(CoffeeType::Espresso),
            "latte" => coffee_machine.make(CoffeeType::Latte),
            "cappuccino" => coffee_machine.make(CoffeeType::Cappuccino),
            "report" => coffee_machine.report(),
            "quit" | "q" | "exit" => {
                println!("Thanks for using my coffee machine!");
                break;
            }
            _ => {
                println!("Please enter a valid coffee type!")
            }
        }
    }

}
