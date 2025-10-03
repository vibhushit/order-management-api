# Order Management API

RESTful API for managing customer orders built with Rust and Actix-web.

## Tech Stack
- **Language**: Rust
- **Framework**: Actix-web
- **Storage**: In-memory (HashMap with `Arc<RwLock>`)

## Project Status
Under Development

## Setup
```bash
cargo build
cargo run
```

## API Endpoints
- `POST /orders` - Create new order
- `GET /orders/{id}` - Retrieve order by ID
- `GET /orders` - List all orders
- `PUT /orders/{id}` - Update order status
- `DELETE /orders/{id}` - Delete order

## Testing

```bash
cargo test
```
