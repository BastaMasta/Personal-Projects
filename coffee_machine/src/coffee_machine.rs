pub struct CoffeeMachine {
    pub water : u32,
    pub milk : u32,
    pub coffee : u32,
    pub money : f64,
}

impl CoffeeMachine {
    pub fn report(&self) {
        println!("Water : {}ml", self.water);
        println!("Milk : {}ml", self.milk);
        println!("Coffee : {}g", self.coffee);
    }

    pub fn make(&mut self, coffee_type: &CoffeeType, money: f64){
        match coffee_type {
            CoffeeType::Espresso => {
                if self.water < 50 {
                    println!("Not enough water!");
                } else if self.coffee < 18 {
                    println!("Not enough coffee!")
                } else if money < 1.50 {
                    println!("Not enough money!");
                    println!("you need {}$ more to get an Espresso!", (1.50 - money));
                } else {
                    self.water -= 100;
                    self.coffee -= 18;
                    self.money += 1.50;
                    println!("Here is your ${} in change", (money-1.50));
                    println!("Enjoy your Espresso☕!");
                }
            },
            CoffeeType::Latte => {
                if self.water < 200 {
                    println!("Not enough water!");
                } else if self.milk < 150 {
                    println!("Not enough milk!")
                } else if self.coffee < 24 {
                    println!("Not enough coffee!")
                } else if money < 2.50 {
                    println!("Not enough money!");
                    println!("you need {}$ more to get a Latte!", (2.50 - money));
                } else {
                    self.water -= 200;
                    self.milk -= 150;
                    self.coffee -= 24;
                    self.money += 2.50;
                    println!("Here is your ${} in change", (money-2.50));
                    println!("Enjoy your Latte☕!");
                }
            },
            CoffeeType::Cappuccino => {
                if self.water < 250 {
                    println!("Not enough water!");
                } else if self.milk < 100 {
                    println!("Not enough milk!")
                } else if self.coffee < 24 {
                    println!("Not enough coffee!")
                } else if money < 3.00 {
                    println!("Not enough money!");
                    println!("you need {}$ more to get a Latte!", (3.00 - money));
                } else {
                    self.water -= 250;
                    self.milk -= 100;
                    self.coffee -= 24;
                    self.money += 3.00;
                    println!("Here is your ${} in change", (money-3.00));
                    println!("Enjoy your Latte☕!");
                }
            },
        }
    }
}

pub enum CoffeeType {
    Espresso,
    Latte,
    Cappuccino,
}

pub struct Coins {
    pub pennies: u32,
    pub nickels : u32,
    pub dimes : u32,
    pub quarters : u32,
}

impl Coins {
    pub fn calculate(&self)-> f64 {
        (self.pennies)as f64 *0.1 + (self.nickels)as f64 *0.5 + (self.dimes)as f64 *0.10 + (self.quarters)as f64 *0.25
    }
}