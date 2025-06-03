use serde::Serialize;
use sqlite::Connection;
use sqlite::State;

const DB_PATH: &str = "src/db/database.sqlite";

#[derive(Serialize)]
struct Finance {
    id: i64,
    reason: String,
    variant: String,
    amount: i64,
    created_at: String,
}

#[derive(Serialize)]
pub struct GetResponse {
    message: Option<String>,
    data: Option<Vec<Finance>>,
}

fn conn() -> Result<Connection, sqlite::Error> {
    sqlite::open(DB_PATH)
}

pub async fn main() -> axum::Json<GetResponse> {
    // async fn get_data() {
    let mut data = Vec::new();
    let sql = match conn() {
        Ok(conn) => conn,
        Err(e) => {
            return axum::Json(GetResponse {
                message: Some(format!("Database connection error: {}", e)),
                data: None,
            });
        }
    };
    let query = "SELECT * FROM finance";

    let mut statement = match sql.prepare(query) {
        Ok(stmt) => stmt,
        Err(e) => {
            return axum::Json(GetResponse {
                message: Some(format!("Failed to prepare statement: {}", e)),
                data: None,
            });
        }
    };

    while let Ok(State::Row) = statement.next() {
        let finance = Finance {
            id: statement.read::<i64, _>(0).unwrap_or_default(),
            reason: statement.read::<String, _>(1).unwrap_or_default(),
            variant: statement.read::<String, _>(2).unwrap_or_default(),
            amount: statement.read::<i64, _>(3).unwrap_or_default(),
            created_at: statement.read::<String, _>(4).unwrap(),
        };
        data.push(finance);
    }

    axum::Json(GetResponse {
        message: Some("Success".to_string()),
        data: Some(data),
    })
}
