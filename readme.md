# Rust Server

A simple Rust API server using `Axum` with `sqlite` as database

### Running the Server

```bash
cargo run
```

The server will start on `http://localhost:8080`.

---

## API Documentation

### Base URL

```
http://localhost:8080/v1/finance
```

### Endpoints

#### `GET /v1/finance`

- **Description:** Retrieve finance data.
- **Response:**
  - `200 OK` – Returns a JSON array of finance records.

```json
{
  "message": "Success",
  "data": [
    {
      "id": 1,
      "reason": "salary",
      "variant": "in",
      "amount": 10000,
      "created_at": "2025-05-30 08:59:34"
    },
    {
      "id": 2,
      "reason": "buy thing",
      "variant": "out",
      "amount": 1000,
      "created_at": "2025-05-30 09:02:50"
    },
    {
      "id": 3,
      "reason": "buy thing again",
      "variant": "out",
      "amount": 10000,
      "created_at": "2025-05-30 09:04:56"
    }
  ]
}
```

#### `POST /v1/finance`

- **Description:** Create a new finance record.
- **Request Body:**
  ```ts
  interface Body {
    variant: "in" | "out";
    reason: string;
    amount: number;
  }
  ```
- **Response:**
  - `201 Created` – Returns the created record.

#### `GET /v1/finance/{id}`

- **Description:** Retrieve a specific finance record by ID.
- **Response:**
  - `200 OK` – Returns the finance record.

```json
{
  "message": "Success",
  "data": {
    "id": 1,
    "reason": "salary",
    "variant": "in",
    "amount": 10000,
    "created_at": "2025-05-30 08:59:34"
  }
}
```

---
