use std::vec;

mod string;


#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    
    // Vectors can only store values of the same type

    let mut v: Vec<i32> = Vec::new();

    let macro_vec = vec![1,2,3];


    v.push(1);
    v.push(2);
    v.push(3);
    v.push(5);


    let third_ele: &i32 = &v[2]; // getting a reference to an i32 / 3rd element in the vector

    println!("Third element in the vector is {third_ele}");

    let third_ele: Option<&i32> = v.get(2); // getting the element at the 2nd index in a different way

    // using the .get method on a vector is really helpful because it's more SAFE. it returns an Option<&T> that I can use a match statement with
    // to validate the item

    match third_ele {
        Some(ele) => println!("Third element in the vector is {ele}"),
        None => println!("Third element in the vector is NUTHIN"),
    }

    // let does_not_exist = &macro_vec[100];
    let does_not_exist = macro_vec.get(100); // this method won't panick. It will just return NONE. which is super freaking helpful


    // REMEMBER OWNERSHIP principles 
    let mut vecky = vec![1,2,3,4];

    let first = &vecky[0];

    // Can't have bot a mutable reference and immutable reference in the same scope!
    // so this has to go first!
    println!("The first element is: {first}");

    vecky.push(5);

    for i in &vecky { // using immutable references
        println!("item = {i}")
    }
    
    
    // The reference to the vector that the for loop holds prevents simultaneous modification of the whole vector.

    for i in &mut vecky { // using mutable references
        *i += 50; // get me the value at i / dereference i and add 50 to it
    }



    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    // using enums is super helpful because everything inside of them is technically the same type so we can use 
    // vecs to store a bunch of them together in memory

    for i in &row {
        println!(" item in the spreadsheet is {:?}",*i);
    }



    string::string_practice();





}
