mod matching_with_options;
use crate::matches::matching_with_options::matching_with_options;

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

#[derive(Debug)]
enum UsState {
    India,
    Usa,
    Pakistan,
}

#[derive(Debug)]
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn matches() {
    println!("\n M A T C H \n");

    // match: to match the possible pattern and write the execution against that pattern
    let coin_1 = Coin::Nickel;
    let coin_1_cents = value_in_cents(&coin_1);
    println!("Value of coin_1 : {coin_1:?} is {coin_1_cents}");


    // we can perform multiple operations also inside the match case:
    print_and_value_in_cents(&coin_1);


    // we can use match to access the associated values with a enum:
    let coin_2 = Coin2::Quarter(UsState::Usa);
    let value = value_in_cents_with_quarter_type_check(&coin_2);
    println!("value of {coin_2:?} -> {value}");


    matching_with_options();
}

// 1. function to tell the value of coin in cents:
fn value_in_cents(coin : &Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// 2. function to print and tell the value of coin in cents:
fn print_and_value_in_cents(coin : &Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!{"The coin is a penny, so the value will be : 1"};
            1
        },
        Coin::Nickel => {
            println!("The coin is a nickel, so the value will be 5.");
            5
        },
        Coin::Dime => {
            println!("The coin is a dime, so the value will be 10.");
            10
        },
        Coin::Quarter => {
            println!("The coin is a quarter, so the value will be 25.");
            25
        },
    }
}

// 3. function to check the coin and associated value and return the value:
fn value_in_cents_with_quarter_type_check(coin: &Coin2) -> i32 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("Value is a Quarter : checking value in cents based on UsState type:");
            get_quarter_state_value(state)
        },
    }
}

fn get_quarter_state_value(state : &UsState) -> i32 {
    match state {
        UsState::India => 26,
        UsState::Pakistan => 27,
        UsState::Usa => 29,
    }
}
