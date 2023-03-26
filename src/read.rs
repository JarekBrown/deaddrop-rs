use crate::{
    db::{messages, users},
    encryption, session,
};
use log::{error, info, warn};

pub fn read_messages(user: String) {
    let user_exists = users::get_user(user.clone()).is_some();

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
        let dec_message = encryption::decrypt(message.clone()).unwrap_or_else(|e| {
            error!("{} for {}", &e, &user);
            format!("ERROR: {}", e)
        });
        println!("{:?}", dec_message);
    }
    info!("messages read: {}", user);
}
