use std::io;

fn main() {
    
    println!("Welcome to the guessing game...");
    println!("You're going to enter a few numbers and we gon see how nice you are at guessing.");
    println!("Please enter your guess below:\n-->");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");


    println!("You guessed: {}",guess);
}
