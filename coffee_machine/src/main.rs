// Still a work under progress...........

use coffee_machine::coffee_machine::{CoffeeMachine, CoffeeType};

#[allow(unused_mut)]
fn main() {
    let mut coffee_machine = CoffeeMachine {
        water: 300,
        milk: 200,
        coffee: 100,
        money: 0.0,
    };

    coffee_machine.report();

    let coffee_type = CoffeeType::Espresso;
    let money = 4.00;

    coffee_machine.make(&coffee_type, money);
    coffee_machine.report();

}
