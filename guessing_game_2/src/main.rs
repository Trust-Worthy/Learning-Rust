use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    println!("Welcome to the guessing game...");
    println!("You're going to enter a few numbers and we gon see how nice you are at guessing.");

    let secret_num: u32 = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_num}!");


    
    

    let mut guess: String = String::new();

    
    io::stdin() // this is the way to receive user in
    .read_line(&mut guess)
    .expect("Failed to read line!");

    println!("You guessed: {}",guess);
    let guess: u32  = guess.trim().parse() {

    }
    

    loop {
        println!("Please enter your guess below:\n-->");

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
