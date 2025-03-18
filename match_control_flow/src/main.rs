

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Colorado,
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}





fn main() {
    
}


fn value_in_cents(coin: Coin) -> u8 {
    // with an if statement the condition has to evalue to a bool
    // but with match it can be any type
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }

}
