#![allow(dead_code,unused_variables)]


// tells rust that crate module has a sub-module called database
// because contents of batabase module aren't defined in-line, rust automatically looks elsewhere in the src directory
mod database; // simple. all contents of database module just happen to be in another file
// now learning about modules with sub-modules
mod auth_utils;

// 1. auth utils is a sub module of crate 
// 2. contents of auth utils aren't def in-line so rust looks for a file called auth_utlis.rs
// 3. no file named auth_utils exists so rust looks for a mod.rs file inside of a folder called auth_utils


    


// rust modules don't have a 1-1 mapping to the file system like other languages... AND I LOVE IT

// Modules were introduced into rust to allow for very specific conditional compilation



// use keywords allow for shortcuts so I don't have to type the whole path!
use auth_utils::models::Credentials; // bringing credentials into SCOPE --> use keyword means referencing full-qualified symbols defined somewhere in module tree
use database::Status; // Bringing the Status Enum INTO SCOPE (this is the proper language to use )


// We want to expose this function
pub fn authenticate(creds: Credentials) { // relative path

    if let Status::Connected = database::connect_to_database() { //
        auth_utils::login(creds);
    }

}