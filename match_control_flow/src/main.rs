

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Colorado,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Colorado => year >= 1819,
            UsState::Alabama => year >= 1945,
            UsState::Alaska => year >= 1923,
        }
    }
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}





fn main() {

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("{:?}",five);

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // example of empty tuple type 
    }


    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("to the maxxxxx {}",max),
        _ => (),
    }


    // if let syntax is more simple in some cases . if let is syntax sugar for match
    // but I lose the exhaustive checking of a match statement
    if let Some(max) = config_max {
        println!("To the max via an if let {max}");
    }

    let coin = Coin::Quarter(UsState::Colorado);
    let mut count = 1;

    if let Coin::Quarter(state) = coin {
        println!("State quarter from US state {state:?}");
    } else {
        count += 1;
    }


}

fn add_fancy_hat(){}
fn remove_fancy_hat(){}

fn plus_one(x: Option<i32>) -> Option<i32> { 
    // Match statements similar to Switch statements in most languages have to cover ALL patterns / possibilities!!!
    // Matches are exhaustive in Rust!
    match x {
        // I get it now. None is the first field of the Option enum. Some is the second. I can add an 'i' or any other variable to represent whatever 
        // type of Option it is!
        None => None, 
        Some(i) => Some(i + 1)
    }
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
