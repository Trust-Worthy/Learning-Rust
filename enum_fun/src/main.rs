


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




}
