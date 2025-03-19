use std::fs::{self, File};
use std::io::{self, Read};

fn panic_fun() {

    panic!("crash and burn");
    
    let v = vec![1,2,3];
    
    v[99]; // this will panic
}

fn reading_file() {


    let greeting_file_result = File::open("hello.txt"); // return type here is a Result<T,E>

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating the file: {e:?}"),
            },
            other_error => {
                panic!("Bruh still not working for some reason {other_error:?}");
            }
        },
    };


    let better = File::open("huh.txt").unwrap(); // unwrap is exactly like the match but it panics automatically

    let informed = File::open("sadfasd.txt").expect("Custom error message that unwrap doesn't provide");

    // in PRODUCTION code .expect() >>>>> .unwrap() because of the custom error messages.

}


fn read_username_from_file() -> Result<String, io::Error> {


    let username_file_result = File::open("nope.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) { // reads the contents of the file into username!
 
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }

    // propogating erros is so common that the ? operator is used to help out.



}

fn clean_read_username_from_file() -> Result<String, io::Error> {

    // using the question mark to propogate errors if an error occurs!
    let mut username_file = File::open("bruh.txt")?;

    let mut username = String::new();

    username_file.read_to_string(&mut username)?;

    Ok(username)
}

fn super_clean_read_username_from_file() -> Result<String, io::Error> {

    let mut username = String::new();

    File::open("asdfas.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn native_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt") // built in method 
    
}

fn main() {
    
    // There are two types of errors in this rusty world
    // 1. Recoverable errors --> usually report the bug to the user and retry the operation
    // 2. Unrecoverable errors --> stop / crash the program because it usually refers to a bug
    // there are no exceptions in rust! instead Result<T, E> is used for recoverable
    // and the macro panic! is used for those unrecoverable ones


    

    // propogating the error is when you pass the error to the code that's calling 




    // The return type of the function has to be compatible with what ? may return if something fails / there's an error

    // let get_file = File::open("ayyy.txt")?; // function has to return Result<T, E> (OK & Err) or Option (Same & None)



}
