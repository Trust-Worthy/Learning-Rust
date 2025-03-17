
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

    fn width(&self) -> bool { // Usually, methods that have the same name as struct fields are getters just like in other languages. 
        // This way the fields can be private and the method acts as the interface / api.
        self.width > 0
    }

    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function that's not a method 
    pub fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
    
}

fn main() {
    // will take the width and height of a rectangle specified in pixels and calculate the area of the rectangle


    let rect1: Rectangle = Rectangle { width: 30, height: 50 };

    let rect2: Rectangle = Rectangle { width: 10, height: 20 };

    let sq: Rectangle = Rectangle::square(23); // Associated function requires the ::!
}

