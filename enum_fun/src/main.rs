


enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x:i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self) {
        // method body def here
    }


}

fn main() {
    // Enums give you a way of saying a value is one of a possible set of values


    // let four: IpAddrKind = IpAddrKind::V4;
    // let six : IpAddrKind = IpAddrKind::V6;
    
    // You can put actual data inside the instance of an enum
    // I can attatch data to each variant instead of making a whol ip address class
    
    // let home = IpAddrKind::V4(String::from("127.0.0.1"));
    // let loopback = IpAddrKind::V6(String::from("::1"));

    // Even better way?
    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));


    let aye = Message::Write(String::from("we love you bro"));

    aye.call();

    // Examples using the Option values
    let some_number= Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None; // I mean of absent number to be of type i32

    // Rust doesn't treat Option enums as values that operations can be performed on

    let x: i8 = 5;
    let y: Option<i8> =  Some(5);

    // Doesn't work because I have to convert an Option<T> to a T before I can do operations with T.
    //let sum = x + y;

    // Rust basically gives me the safety to assume that a value is not null when it isn't an Option<T>.






}
