use axum::extract::Json;
use serde::Deserialize;
use serde::Serialize;
use sqlite::Connection;
use sqlite::State;

const DB_PATH: &str = "src/db/database.sqlite";

fn conn() -> Result<Connection, sqlite::Error> {
    sqlite::open(DB_PATH)
}

#[derive(Deserialize)]
pub struct NewFinance {
    variant: String,
    amount: i64,
    reason: String,
}

#[derive(Serialize)]
pub struct InsertResponse {
    message: Option<String>,
    id: Option<i64>,
}

pub async fn main(Json(payload): Json<NewFinance>) -> axum::Json<InsertResponse> {
    let sql = match conn() {
        Ok(c) => c,
        Err(e) => {
            return axum::Json(InsertResponse {
                message: Some(format!("Database connection error: {}", e)),
                id: None,
            });
        }
    };

    let query = "INSERT INTO finance (reason, variant, amount) VALUES (?, ?, ?)";
    let mut statement = match sql.prepare(query) {
        Ok(stmt) => stmt,
        Err(e) => {
            return axum::Json(InsertResponse {
                message: Some(format!("Failed to prepare statement: {}", e)),
                id: None,
            });
        }
    };

    if let Err(e) = statement.bind((1, payload.reason.as_str())) {
        return axum::Json(InsertResponse {
            message: Some(format!("Failed to bind reason: {}", e)),
            id: None,
        });
    }
    if let Err(e) = statement.bind((2, payload.variant.as_str())) {
        return axum::Json(InsertResponse {
            message: Some(format!("Failed to bind variant: {}", e)),
            id: None,
        });
    }
    if let Err(e) = statement.bind((3, payload.amount)) {
        return axum::Json(InsertResponse {
            message: Some(format!("Failed to bind amount: {}", e)),
            id: None,
        });
    }

    match statement.next() {
        Ok(State::Done) => {
            // Query the last inserted row id
            let mut id: i64 = 0;
            let mut stmt = match sql.prepare("SELECT last_insert_rowid()") {
                Ok(s) => s,
                Err(e) => {
                    return axum::Json(InsertResponse {
                        message: Some(format!("Success but can't get id : {}", e)),
                        id: None,
                    });
                }
            };
            if let Ok(State::Row) = stmt.next() {
                id = stmt.read::<i64, _>(0).unwrap_or_default();
            }
            axum::Json(InsertResponse {
                message: Some("Success inserting data".to_string()),
                id: Some(id),
            })
        }
        Err(e) => axum::Json(InsertResponse {
            message: Some(format!("Failed to insert data: {}", e)),
            id: None,
        }),
        _ => axum::Json(InsertResponse {
            message: Some("Unknown error".to_string()),
            id: None,
        }),
    }
}
