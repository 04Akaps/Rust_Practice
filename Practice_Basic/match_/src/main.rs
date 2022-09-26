#[derive(Debug)]

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin : Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Penny!!");
            99
        },
        Coin::Nickel => 5,
        _ => 10
    }
}

fn main() {
    println!("Hello, world!");

    let penny : Coin = Coin::Penny;

    value_in_cents(Coin::Penny);
    value_in_cents(penny);

    
}
