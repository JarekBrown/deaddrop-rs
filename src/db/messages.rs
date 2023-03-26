use super::db::connect;

pub fn get_messages_for_user(user: String) -> Vec<String> {
    let db = connect();

    let query =
        "SELECT (data) FROM Messages WHERE recipient = (SELECT id FROM Users WHERE user = :user);";
    let mut stmt = db.prepare(query).expect("expected to prepare query");
    let mut rows = stmt
        .query(&[(":user", &user)])
        .expect("expected query to succeed");

    let mut messages = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        messages.push(row.get(0).expect("expected a value in the row"));
    }
    messages
}

pub fn save_message(message: String, recipient: String) {
    let db = connect();

    let query = "INSERT INTO Messages (recipient, data) VALUES ((SELECT id FROM Users WHERE user = :recipient), :message);";
    let mut stmt = db
        .prepare(query)
        .expect("expected to prepare statement correctly");
    stmt.execute(&[(":recipient", &recipient), (":message", &message)])
        .expect("expected query to execute");
}
