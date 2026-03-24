use rand::Rng;

use crate::models::customer::{persona_weights, Customer};
use crate::models::order::Order;
use crate::models::store::Store;
use crate::models::tweet::Tweet;
use crate::simulation::time::Day;

pub struct Market {
    pub store: Store,
    pub customers: Vec<Customer>,
    pub days_to_penetration: i64,
}

#[derive(Debug)]
pub struct DayResult {
    pub order: Option<Order>,
    pub tweet: Option<Tweet>,
    pub customer: Customer,
}

impl Market {
    pub fn new(store: Store, num_customers: i64, rng: &mut impl Rng) -> Self {
        let weights = persona_weights();
        let mut customers = Vec::new();

        for (persona, weight) in &weights {
            let count = (*weight * num_customers as f64).round() as usize;
            for _ in 0..count {
                customers.push(Customer::new(*persona, &store.id, rng));
            }
        }

        // Shuffle customers so that the take(active_count) approach
        // doesn't always favour the same personas in order.
        for i in (1..customers.len()).rev() {
            let j = rng.gen_range(0..=i);
            customers.swap(i, j);
        }

        Market {
            store,
            customers,
            days_to_penetration: 365,
        }
    }

    fn penetration_factor(&self, day: &Day) -> f64 {
        if !self.store.is_open(day) {
            return 0.0;
        }

        let days_open = day.date_index - self.store.opened_day;
        let penetration = (days_open as f64) / (self.days_to_penetration as f64);
        let penetration = penetration.min(1.0).max(0.0);

        if days_open < 7 {
            (1.2 + penetration * (std::f64::consts::E - 1.2)).ln()
        } else {
            (1.0 + penetration * (std::f64::consts::E - 1.0)).ln()
        }
    }

    pub fn sim_day(&self, day: &Day, rng: &mut impl Rng) -> Vec<DayResult> {
        let mut results = Vec::new();

        if !self.store.is_open(day) {
            return results;
        }

        let penetration = self.penetration_factor(day);
        let active_count = (self.customers.len() as f64 * penetration).round() as usize;

        for customer in self.customers.iter().take(active_count) {
            let p_buy_season = self.store.p_buy(day);
            let p_buy_persona = customer.p_buy_persona(day);
            let p_buy = (p_buy_season * p_buy_persona).sqrt();

            let p_buy_threshold: f64 = rng.gen();

            if p_buy > p_buy_threshold {
                let items = customer.get_order_items(day, rng);
                if items.is_empty() {
                    continue;
                }

                let minute = customer.get_order_minute(day, rng);
                let order = Order::new(
                    &customer.id,
                    &self.store.id,
                    day.date(),
                    minute,
                    items,
                    self.store.tax_rate,
                );

                let p_tweet = customer.p_tweet_persona();
                let p_tweet_threshold: f64 = rng.gen();

                let tweet = if p_tweet > p_tweet_threshold {
                    Some(Tweet::new(
                        &customer.id,
                        order.ordered_at,
                        &order.items,
                        customer.fan_level,
                        rng,
                    ))
                } else {
                    None
                };

                results.push(DayResult {
                    order: Some(order),
                    tweet,
                    customer: customer.clone(),
                });
            }
        }

        results
    }
}
