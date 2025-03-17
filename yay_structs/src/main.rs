

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email:String, username:String) -> User {

    User {
        active:true,
        username,
        email,
        sign_in_count: 1,
    }
}

// tuple Structs!!!
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

// Foreshadowing traits and the usefulness of unit tuple structs
struct AlwaysEqual;

fn main() {

    let mut user1 = User { // entire instance must be mutable. Specific fields can't just be mutable
        active: true,
        username: String::from("testuser123"),
        email: String::from("testemail@yupyup.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("newemail@yupyup.com");

    // update syntax for creating a new user

    let user2 = User {
        active : user1.active,
        username : user1.username,
        email: String::from("anotherone@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };
    // CAN NO LONGER USER user 1 after lines 33-38 because the values were MOVED not borrowed of copied!!!!

    // shortcut for update syntax
    let user3 = User {
        email: String::from("whatupwhatup@yahooo.com"),
        ..user2
    };

    let built_user = build_user(String::from("hiii@gmail.com"), String::from("yo-mamaa"));


    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let subject = AlwaysEqual;

}
