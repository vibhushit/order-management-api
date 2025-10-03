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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_creation() {
        let order = Order::new(
            "Test Customer".to_string(),
            "Item A, Item B".to_string(),
            250.0,
        );

        assert_eq!(order.customer_name, "Test Customer");
        assert_eq!(order.items, "Item A, Item B");
        assert_eq!(order.total_amount, 250.0);
        assert_eq!(order.status, OrderStatus::Pending);
    }

    #[test]
    fn test_order_status_update() {
        let mut order = Order::new("Customer".to_string(), "Product".to_string(), 100.0);
        let initial_time = order.updated_at;

        // Small delay to ensure timestamp difference
        std::thread::sleep(std::time::Duration::from_millis(10));

        order.update_status(OrderStatus::Completed);

        assert_eq!(order.status, OrderStatus::Completed);
        assert!(order.updated_at > initial_time);
    }
}
