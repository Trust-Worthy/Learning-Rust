



fn main() {
    // will take the width and height of a rectangle specified in pixels and calculate the area of the rectangle


    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}



fn area(width: u32, height:u32) -> u32 {
    width * height
}