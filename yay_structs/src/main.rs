

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}



fn main() {

    let user1 = User {
        active: true,
        username: String::from("testuser123"),
        email: String::from("testemail@yupyup.com"),
        sign_in_count: 1,
    };

    


}
