

use std::io;

fn initial_exercises() {

    let x:u8 = 5;

    println!("The value of x is: {x}");

    {
        let x: u8 = x * 2;

        println!("The value of x in the inner scope is {x}");
    }

    let x = 6;
    println!("The value of x is: {x} ");    

    // Example of a tuple:
    let tup: (i32,f64,u8) = (500,6.4,1); // I think the type annotations for a tuple are cool
    let (_x,_y,_z) = tup;

    println!("The value of y is {_y}");
    println!("The value of x is {_x}");
    println!("The value of y is {_y}");


    // Arrays in rust have fixed sizes
    let array: [i32; 5] = [4,5,6,7,8];

    let first: i32 = array[0];
    let second: i32 = array[1];

    println!("First is {first}");
    println!("Second is {second}");
}





fn get_array_index() {


    let a: [u32;5] = [1,2,3,4,5];
    println!("Please enter an array index.");

    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element: u32 = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn print_labeled_measurement(value: u32, unit_label: char) {
    println!("The value is {value}{unit_label}");

}

fn expression_ex() {

    let y: u32 = {

        let x: u32 = 5;
        x + 1
    };

    println!("The var y is {y}");
}

fn learning_loops() -> u32 {

    let mut counter: u32 = 0;

    let result: u32 = loop{

        counter += 1;

        if counter == 10 {
            break counter * 2;
        }

    };

    return result
}

fn main() { 
    initial_exercises();
    get_array_index();
    print_labeled_measurement(57, 'F');
    expression_ex();
    

    let x:u32 = learning_loops();

    println!("x is {x}");




    }


