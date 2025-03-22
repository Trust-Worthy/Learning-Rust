use std::io;
use rand::Rng;
use std::cmp::Ordering;



pub struct Guess {
    value:
}

fn main() {
    
    println!("Welcome to the guessing game...");
    println!("You're going to enter a few numbers and we gon see how nice you are at guessing.");

    let secret_num: i32 = rand::thread_rng().gen_range(1..=100);

    
    loop {
        
        
        println!("Please enter your guess below:\n-->");
        let mut guess: String = String::new();


        io::stdin() // this is the way to receive user in
            .read_line(&mut guess)
            .expect("Failed to read line!");

        
        let guess: i32  = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        if guess < 1|| guess > 100 {
            println!("Try again. The secret number will be between 1 and 100.x  ");
            continue;
        }


        println!("You guessed: {}",guess);
            

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Guess higher!"),
            Ordering::Greater => println!("Guess lower"),
            Ordering::Equal => {
                println!("Bingo! You got it.");
                break;
            } 
        
        }
    }



}
