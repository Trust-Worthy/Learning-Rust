//! Finance Tracker in Rust
//! 
//! 
//! This module handles expense tracking.




fn welcome_message() {


    println!("\nWelcome to the rust finance tracker.");
    println!("Please select an option below to get started.");
}




fn main() {
    

    welcome_message();

    loop {
        println!("\n1.Add a new transaction");
        println!("2. View transactions and summary within a date range");
        println!("3. Exit");
    }


}
