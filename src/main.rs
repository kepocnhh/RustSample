use crate::UsState::{Alabama, Alaska};

fn main() {
    for it in [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter(Alabama), Coin::Quarter(Alaska)] {
        let value = value_in_cents(&it);
        println!("coin: {:?} -> value: {}", it, value);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_state) => 25,
    }
}