use crate::{session, db::users, logging::log_event};
use std::io::{self, BufRead};

pub fn new_user(user: String) {
    let user_exists = match users::get_user(user.clone()) {
        Some(_) => true,
        None => false,
    };

    if !users::no_users() && !user_exists {
        log_event("error", format!("(new) user not found: {}", {user}));
        panic!("User not recognized");
    }

    if !session::authenticate(user.clone()).expect("Unable to authenticate user") {
        log_event("warn", format!("(new) unable to authenticate: {}", user));
        panic!("Unable to authenticate user");
    }

    println!("Username: ");
    let new_user = get_new_username();
    let new_pass_hash = session::get_password();

    users::set_user_pass_hash(new_user, new_pass_hash);

    log_event("info", format!("user created: {}", user));
}

fn get_new_username() -> String {
    io::stdin().lock().lines().next().expect("there was no next line").expect("the line could not be read")
}