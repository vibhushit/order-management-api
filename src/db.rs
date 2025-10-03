use crate::models::{Order, OrderStatus};
use anyhow::{anyhow, Result};
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
