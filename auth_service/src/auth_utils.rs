

// This approach allows contents of auth utils to defined in auth utils rust file
// NOW all of auth_utils.rs sub-modules are defined in a folder called auth_utils


pub fn login(creds: models::Credentials) { // relative path to get to the Creds struct
    crate::database::get_user(); // absolute path to function inside database module
}

fn logout() {
    // log user out...
}

pub mod models; // models is a sub-module of auth_utils 
// 1. contents of models modules aren't defined in-line
// 2. rust looks for a file called models.rs
// 3. models.rs MUST BE located inside of auth_utils folder not src