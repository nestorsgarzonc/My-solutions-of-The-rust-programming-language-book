enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("Hello, match!");
    let coint_dime = Coin::Dime;
    let res_dime = value_in_cents(coint_dime);
    println!("ResDime: {}", res_dime);
    let coint_penny = Coin::Penny;
    let res_penny = value_in_cents(coint_penny);
    println!("ResPenny: {}", res_penny);
    let coint_nickel = Coin::Nickel;
    let res_nickel = value_in_cents(coint_nickel);
    println!("ResNickel: {}", res_nickel);
    let coint_quarter = Coin::Quarter;
    let res_quarter = value_in_cents(coint_quarter);
    println!("ResDime: {}", res_quarter);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter => 4,
    }
}
