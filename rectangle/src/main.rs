
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // will take the width and height of a rectangle specified in pixels and calculate the area of the rectangle


    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Refactoring with tuples
    let rect1 = (30,50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    // Refactoring with structs
    let struct_rect1 = Rectangle{width:30,height:50};

    
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&struct_rect1)
    );
    println!("{:#?}",struct_rect1);
    
    let scale = 2;
    let debug_rect: Rectangle = Rectangle { width: dbg!(30 * scale), height: 50 };
    dbg!(&debug_rect);

}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn area_tuple(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}
 
fn area(width: u32, height:u32) -> u32 {
    width * height
}