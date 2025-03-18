#![allow(dead_code,unused_variables)]

pub struct Credentials {
    username: String,
    password: String,
}

enum Status {
    Connected,
    Interrupted,
}

fn connect_to_database() -> Status {
    return Status::Connected;
}

fn login(creds: Credentials) {
    get_user();
}

fn logout() {
    // log user out...
}

fn get_user() {
    // get user from database
}

// We want to expose this function
pub fn authenticate(creds: Credentials) {

    if let Status::Connected = connect_to_database() {
        login(creds);
    }

}