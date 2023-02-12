use log::info;
use rusqlite::{Connection};
use std::{fs, path::Path, env};

pub fn connect() -> Connection {
    let mut must_initialize_db = false;
    if !Path::new("dd.db").exists() {
        must_initialize_db = true;
    }

    let connection = Connection::open("dd.db").unwrap();
    
    if must_initialize_db {
        // in some cases, wrong line endings for an OS can cause only the first table to be created
        let line_ending = match env::consts::OS {
            "windows" => ";\r\n\r\n",
            _ => ";\n"
        };
        let query = fs::read_to_string("init.sql").expect("initial schema does not exist");
        let commands = query.split(line_ending);

        for command in commands {
            connection.execute(command, ()).unwrap();
        }
        info!("database created");
    }
    
    return connection;
}