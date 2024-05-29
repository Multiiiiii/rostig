enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}


fn main() {
    let penny = value_in_cents(Coin::Penny);
    println!("A penny is worth {} cent.", penny);
    let Querter_Alaska = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("A quarter from Alaska is worth {} cent.", Querter_Alaska);

    let five = Some(5);
    let six = plus_one(five);
    let nono =plus_one(None);

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }

    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
