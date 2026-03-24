use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::Serialize;
use uuid::Uuid;

use super::item::Item;

#[derive(Debug, Clone, Serialize)]
pub struct Order {
    pub id: String,
    pub customer_id: String,
    pub store_id: String,
    pub ordered_at: NaiveDateTime,
    pub items: Vec<Item>,
    pub subtotal: f64,
    pub tax_paid: f64,
    pub order_total: f64,
}

impl Order {
    pub fn new(
        customer_id: &str,
        store_id: &str,
        date: NaiveDate,
        minutes: u32,
        items: Vec<Item>,
        tax_rate: f64,
    ) -> Self {
        let hours = minutes / 60;
        let mins = minutes % 60;
        let time = NaiveTime::from_hms_opt(hours.min(23), mins.min(59), 0)
            .unwrap_or_else(|| NaiveTime::from_hms_opt(23, 59, 0).unwrap());
        let ordered_at = NaiveDateTime::new(date, time);

        let subtotal: f64 = items.iter().map(|i| i.price).sum();
        let tax_paid = (tax_rate * subtotal * 100.0).round() / 100.0;
        let order_total = subtotal + tax_paid;

        Order {
            id: Uuid::new_v4().to_string(),
            customer_id: customer_id.to_string(),
            store_id: store_id.to_string(),
            ordered_at,
            items,
            subtotal,
            tax_paid,
            order_total,
        }
    }

    pub fn subtotal_cents(&self) -> i64 {
        (self.subtotal * 100.0).round() as i64
    }

    pub fn tax_paid_cents(&self) -> i64 {
        (self.tax_paid * 100.0).round() as i64
    }

    pub fn order_total_cents(&self) -> i64 {
        (self.order_total * 100.0).round() as i64
    }

    pub fn to_csv_row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.customer_id.clone(),
            self.ordered_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
            self.store_id.clone(),
            self.subtotal_cents().to_string(),
            self.tax_paid_cents().to_string(),
            self.order_total_cents().to_string(),
        ]
    }

    pub fn items_csv_rows(&self) -> Vec<Vec<String>> {
        self.items
            .iter()
            .map(|item| {
                vec![
                    self.id.clone(),
                    item.sku.clone(),
                ]
            })
            .collect()
    }
}
