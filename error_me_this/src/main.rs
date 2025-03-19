use std::{fs::File, io::ErrorKind};

fn panic_fun() {

    panic!("crash and burn");
    
    let v = vec![1,2,3];
    
    v[99]; // this will panic
}


fn main() {
    
    // There are two types of errors in this rusty world
    // 1. Recoverable errors --> usually report the bug to the user and retry the operation
    // 2. Unrecoverable errors --> stop / crash the program because it usually refers to a bug
    // there are no exceptions in rust! instead Result<T, E> is used for recoverable
    // and the macro panic! is used for those unrecoverable ones


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

    







}
