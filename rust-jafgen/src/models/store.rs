use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::simulation::time::Day;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Store {
    pub id: String,
    pub name: String,
    pub city: String,
    pub country: String,
    pub base_popularity: f64,
    pub opened_day: i64,
    pub tax_rate: f64,
    pub tam: i64,
}

impl Store {
    pub fn new(
        name: &str,
        city: &str,
        country: &str,
        base_popularity: f64,
        opened_day: i64,
        tax_rate: f64,
        tam: i64,
    ) -> Self {
        Store {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            city: city.to_string(),
            country: country.to_string(),
            base_popularity,
            opened_day,
            tax_rate,
            tam,
        }
    }

    pub fn p_buy(&self, day: &Day) -> f64 {
        self.base_popularity * day.get_effect()
    }

    pub fn is_open(&self, day: &Day) -> bool {
        day.date_index >= self.opened_day
    }

    pub fn opened_at_date(&self, epoch: NaiveDate) -> NaiveDate {
        epoch + chrono::Duration::days(self.opened_day)
    }

    pub fn to_csv_row(&self, epoch: NaiveDate) -> Vec<String> {
        let opened_at = self.opened_at_date(epoch);
        vec![
            self.id.clone(),
            self.name.clone(),
            opened_at.format("%Y-%m-%d").to_string(),
            self.tax_rate.to_string(),
        ]
    }

    pub fn country_code(&self) -> &str {
        &self.country
    }
}

/// Default store configurations matching the Python version
pub fn default_stores() -> Vec<Store> {
    vec![
        Store::new("Philadelphia", "Philadelphia", "US", 0.85, 0, 0.06, 900),
        Store::new("Brooklyn", "Brooklyn", "US", 0.95, 192, 0.04, 1400),
        Store::new("Chicago", "Chicago", "US", 0.92, 605, 0.0625, 1200),
        Store::new("San Francisco", "San Francisco", "US", 0.87, 615, 0.075, 1100),
        Store::new("New Orleans", "New Orleans", "US", 0.92, 920, 0.04, 800),
        Store::new("Los Angeles", "Los Angeles", "US", 0.87, 1107, 0.08, 800),
    ]
}
