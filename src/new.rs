use crate::{db::users, session};
use log::{error, info, warn};
use std::io::{self, BufRead};

pub fn new_user(user: String) {
    let user_exists = match users::get_user(user.clone()) {
        Some(_) => true,
        None => false,
    };

    if !users::no_users() && !user_exists {
        error!("user not found: {}", user);
        panic!("User not recognized");
    }

    if !session::authenticate(user.clone()).expect("Unable to authenticate user") {
        warn!("unable to authenticate user: {}", user);
        panic!("Unable to authenticate user");
    }

    println!("Username: ");
    let new_user = get_new_username();
    let new_pass_hash = session::get_password();

    users::set_user_pass_hash(new_user.clone(), new_pass_hash);

    info!("user created by {}: {}", user, new_user);
}

fn get_new_username() -> String {
    loop {
        let input = io::stdin()
            .lock()
            .lines()
            .next()
            .expect("there was no next line")
            .expect("the line could not be read");

        let user_exists = match users::get_user(input.clone()) {
            Some(_) => true,
            None => false,
        };

        if user_exists {
            println!("Already taken, please try again:");
        } else {
            break input;
        }
    }
}
