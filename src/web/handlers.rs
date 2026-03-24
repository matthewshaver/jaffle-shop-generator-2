use axum::{
    extract::Json,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::{Deserialize, Serialize};

use crate::models::store::Store;
use crate::simulation::sim::{run_simulation, save_results, SimConfig};

#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    pub start_date: String,
    pub end_date: String,
    pub num_orders: Option<u64>,
    pub stores: Vec<StoreInput>,
    pub prefix: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StoreInput {
    pub name: String,
    pub city: String,
    pub country: String,
    pub base_popularity: f64,
    pub opened_offset_days: i64,
    pub tax_rate: f64,
    pub tam: i64,
}

#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    pub success: bool,
    pub message: String,
    pub stats: Option<GenerateStats>,
}

#[derive(Debug, Serialize)]
pub struct GenerateStats {
    pub total_orders: usize,
    pub total_customers: usize,
    pub total_tweets: usize,
    pub total_days: i64,
    pub output_path: String,
}

pub async fn index() -> Html<&'static str> {
    Html(include_str!("../../static/index.html"))
}

pub async fn generate(Json(req): Json<GenerateRequest>) -> impl IntoResponse {
    let start_date = match chrono::NaiveDate::parse_from_str(&req.start_date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(GenerateResponse {
                    success: false,
                    message: format!("Invalid start date: {}", e),
                    stats: None,
                }),
            );
        }
    };

    let end_date = match chrono::NaiveDate::parse_from_str(&req.end_date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(GenerateResponse {
                    success: false,
                    message: format!("Invalid end date: {}", e),
                    stats: None,
                }),
            );
        }
    };

    if end_date <= start_date {
        return (
            StatusCode::BAD_REQUEST,
            Json(GenerateResponse {
                success: false,
                message: "End date must be after start date".to_string(),
                stats: None,
            }),
        );
    }

    let stores: Vec<Store> = req
        .stores
        .iter()
        .map(|s| {
            Store::new(
                &s.name,
                &s.city,
                &s.country,
                s.base_popularity,
                s.opened_offset_days,
                s.tax_rate,
                s.tam,
            )
        })
        .collect();

    if stores.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(GenerateResponse {
                success: false,
                message: "At least one store is required".to_string(),
                stats: None,
            }),
        );
    }

    let config = SimConfig {
        start_date,
        end_date,
        num_orders: req.num_orders,
        stores,
        prefix: req.prefix.unwrap_or_else(|| "raw".to_string()),
    };

    let result = run_simulation(&config);

    match save_results(&config, &result) {
        Ok(output_path) => (
            StatusCode::OK,
            Json(GenerateResponse {
                success: true,
                message: "Data generated successfully!".to_string(),
                stats: Some(GenerateStats {
                    total_orders: result.orders.len(),
                    total_customers: result.customers.len(),
                    total_tweets: result.tweets.len(),
                    total_days: result.total_days,
                    output_path,
                }),
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(GenerateResponse {
                success: false,
                message: format!("Failed to save results: {}", e),
                stats: None,
            }),
        ),
    }
}

pub async fn default_config() -> Json<DefaultConfigResponse> {
    use crate::models::store::default_stores;

    let stores: Vec<StoreInput> = default_stores()
        .into_iter()
        .map(|s| StoreInput {
            name: s.name,
            city: s.city,
            country: s.country,
            base_popularity: s.base_popularity,
            opened_offset_days: s.opened_day,
            tax_rate: s.tax_rate,
            tam: s.tam,
        })
        .collect();

    Json(DefaultConfigResponse {
        start_date: "2023-09-01".to_string(),
        end_date: "2024-09-01".to_string(),
        stores,
    })
}

#[derive(Debug, Serialize)]
pub struct DefaultConfigResponse {
    pub start_date: String,
    pub end_date: String,
    pub stores: Vec<StoreInput>,
}

impl Serialize for StoreInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("StoreInput", 7)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("city", &self.city)?;
        state.serialize_field("country", &self.country)?;
        state.serialize_field("base_popularity", &self.base_popularity)?;
        state.serialize_field("opened_offset_days", &self.opened_offset_days)?;
        state.serialize_field("tax_rate", &self.tax_rate)?;
        state.serialize_field("tam", &self.tam)?;
        state.end()
    }
}
