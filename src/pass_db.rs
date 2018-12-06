fn testing() -> Result<(), Box<std::error::Error>> {
    let conn = rusqlite::Connection::open("pwdb.sqlite").expect("failed to open connection");

    conn.execute("DROP TABLE IF EXISTS credentials", rusqlite::NO_PARAMS)
        .expect("failed to drop table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS credentials (
        id          ROWID INTEGER PRIMARY KEY,
        site        TEXT NOT NULL,
        username    TEXT NOT NULL,
        password    TEXT NOT NULL
    )",
        rusqlite::NO_PARAMS,
    ).expect("failed to create table.");

    conn.execute(
        "INSERT INTO credentials (site, username, password) VALUES ('facebook', 'andysalerno', 'password123')",
        rusqlite::NO_PARAMS,
    ).expect("insert failed");

    let my_row: String = conn
        .query_row("SELECT * FROM credentials", rusqlite::NO_PARAMS, |row| {
            row.get(1)
        }).expect("failed to run query");

    println!("{}", my_row);

    Ok(())
}

pub trait PasswordDb {
    fn find_for_site(&self, site: &str) -> String;
    fn insert_for_site(&mut self, site: &str, username: &str, password: &str) -> Result<(), ()>;
}

pub struct SqlitePasswordDb {
    conn: rusqlite::Connection,
}

impl SqlitePasswordDb {
    pub fn new() -> Self {
        let conn = rusqlite::Connection::open("pwdb.sqlite").expect("failed to open connection");
        let db = SqlitePasswordDb { conn };
        db
    }
}

impl PasswordDb for SqlitePasswordDb {
    fn find_for_site(&self, site: &str) -> String {
        return "my_pwd_123".to_owned();
    }

    fn insert_for_site(&mut self, site: &str, username: &str, password: &str) -> Result<(), ()> {
        Ok(())
    }
}

pub struct TestingPasswordDb;
impl PasswordDb for TestingPasswordDb {
    fn find_for_site(&self, site: &str) -> String {
        return "my_pwd_123".to_owned();
    }

    fn insert_for_site(&mut self, site: &str, username: &str, password: &str) -> Result<(), ()> {
        Ok(())
    }
}

pub fn open_db(path: &str) -> impl PasswordDb {
    TestingPasswordDb {}
}
