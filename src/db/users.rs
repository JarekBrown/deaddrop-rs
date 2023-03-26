use super::db::connect;

pub fn get_user(user: String) -> Option<i64> {
    let db = connect();

    let query = "SELECT id FROM Users WHERE user = :user ;";
    let mut stmt = db.prepare(query).expect("expected to prepare statement");
    let mut rows = stmt
        .query(&[(":user", &user)])
        .expect("expected query to execute");

    if let Some(row) = rows.next().expect("expected to find something in results") {
        Some(row.get(0).expect("expected there to be a value in the row"))
    } else {
        None
    }
}

pub fn get_user_pass_hash(user: String) -> String {
    let db = connect();

    let query = "SELECT hash FROM Users WHERE user = :user ;";
    let mut stmt = db
        .prepare(query)
        .expect("statement should prepare correctly");
    let mut rows = stmt
        .query(&[(":user", &user)])
        .expect("expected query to execute");

    if let Some(row) = rows.next().expect("expected to find something in results") {
        row.get(0).expect("expected to find a value in the table")
    } else {
        panic!("{:?}", rusqlite::Error::QueryReturnedNoRows);
    }
}

pub fn set_user_pass_hash(user: String, hash: String) {
    let db = connect();

    let query = "INSERT INTO Users (user, hash) VALUES (:user, :hash);";
    let mut stmt = db
        .prepare(query)
        .expect("expected to prepare statement correctly");
    stmt.execute(&[(":user", &user), (":hash", &hash)])
        .expect("expected query to execute");
}

// returns true if there are no registered users and false otherwise
pub fn no_users() -> bool {
    let db = connect();

    let query = "SELECT COUNT(*) FROM Users;";
    let mut stmt = db
        .prepare(query)
        .expect("expected to prepare statement correctly");
    let mut rows = stmt.query([]).expect("expected query to execute");

    if let Some(row) = rows.next().expect("expected to find something in results") {
        row.get::<usize, usize>(0)
            .expect("expected to find a value in the table")
            == 0
    } else {
        panic!("{:?}", rusqlite::Error::QueryReturnedNoRows);
    }
}
