# Order Management API

RESTful API for managing customer orders built with Rust and Actix-web.

## Tech Stack

- Language: Rust
- Framework: Actix-web
- Storage: In-memory HashMap with Arc<RwLock>
- Error Handling: anyhow

## Setup

```bash
cargo build
cargo run
```

Server starts at http://127.0.0.1:8080

## API Endpoints

### Create Order
POST /orders

Request:

```
{
  "customer_name": "John",
  "items": "Laptop, Mouse",
  "total_amount": 85000.00
}
```

### Get Order by ID
GET /orders/{id}

### List All Orders
GET /orders

### Update Order Status
PUT /orders/{id}

Request:
```
{
  "status": "completed"
}
```

### Delete Order
DELETE /orders/{id}

## Testing

```bash
cargo test
```

Includes both unit and integration tests.

## Example Usage

```
# Create order
curl -X POST http://127.0.0.1:8080/orders \
  -H "Content-Type: application/json" \
  -d '{"customer_name":"John","items":"Phone","total_amount":65000.00}'

# Get order
curl http://127.0.0.1:8080/orders/{id}

# List orders
curl http://127.0.0.1:8080/orders

# Update status
curl -X PUT http://127.0.0.1:8080/orders/{id} \
  -H "Content-Type: application/json" \
  -d '{"status":"processing"}'

# Delete order
curl -X DELETE http://127.0.0.1:8080/orders/{id}
```

## Design Decisions

- In-memory HashMap for simplicity
- UUID for unique order IDs
- Thread-safe operations with `Arc<RwLock>`
- Modular structure for maintainability
