fn main() {
    for it in [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter] {
        let value = value_in_cents(&it);
        println!("coin: {:?} -> value: {}", it, value);
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}