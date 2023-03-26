use std::io::{self, BufRead};

use log::{error, info, warn};

use crate::{
    db::{messages, users},
    encryption, session,
};

pub fn send_message(recipient: String) {
    let sender = verify_sender();

    let recipient_exists = users::get_user(recipient.clone()).is_some();

    if !recipient_exists {
        error!("recipient not found: {}", recipient);
        panic!("Recipeint not recognized");
    }

    let message = get_user_message();

    let message = encryption::encrypt(message, sender);

    messages::save_message(message, recipient.clone());

    info!("message sent to: {}", recipient);
}

fn get_user_message() -> String {
    println!("Enter your message: ");
    io::stdin()
        .lock()
        .lines()
        .next()
        .expect("there was no next line")
        .expect("the line could not be read")
}

fn verify_sender() -> String {
    //! authenticates the sender
    println!("Enter your username: ");

    let sender = io::stdin()
        .lock()
        .lines()
        .next()
        .expect("there was no next line")
        .expect("the line could not be read");

    let sender_exists = users::get_user(sender.clone()).is_some();

    if !sender_exists {
        error!("sender not found: {}", sender);
        panic!("Sender user not recognized");
    } else if !session::authenticate(sender.clone()).expect("Unable to authenticate user") {
        warn!("unable to authenticate sender: {}", sender);
        panic!("Unable to authenticate sender");
    } else {
        sender
    }
}
