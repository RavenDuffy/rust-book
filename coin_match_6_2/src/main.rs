#![allow(dead_code)]
#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
    // etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState), // setting our enum to have a value of another us state
}

fn main() {
    let my_coin = Coin::Quarter(USState::Alabama); // init coin with state value
    let value = value_in_cents(my_coin);

    println!("{value}");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // If matched, send the value and print the state value
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}
