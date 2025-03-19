
use std::{collections::HashMap, hash::Hash};

mod string;


#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn string_fun() {

    string::string_practice();
}

fn vec_fun() {
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
}

fn main() {

    // just like vectors hashmaps store their data on the heap
    // all of the keys of a hashmap must have the same type

    
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("blue team"), 10);
    scores.insert(String::from("yellow team"), 34);

    let team_name = String::from("blue team");

    let blue_score = scores.get(&team_name).copied().unwrap_or(0);
    // get method returns an Option<&V>

    // above 
    // 1. calls get method on hashmap
    // 2. calls copied method on the returned value Option<&V> to get Option<i32> rather than Option<&i32> 
    // 3. then unwrap_or to set score to zero if scores doesn’t have an entry for the key.

    

    // Ownership and hashmaps
    // for types like i32 that have the copy trait, they're copied into the hashmap
    // for types like String. the values are MOVED and the hashmpa now OWNS them!




    let field_name = String::from("First");
    let field_value = String::from("val");

    let mut hashy:HashMap<String,String> = HashMap::new();


    hashy.insert(field_name, field_value); // instead insert references but then you get into dealing with lifetimes! How long will a reference last for?
    // println!("name = {field_name} value = {field_value}"); // this is invalid. the hashmap has ownership


    scores.insert(String::from("blue team"), 23);


    // checking if an entry already exists in a map
    scores.entry(String::from("blue team")).or_insert(0);
    scores.entry(String::from("red team")).or_insert(0);



    for (key,value) in &scores {
        println!("{key}: {value}");
    }




}
