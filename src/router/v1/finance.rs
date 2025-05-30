use axum::extract::{Json, Path};
use axum::routing::post;
use axum::{Router, routing::get};
use serde::Deserialize;
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
struct GetResponse {
    message: Option<String>,
    data: Option<Vec<Finance>>,
}

fn conn() -> Result<Connection, sqlite::Error> {
    sqlite::open(DB_PATH)
}

async fn get_finance() -> axum::Json<GetResponse> {
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

#[derive(Deserialize)]
struct NewFinance {
    variant: String,
    amount: i64,
    reason: String,
}

#[derive(Serialize)]
struct InsertResponse {
    message: Option<String>,
    id: Option<i64>,
}

async fn insert_finance(Json(payload): Json<NewFinance>) -> axum::Json<InsertResponse> {
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

#[derive(Serialize)]
struct GetDetailResponse {
    message: String,
    data: Option<Finance>,
}
async fn get_finance_detail(Path(id): Path<String>) -> axum::Json<GetDetailResponse> {
    // async fn get_data() {
    let mut data = Vec::new();
    let sql = match conn() {
        Ok(conn) => conn,
        Err(e) => {
            return axum::Json(GetDetailResponse {
                message: format!("Database connection error: {}", e),
                data: None,
            });
        }
    };
    let query = "SELECT * FROM finance WHERE id=?";

    let mut statement = match sql.prepare(query) {
        Ok(stmt) => stmt,
        Err(e) => {
            return axum::Json(GetDetailResponse {
                message: format!("Failed to prepare statement: {}", e),
                data: None,
            });
        }
    };

    if let Err(e) = statement.bind((1, id.as_str())) {
        return axum::Json(GetDetailResponse {
            message: format!("Failed to bind reason: {}", e),
            data: None,
        });
    }

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

    let first = data.first();

    let response = match first {
        Some(finance) => axum::Json(GetDetailResponse {
            message: "Success".to_string(),
            data: Some(Finance {
                id: finance.id,
                reason: finance.reason.clone(),
                variant: finance.variant.clone(),
                amount: finance.amount,
                created_at: finance.created_at.clone(),
            }),
        }),
        None => axum::Json(GetDetailResponse {
            message: "Finance not found".to_string(),
            data: None,
        }),
    };

    response
}

pub fn main() -> Router {
    let router = Router::new()
        .route("/v1/finance", get(get_finance))
        .route("/v1/finance/{id}", get(get_finance_detail))
        .route("/v1/finance", post(insert_finance));
    router
}
