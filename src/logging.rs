use log4rs;

/// ## Event logger
///
/// Initializes the log4rs logging environment, configured by log4rs.yaml file
///
/// Uses macros provided by log crate
///
/// # Examples
///
/// ```
/// use deaddrop_rs::logging::log_event;
///
/// info!("something happend");
///
/// error!("there was a problem here");
///
/// warn!("there could be an issue here");
/// ```
pub fn log_init() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or_else(|error| {
        println!(
            "ERROR: problem encountered when initializing logger -- {}",
            error
        )
    });
}
