use log::{error, info, warn, debug};
use log4rs;

/// ## Event logger
///
/// Used to log the following events:
/// - sending/reading a message to user that exists
/// - creation of new user
/// - reading message with the wrong password
/// - reading message for a user that does not exist
/// - sending message to user that does not exist
/// - other important events that should be noted
///
/// # Arguments
///
/// * `category` - either "info", "warn", or "error"
/// * `command` - holds the logging message (and relevant context)
///
/// Returns Result<(), &str>
///
/// # Examples
///
/// ```
/// use deaddrop_rs::logging::log_event;
///
/// log_event("info", "something happened");
/// 
/// log_event("error", "there was a problem here");
/// 
/// log_event("warn", "this could be an issue");
/// ```
pub fn log_event(category: &str, event: String) {   
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or_else(|error| 
        println!("Error encountered when initializing logger: {}", error) );

    match category {
        "info" => info!("{event}"),
        "warn" => warn!("{event}"),
        "error" => error!("{event}"),
        _ => debug!("invalid category entered when trying to log: {category}, {event}")
    }
}
