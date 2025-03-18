#![allow(dead_code,unused_variables)]



mod database {

    pub enum Status {
        Connected,
        Interrupted,
    }

    pub fn connect_to_database() -> Status {
        return Status::Connected;
    }

    pub fn get_user() {
        // get user from database
    }

}

mod auth_utils {

    pub fn login(creds: models::Credentials) { // relative path to get to the Creds struct
        crate::database::get_user(); // absolute path to function inside database module
    }
    
    fn logout() {
        // log user out...
    }

    pub mod models {
        pub struct Credentials {
            username: String,
            password: String,
        }
    }

}

// We want to expose this function
pub fn authenticate(creds: auth_utils::models::Credentials) {

    if let database::Status::Connected = database::connect_to_database() { //
        auth_utils::login(creds);
    }

}