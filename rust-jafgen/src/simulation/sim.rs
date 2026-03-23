use std::collections::HashMap;
use std::fs;
use chrono::NaiveDate;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};

use crate::models::customer::Customer;
use crate::models::item::all_items;
use crate::models::order::Order;
use crate::models::store::Store;
use crate::models::supply::all_supplies;
use crate::models::tweet::Tweet;
use crate::simulation::market::Market;
use crate::simulation::time::Day;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimConfig {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub num_orders: Option<u64>,
    pub stores: Vec<Store>,
    pub prefix: String,
}

impl Default for SimConfig {
    fn default() -> Self {
        use crate::models::store::default_stores;

        SimConfig {
            start_date: NaiveDate::from_ymd_opt(2023, 9, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 9, 1).unwrap(),
            num_orders: None,
            stores: default_stores(),
            prefix: "raw".to_string(),
        }
    }
}

pub struct SimResult {
    pub orders: Vec<Order>,
    pub tweets: Vec<Tweet>,
    pub customers: HashMap<String, Customer>,
    pub stores: Vec<Store>,
    pub total_days: i64,
}

pub fn run_simulation(config: &SimConfig) -> SimResult {
    let mut rng = StdRng::from_entropy();

    let total_days = (config.end_date - config.start_date).num_days();

    // Create markets for each store
    let markets: Vec<Market> = config
        .stores
        .iter()
        .map(|store| {
            let num_customers = store.tam / 100 * 100; // Scale TAM
            Market::new(store.clone(), num_customers, &mut rng)
        })
        .collect();

    let mut all_orders: Vec<Order> = Vec::new();
    let mut all_tweets: Vec<Tweet> = Vec::new();
    let mut all_customers: HashMap<String, Customer> = HashMap::new();

    // Run simulation day by day
    for day_idx in 0..total_days {
        let day = Day::new(day_idx, 0, config.start_date);

        for market in &markets {
            let results = market.sim_day(&day, &mut rng);
            for result in results {
                if let Some(order) = result.order {
                    all_orders.push(order);
                }
                if let Some(tweet) = result.tweet {
                    all_tweets.push(tweet);
                }
                all_customers
                    .entry(result.customer.id.clone())
                    .or_insert(result.customer);
            }
        }
    }

    // If num_orders is specified, truncate or note
    if let Some(target) = config.num_orders {
        let target = target as usize;
        if all_orders.len() > target {
            all_orders.truncate(target);
            // Keep tweets that match the time window of remaining orders
            if let Some(last_order) = all_orders.last() {
                all_tweets.retain(|t| t.tweeted_at <= last_order.ordered_at);
            }
        }
    }

    SimResult {
        orders: all_orders,
        tweets: all_tweets,
        customers: all_customers,
        stores: config.stores.clone(),
        total_days,
    }
}

pub fn save_results(config: &SimConfig, result: &SimResult) -> Result<String, String> {
    let cwd = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    let output_dir = cwd.join("jaffle-data");
    let output_dir = output_dir.to_string_lossy().to_string();
    let output_dir = output_dir.as_str();
    fs::create_dir_all(output_dir).map_err(|e| format!("Failed to create output directory: {}", e))?;

    let prefix = &config.prefix;

    // Determine if we need country abbreviations
    // Check if any store has a non-US country
    let has_international = result.stores.iter().any(|s| s.country != "US");

    // Write customers CSV
    write_csv(
        &format!("{}/{}_customers.csv", output_dir, prefix),
        &["id", "name"],
        result
            .customers
            .values()
            .map(|c| c.to_csv_row())
            .collect::<Vec<_>>(),
    )?;

    // Write orders CSV
    write_csv(
        &format!("{}/{}_orders.csv", output_dir, prefix),
        &[
            "id",
            "customer",
            "ordered_at",
            "store_id",
            "subtotal",
            "tax_paid",
            "order_total",
        ],
        result.orders.iter().map(|o| o.to_csv_row()).collect::<Vec<_>>(),
    )?;

    // Write items CSV (order items - each item in each order)
    let items_rows: Vec<Vec<String>> = result
        .orders
        .iter()
        .flat_map(|o| o.items_csv_rows())
        .collect();
    write_csv(
        &format!("{}/{}_items.csv", output_dir, prefix),
        &["order_id", "sku"],
        items_rows,
    )?;

    // Write stores CSV - with country suffix if international
    if has_international {
        // Group stores by country and write separate files
        let mut stores_by_country: HashMap<String, Vec<&Store>> = HashMap::new();
        for store in &result.stores {
            stores_by_country
                .entry(store.country.clone())
                .or_default()
                .push(store);
        }

        for (country, stores) in &stores_by_country {
            let suffix = format!("_{}", country.to_uppercase());
            write_csv(
                &format!("{}/{}_stores{}.csv", output_dir, prefix, suffix),
                &["id", "name", "opened_at", "tax_rate"],
                stores
                    .iter()
                    .map(|s| s.to_csv_row(config.start_date))
                    .collect::<Vec<_>>(),
            )?;
        }
    } else {
        write_csv(
            &format!("{}/{}_stores.csv", output_dir, prefix),
            &["id", "name", "opened_at", "tax_rate"],
            result
                .stores
                .iter()
                .map(|s| s.to_csv_row(config.start_date))
                .collect::<Vec<_>>(),
        )?;
    }

    // Write products CSV
    let products = all_items();
    write_csv(
        &format!("{}/{}_products.csv", output_dir, prefix),
        &["sku", "name", "type", "price", "description"],
        products.iter().map(|i| i.to_csv_row()).collect::<Vec<_>>(),
    )?;

    // Write supplies CSV
    let supplies = all_supplies();
    let supply_rows: Vec<Vec<String>> = supplies.iter().flat_map(|s| s.to_csv_rows()).collect();
    write_csv(
        &format!("{}/{}_supplies.csv", output_dir, prefix),
        &["id", "name", "cost", "perishable", "sku"],
        supply_rows,
    )?;

    // Write tweets CSV
    write_csv(
        &format!("{}/{}_tweets.csv", output_dir, prefix),
        &["id", "user_id", "tweeted_at", "content"],
        result
            .tweets
            .iter()
            .map(|t| t.to_csv_row())
            .collect::<Vec<_>>(),
    )?;

    let abs_path = fs::canonicalize(output_dir)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| output_dir.to_string());

    Ok(abs_path)
}

fn write_csv(path: &str, headers: &[&str], rows: Vec<Vec<String>>) -> Result<(), String> {
    let mut wtr = csv::Writer::from_path(path)
        .map_err(|e| format!("Failed to create CSV writer for {}: {}", path, e))?;

    wtr.write_record(headers)
        .map_err(|e| format!("Failed to write headers to {}: {}", path, e))?;

    for row in rows {
        wtr.write_record(&row)
            .map_err(|e| format!("Failed to write row to {}: {}", path, e))?;
    }

    wtr.flush()
        .map_err(|e| format!("Failed to flush {}: {}", path, e))?;

    Ok(())
}
