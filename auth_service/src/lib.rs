#![allow(dead_code,unused_variables)]

struct Credentials {
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

fn get_user() {
    // get user from database
}

fn authenticate(creds: Credentials) {

    if let Status::Connected = connect_to_database() {
        login(creds);
    }

}