// Still a work under progress...........

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
    let mut coins = Coins {
        pennies : 0,
        nickels : 0,
        dimes : 0,
        quarters : 0,
    };
    loop {
        let espresso = String::from("espresso");
        let latte = String::from("latte");
        let cappuccino = String::from("cappuccino");
        println!("What would you like? (espresso/latte/cappuccino):");
        let mut inp_coffee = String::new();
        io::stdin().read_line(&mut inp_coffee).expect("Failed to read input!");

        let mut coins = Coins {
            pennies : 0,
            nickels : 0,
            dimes : 0,
            quarters : 0,
        };
        let mut temp_str = String::new();
        println!("Enter number of pennies:");
        io::stdin().read_line(&mut temp_str).expect("Failed to read input!");
        coins.pennies = temp_str.trim().parse().expect("please enter valid number!");
        temp_str.clear();
        println!("Enter number of nickels:");
        io::stdin().read_line(&mut temp_str).expect("Failed to read input!");
        coins.nickels = temp_str.trim().parse().expect("please enter valid number!");
        temp_str.clear();
        println!("Enter number of dimes:");
        io::stdin().read_line(&mut temp_str).expect("Failed to read input!");
        coins.dimes = temp_str.trim().parse().expect("please enter valid number!");
        temp_str.clear();
        println!("Enter number of quarters:");
        io::stdin().read_line(&mut temp_str).expect("Failed to read input!");
        coins.quarters = temp_str.trim().parse().expect("please enter valid number!");
        temp_str.clear();

        println!("{:?}", coins);



        if espresso.to_lowercase().starts_with(&inp_coffee) {

        }
        break;
    }


    /*coffee_machine.report();

    let coffee_type = CoffeeType::Espresso;
    let money = 4.00;

    coffee_machine.make(&coffee_type, money);
    coffee_machine.report();*/

}
