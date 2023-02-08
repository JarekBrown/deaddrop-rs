use rusqlite::{Connection};
use std::{fs, path::Path};
use crate::logging::log_event;

pub fn connect() -> Connection {
    let mut must_initialize_db = false;
    if !Path::new("dd.db").exists() {
        must_initialize_db = true;
    }

    let connection = Connection::open("dd.db").unwrap();
    
    if must_initialize_db {
        let query = fs::read_to_string("init.sql").expect("initial schema does not exist");
        let commands = query.split(";\n");

        for command in commands {
            connection.execute(command, ()).unwrap();
        }
        log_event("info", format!("database created"));
    }
    
    return connection;
}