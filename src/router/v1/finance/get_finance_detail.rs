use axum::extract::Path;
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

fn conn() -> Result<Connection, sqlite::Error> {
    sqlite::open(DB_PATH)
}

#[derive(Serialize)]
pub struct GetDetailResponse {
    message: String,
    data: Option<Finance>,
}
pub async fn main(Path(id): Path<String>) -> axum::Json<GetDetailResponse> {
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
