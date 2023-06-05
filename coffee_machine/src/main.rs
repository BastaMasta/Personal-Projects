use coffee_machine::coffee_machine::{CoffeeMachine, CoffeeType, Coins};
use std::io;

#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() {
    let mut coffee_machine = CoffeeMachine {
        water: 300,
        milk: 200,
        coffee: 100,
        money: 0.0,
    };
    loop {

        println!("What would you like? (espresso/latte/cappuccino):");
        let mut inp_coffee = String::new();
        io::stdin().read_line(&mut inp_coffee).expect("Failed to read input!");

        let mut coins = Coins {
            pennies : 0.0,
            nickels : 0.0,
            dimes : 0.0,
            quarters : 0.0,
        };

        if inp_coffee.trim().to_lowercase() == "espresso" {
            coins.take_inp();
            coffee_machine.make(CoffeeType::Espresso, coins.calculate());
        } else if inp_coffee.trim().to_lowercase() == "latte" {
            coins.take_inp();
            coffee_machine.make(CoffeeType::Latte, coins.calculate());
        } else if inp_coffee.trim().to_lowercase() == "cappuccino" {
            coins.take_inp();
            coffee_machine.make(CoffeeType::Cappuccino, coins.calculate());
        } else if inp_coffee.trim().to_lowercase() == "report".to_string() {
            coffee_machine.report();
        }
        else if (inp_coffee.trim().to_lowercase() == "quit")
            || (inp_coffee.trim().to_lowercase() == "q")
            || (inp_coffee.trim().to_lowercase() == "exit") {
            println!("Thanks for using my coffee machine!");
            break;
        }
    }

}
