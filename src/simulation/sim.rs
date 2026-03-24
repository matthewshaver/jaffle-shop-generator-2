use std::collections::HashMap;
use std::fs;
use chrono::NaiveDate;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
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
            let num_customers = store.tam;
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

    // Determine target order count
    let target_orders: usize = if let Some(explicit) = config.num_orders {
        // User specified an exact cap — simple truncation
        let target = explicit as usize;
        if all_orders.len() > target {
            all_orders.truncate(target);
            if let Some(last_order) = all_orders.last() {
                all_tweets.retain(|t| t.tweeted_at <= last_order.ordered_at);
            }
        }
        all_orders.len()
    } else {
        // No cap specified: target 10,000–15,000 orders per effective store-year
        // Account for stores that open partway through (or never) via their offset
        let per_store_per_year = rng.gen_range(10_000usize..=15_000usize);
        let effective_store_years: f64 = config.stores.iter().map(|s| {
            let open_days = (total_days - s.opened_day).max(0) as f64;
            open_days / 365.25
        }).sum();
        let target = (per_store_per_year as f64 * effective_store_years.max(1.0)).round() as usize;

        // Group orders by customer
        let mut orders_by_customer: HashMap<String, Vec<Order>> = HashMap::new();
        for order in all_orders.drain(..) {
            orders_by_customer
                .entry(order.customer_id.clone())
                .or_default()
                .push(order);
        }

        let available_customers = orders_by_customer.len();
        // Calculate how many orders per customer we need to hit the target.
        // Prefer ~3 orders/customer/year, but flex up if the pool is too small.
        let avg_years = (effective_store_years / (config.stores.len() as f64).max(1.0)).max(1.0);
        let ideal_per_customer = 3.0 * avg_years;
        let ideal_customers = ((target as f64) / ideal_per_customer).ceil() as usize;

        // If we have enough customers, select ideal_customers.
        // Otherwise, use all available and increase per-customer orders.
        let selected_count = ideal_customers.min(available_customers).max(1);
        let avg_per_customer = ((target as f64) / (selected_count as f64)).ceil().max(1.0) as usize;

        // Shuffle customer IDs and pick selected_count of them
        let mut customer_ids: Vec<String> = orders_by_customer.keys().cloned().collect();
        fisher_yates_shuffle(&mut customer_ids, &mut rng);
        customer_ids.truncate(selected_count);

        // For each selected customer, keep a random number of orders centred
        // on avg_per_customer (uniform between 1 and 2×avg, capped at actual).
        let mut kept_orders: Vec<Order> = Vec::with_capacity(target);
        for cid in &customer_ids {
            if let Some(mut orders) = orders_by_customer.remove(cid) {
                let lower = (avg_per_customer / 2).max(1);
                let upper = (2 * avg_per_customer).min(orders.len()).max(lower);
                let keep = rng.gen_range(lower..=upper);
                fisher_yates_shuffle(&mut orders, &mut rng);
                orders.truncate(keep);
                kept_orders.extend(orders);
            }
        }

        // If we overshot the target, randomly trim
        if kept_orders.len() > target {
            fisher_yates_shuffle(&mut kept_orders, &mut rng);
            kept_orders.truncate(target);
        }

        // Sort chronologically for clean output
        kept_orders.sort_by_key(|o| o.ordered_at);
        all_orders = kept_orders;

        // Split heavy-ordering customers into multiple unique customers
        // so the dataset has a more realistic customer count (~5 orders/customer/year).
        let max_orders_per_customer = (5.0 * avg_years).ceil() as usize;
        if max_orders_per_customer > 0 {
            // Group orders by customer
            let mut orders_by_cust: HashMap<String, Vec<usize>> = HashMap::new();
            for (i, order) in all_orders.iter().enumerate() {
                orders_by_cust.entry(order.customer_id.clone()).or_default().push(i);
            }

            for (cid, indices) in &orders_by_cust {
                if indices.len() <= max_orders_per_customer {
                    continue;
                }
                // Split this customer's orders across multiple new customers
                let num_splits = (indices.len() + max_orders_per_customer - 1) / max_orders_per_customer;
                if num_splits <= 1 {
                    continue;
                }
                // Create new customers based on the original
                let original = all_customers.get(cid).cloned();
                if original.is_none() {
                    continue;
                }
                let orig = original.unwrap();

                // First chunk keeps the original ID; subsequent chunks get new IDs
                for (chunk_idx, chunk) in indices.chunks(max_orders_per_customer).enumerate() {
                    if chunk_idx == 0 {
                        continue; // keep original
                    }
                    let new_customer = Customer::new(orig.persona, &orig.store_id, &mut rng);
                    let new_id = new_customer.id.clone();
                    all_customers.insert(new_id.clone(), new_customer);

                    // Reassign orders to the new customer
                    for &order_idx in chunk {
                        all_orders[order_idx].customer_id = new_id.clone();
                    }
                }
            }
        }

        // Rebuild customer set to match surviving orders
        let active_ids: std::collections::HashSet<String> =
            all_orders.iter().map(|o| o.customer_id.clone()).collect();
        all_customers.retain(|k, _| active_ids.contains(k));

        // Thin tweets to ~10% of final order count so they're a small complement
        all_tweets.retain(|t| active_ids.contains(&t.user_id));
        let tweet_target = (all_orders.len() as f64 * 0.10).round() as usize;
        if all_tweets.len() > tweet_target {
            fisher_yates_shuffle(&mut all_tweets, &mut rng);
            all_tweets.truncate(tweet_target);
            all_tweets.sort_by_key(|t| t.tweeted_at);
        }

        all_orders.len()
    };

    let _ = target_orders; // used for the branching above

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

fn fisher_yates_shuffle<T>(slice: &mut [T], rng: &mut impl Rng) {
    for i in (1..slice.len()).rev() {
        let j = rng.gen_range(0..=i);
        slice.swap(i, j);
    }
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
