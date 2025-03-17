

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

fn main() {

    let mut user1 = User { // entire instance must be mutable. Specific fields can't just be mutable
        active: true,
        username: String::from("testuser123"),
        email: String::from("testemail@yupyup.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("newemail@yupyup.com");




}
