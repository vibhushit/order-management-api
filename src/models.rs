use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Processing,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub customer_name: String,
    pub items: String,
    pub status: OrderStatus,
    pub total_amount: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub customer_name: String,
    pub items: String,
    pub total_amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: OrderStatus,
}

impl Order {
    pub fn new(customer_name: String, items: String, total_amount: f64) -> Self {
        let now = Utc::now();
        Order {
            id: Uuid::new_v4(),
            customer_name,
            items,
            status: OrderStatus::Pending,
            total_amount,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_status(&mut self, status: OrderStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
}
