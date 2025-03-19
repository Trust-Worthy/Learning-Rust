


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

    match third_ele {
        Some(ele) => println!("Third element in the vector is {ele}"),
        None => println!("Third element in the vector is NUTHIN"),
    }




}
