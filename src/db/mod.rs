const DB_PATH: &str = "src/db/database.sqlite";

fn migrate() -> Result<String, sqlite::Error> {
    let connection = sqlite::open(DB_PATH).unwrap();
    let query = "
        CREATE TABLE finance (id INTEGER PRIMARY KEY AUTOINCREMENT, reason varchar(255), variant TEXT CHECK(variant in ('in','out')), amount int, created_at DATETIME DEFAULT CURRENT_TIMESTAMP);
        INSERT INTO finance (reason,variant,amount) VALUES ('salary','in',10000);
    ";
    connection.execute(query)?;
    drop(connection);
    Ok("Table finance created".to_string())
}
pub fn main() {
    match migrate() {
        Ok(success) => println!("{}", success),
        Err(e) => eprintln!(
            "Error Occured {}",
            e.message.unwrap_or_else(|| "".to_string())
        ),
    }
}
