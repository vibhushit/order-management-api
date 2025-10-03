use crate::models::{Order, OrderStatus};
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Clone)]
pub struct Database {
    orders: Arc<RwLock<HashMap<Uuid, Order>>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            orders: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn create_order(&self, order: Order) -> Result<Order> {
        let mut orders = self.orders.write().map_err(|_| anyhow!("Lock poisoned"))?;
        let order_clone = order.clone();
        orders.insert(order.id, order);
        Ok(order_clone)
    }

    pub fn get_order(&self, id: &Uuid) -> Result<Order> {
        let orders = self.orders.read().map_err(|_| anyhow!("Lock poisoned"))?;
        orders
            .get(id)
            .cloned()
            .ok_or_else(|| anyhow!("Order not found"))
    }

    pub fn list_orders(&self) -> Result<Vec<Order>> {
        let orders = self.orders.read().map_err(|_| anyhow!("Lock poisoned"))?;
        Ok(orders.values().cloned().collect())
    }

    pub fn update_order_status(&self, id: &Uuid, status: OrderStatus) -> Result<Order> {
        let mut orders = self.orders.write().map_err(|_| anyhow!("Lock poisoned"))?;
        let order = orders
            .get_mut(id)
            .ok_or_else(|| anyhow!("Order not found"))?;
        order.update_status(status);
        Ok(order.clone())
    }

    pub fn delete_order(&self, id: &Uuid) -> Result<()> {
        let mut orders = self.orders.write().map_err(|_| anyhow!("Lock poisoned"))?;
        orders
            .remove(id)
            .ok_or_else(|| anyhow!("Order not found"))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Order;

    #[test]
    fn test_create_and_get_order() {
        let db = Database::new();
        let order = Order::new("Vibhushit".to_string(), "Laptop".to_string(), 1500.0);
        let order_id = order.id;

        let created = db.create_order(order).unwrap();
        assert_eq!(created.customer_name, "Vibhushit");

        let retrieved = db.get_order(&order_id).unwrap();
        assert_eq!(retrieved.id, order_id);
        assert_eq!(retrieved.customer_name, "Vibhushit");
    }

    #[test]
    fn test_list_orders() {
        let db = Database::new();
        let order1 = Order::new("John".to_string(), "Phone".to_string(), 800.0);
        let order2 = Order::new("Wick".to_string(), "Tablet".to_string(), 600.0);

        db.create_order(order1).unwrap();
        db.create_order(order2).unwrap();

        let orders = db.list_orders().unwrap();
        assert_eq!(orders.len(), 2);
    }

    #[test]
    fn test_update_order_status() {
        let db = Database::new();
        let order = Order::new("John".to_string(), "Monitor".to_string(), 300.0);
        let order_id = order.id;

        db.create_order(order).unwrap();

        let updated = db
            .update_order_status(&order_id, OrderStatus::Processing)
            .unwrap();
        assert_eq!(updated.status, OrderStatus::Processing);

        let retrieved = db.get_order(&order_id).unwrap();
        assert_eq!(retrieved.status, OrderStatus::Processing);
    }

    #[test]
    fn test_delete_order() {
        let db = Database::new();
        let order = Order::new("John".to_string(), "Keyboard".to_string(), 150.0);
        let order_id = order.id;

        db.create_order(order).unwrap();
        assert!(db.get_order(&order_id).is_ok());

        db.delete_order(&order_id).unwrap();
        assert!(db.get_order(&order_id).is_err());
    }

    #[test]
    fn test_get_nonexistent_order() {
        let db = Database::new();
        let random_id = Uuid::new_v4();

        let result = db.get_order(&random_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_nonexistent_order() {
        let db = Database::new();
        let random_id = Uuid::new_v4();

        let result = db.delete_order(&random_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_nonexistent_order() {
        let db = Database::new();
        let random_id = Uuid::new_v4();

        let result = db.update_order_status(&random_id, OrderStatus::Completed);
        assert!(result.is_err());
    }
}
