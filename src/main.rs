extern crate rusqlite;
extern crate sodiumoxide;

mod pass_db;
mod simple_sql;

use sodiumoxide::crypto::secretbox;

fn main() -> Result<(), Box<std::error::Error>> {
    sodiumoxide::init().expect("didn't init properly");
    Ok(())
}

fn encrypt() {
    let key = secretbox::gen_key();
    let nonce = secretbox::gen_nonce();
    println!("{:?}", nonce);
    println!("{:?}", key);
    let cipher_text = secretbox::seal(b"hello, everyone!", &nonce, &key);
    let decrypted = secretbox::open(&cipher_text, &nonce, &key).unwrap();
    println!("{}", String::from_utf8(decrypted).unwrap());
}

#[cfg(test)]
mod tests {
    use pass_db;
    use pass_db::PasswordDb;

    #[test]
    fn read_scenario() {
        // decrypt the db file
        const path: &str = "pwdb.sqlite.crypt";
        const decrypt_key: &str = "test123";
        const decrypted_path: &str = "pwdb.sqlite";
        // crypto::decrypt_file(path, decrypt_key, decrypted_path);

        // query the decrypted db for the desired site
        const site: &str = "friendster.com";
        let password_db = pass_db::open_db(decrypted_path);
        let user_credentials = password_db.find_for_site(site);
    }

    #[test]
    fn write_scenario() {
        // decrypt the db file
        const path: &str = "pwdb.sqlite.crypt";
        const decrypt_key: &str = "test123";
        const decrypted_path: &str = "pwdb.sqlite";
        // crypto::decrypt_file(path, decrypt_key, decrypted_path);

        // query the decrypted db for the desired site
        const site: &str = "friendster.com";
        let mut password_db = pass_db::open_db(decrypted_path);
        let result = password_db.insert_for_site(site, "username", "pw");
        result.expect("insert failed");
    }
}
