
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Methods can 
    // 1. Take ownership of self
    // 2. Borrow self immutably as is done above --> to read data 
    // 3. Or, Borrow self mutably
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    
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
        "The area of the STRUCT rectangle is {} square pixels.",
        struct_rect1.area()
    );
    println!("{:#?}",struct_rect1);

    if struct_rect1.width() {
        println!("AYYY you a big recty!");
    }
    
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