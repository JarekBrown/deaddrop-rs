use crate::db;

pub fn get_password() -> String {
    hash(read_pass())
}

pub fn authenticate(user: String) -> Result<bool, bcrypt::BcryptError> {
    // bypass auth if no users currently exist
    if db::users::no_users() {
        return Ok(true);
    }

    let pass = read_pass();
    let hash_opt = db::users::get_user_pass_hash(user);

    match hash_opt {
        hash => bcrypt::verify(pass, &hash[..]),
    }
}

fn hash(pass: String) -> String {
    let hash = bcrypt::hash(pass, bcrypt::DEFAULT_COST);
    match hash {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    }
}

fn read_pass() -> String {
    rpassword::prompt_password("Password: ").unwrap()
}
