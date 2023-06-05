use std::io;

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

    pub fn make(&mut self, coffee_type: CoffeeType, money: f64){
        match coffee_type {
            CoffeeType::Espresso => {
                if self.water < 50 {
                    println!("Not enough water!");
                } else if self.coffee < 18 {
                    println!("Not enough coffee!")
                } else if money < 1.50 {
                    println!("Not enough money!");
                    println!("you need {:.2}$ more to get an Espresso!", (1.50 - money));
                } else {
                    self.water -= 100;
                    self.coffee -= 18;
                    self.money += 1.50;
                    println!("Here is your ${:.2} in change", (money-1.50));
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
                    println!("you need {:.2}$ more to get a Latte!", (2.50 - money));
                } else {
                    self.water -= 200;
                    self.milk -= 150;
                    self.coffee -= 24;
                    self.money += 2.50;
                    println!("Here is your ${:.2} in change", (money-2.50));
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
                    println!("you need {:.2}$ more to get a Latte!", (3.00 - money));
                } else {
                    self.water -= 250;
                    self.milk -= 100;
                    self.coffee -= 24;
                    self.money += 3.00;
                    println!("Here is your ${:.2} in change", (money-3.00));
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

#[derive(Debug)]
pub struct Coins {
    pub pennies: f64,
    pub nickels : f64,
    pub dimes : f64,
    pub quarters : f64,
}

impl Coins {
    pub fn calculate(&self)-> f64 {
        (self.pennies)*0.01 + (self.nickels)*0.05 + (self.dimes)*0.10 + (self.quarters)*0.25
    }
    pub fn take_inp(&mut self) {
        let mut temp_str = String::new();
        println!("Enter number of pennies:");
        io::stdin().read_line(&mut temp_str).expect("Failed to read input!");
        self.pennies = temp_str.trim().parse().expect("please enter valid number!");
        temp_str.clear();
        println!("Enter number of nickels:");
        io::stdin().read_line(&mut temp_str).expect("Failed to read input!");
        self.nickels = temp_str.trim().parse().expect("please enter valid number!");
        temp_str.clear();
        println!("Enter number of dimes:");
        io::stdin().read_line(&mut temp_str).expect("Failed to read input!");
        self.dimes = temp_str.trim().parse().expect("please enter valid number!");
        temp_str.clear();
        println!("Enter number of quarters:");
        io::stdin().read_line(&mut temp_str).expect("Failed to read input!");
        self.quarters = temp_str.trim().parse().expect("please enter valid number!");
        temp_str.clear();
        println!("you have entered ${:.2}", self.calculate());
    }
}