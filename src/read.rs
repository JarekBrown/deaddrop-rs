use crate::{session, db::{messages, users}, encryption};
use log::{info, warn, error};

pub fn read_messages(user: String) {
    let user_exists = match users::get_user(user.clone()) {
        Some(_) => true,
        None => false,
    };

    if !user_exists {
        error!("user not found: {}", user);
        panic!("User not recognized");
    }

    if !session::authenticate(user.clone()).expect("Unable to authenticate user") {
        warn!("unable to authenticate: {}", user);
        panic!("Unable to authenticate user");
    }

    let messages = messages::get_messages_for_user(user.clone());
    for message in messages {
        let dec_message = encryption::decrypt(message.clone());
        println!("{:?}", dec_message);
    }
    info!("messages read: {}", user);
}