use rand::Rng;
use serde::Serialize;
use uuid::Uuid;

use super::item::{all_beverages, all_jaffles, Item};
use super::names::random_name;
use crate::simulation::time::{Day, Season};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PersonaType {
    Commuter,
    RemoteWorker,
    BrunchCrowd,
    Student,
    Casuals,
    HealthNut,
}

#[derive(Debug, Clone, Serialize)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub favorite_number: i32,
    pub fan_level: i32,
    pub persona: PersonaType,
    pub store_id: String,
}

impl Customer {
    pub fn new(persona: PersonaType, store_id: &str, rng: &mut impl Rng) -> Self {
        Customer {
            id: Uuid::new_v4().to_string(),
            name: random_name(rng),
            favorite_number: rng.gen_range(1..=100),
            fan_level: rng.gen_range(1..=5),
            persona,
            store_id: store_id.to_string(),
        }
    }

    pub fn p_buy_persona(&self, day: &Day) -> f64 {
        match self.persona {
            PersonaType::Commuter => {
                if day.is_weekend() {
                    0.001
                } else {
                    0.5 + (self.favorite_number as f64 / 100.0) * 0.3
                }
            }
            PersonaType::RemoteWorker => {
                if day.is_weekend() {
                    0.001
                } else {
                    (self.favorite_number as f64 / 100.0) * 0.4
                }
            }
            PersonaType::BrunchCrowd => {
                if day.is_weekend() {
                    0.2 + (self.favorite_number as f64 / 100.0) * 0.2
                } else {
                    0.0
                }
            }
            PersonaType::Student => {
                if day.season() == Season::Summer {
                    0.0
                } else {
                    0.1 + (self.favorite_number as f64 / 100.0) * 0.4
                }
            }
            PersonaType::Casuals => 0.1,
            PersonaType::HealthNut => {
                if day.season() == Season::Summer {
                    0.1 + (self.favorite_number as f64 / 100.0) * 0.4
                } else {
                    0.2
                }
            }
        }
    }

    pub fn p_tweet_persona(&self) -> f64 {
        match self.persona {
            PersonaType::Commuter => 0.2,
            PersonaType::RemoteWorker => 0.01,
            PersonaType::BrunchCrowd => 0.8,
            PersonaType::Student => 0.8,
            PersonaType::Casuals => 0.1,
            PersonaType::HealthNut => 0.6,
        }
    }

    pub fn get_order_minute(&self, day: &Day, rng: &mut impl Rng) -> u32 {
        let (mean, stdev) = match self.persona {
            PersonaType::Commuter => (60.0, 30.0),
            PersonaType::RemoteWorker => (420.0, 180.0),
            PersonaType::BrunchCrowd => {
                let offset = self.favorite_number as f64;
                (300.0 + offset, 120.0)
            }
            PersonaType::Student => (540.0, 120.0),
            PersonaType::Casuals => (300.0, 120.0),
            PersonaType::HealthNut => (300.0, 120.0),
        };

        // Box-Muller transform for normal distribution
        let u1: f64 = rng.gen::<f64>().max(1e-10);
        let u2: f64 = rng.gen::<f64>();
        let normal = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        let minute = mean + stdev * normal;

        // Clamp to valid range based on day type
        let (open, close) = if day.is_weekend() {
            (480u32, 900u32) // 8 AM - 3 PM
        } else {
            (420u32, 1200u32) // 7 AM - 8 PM
        };

        (minute.round().max(open as f64).min(close as f64)) as u32
    }

    pub fn get_order_items(&self, day: &Day, rng: &mut impl Rng) -> Vec<Item> {
        let jaffles = all_jaffles();
        let beverages = all_beverages();
        let mut items = Vec::new();

        match self.persona {
            PersonaType::Commuter => {
                // 1 beverage only
                let idx = rng.gen_range(0..beverages.len());
                items.push(beverages[idx].clone());
            }
            PersonaType::RemoteWorker => {
                // 1-2 beverages + 30% chance of 1 jaffle
                let num_bev = rng.gen_range(1..=2);
                for _ in 0..num_bev {
                    let idx = rng.gen_range(0..beverages.len());
                    items.push(beverages[idx].clone());
                }
                if rng.gen::<f64>() < 0.3 {
                    let idx = rng.gen_range(0..jaffles.len());
                    items.push(jaffles[idx].clone());
                }
            }
            PersonaType::BrunchCrowd => {
                // (1 + favorite_number/20) jaffles + same number of beverages
                let count = 1 + (self.favorite_number as usize / 20);
                for _ in 0..count {
                    let idx = rng.gen_range(0..jaffles.len());
                    items.push(jaffles[idx].clone());
                }
                for _ in 0..count {
                    let idx = rng.gen_range(0..beverages.len());
                    items.push(beverages[idx].clone());
                }
            }
            PersonaType::Student => {
                // 1 beverage + 50% chance of 1 jaffle
                let idx = rng.gen_range(0..beverages.len());
                items.push(beverages[idx].clone());
                if rng.gen::<f64>() < 0.5 {
                    let idx = rng.gen_range(0..jaffles.len());
                    items.push(jaffles[idx].clone());
                }
            }
            PersonaType::Casuals => {
                // random(0-3) beverages + random(0-3) jaffles
                let num_bev = rng.gen_range(0..=3);
                let num_jaf = rng.gen_range(0..=3);
                for _ in 0..num_bev {
                    let idx = rng.gen_range(0..beverages.len());
                    items.push(beverages[idx].clone());
                }
                for _ in 0..num_jaf {
                    let idx = rng.gen_range(0..jaffles.len());
                    items.push(jaffles[idx].clone());
                }
            }
            PersonaType::HealthNut => {
                // 1 beverage only
                let idx = rng.gen_range(0..beverages.len());
                items.push(beverages[idx].clone());
            }
        }

        let _ = day; // day used in persona dispatch above
        items
    }

    pub fn to_csv_row(&self) -> Vec<String> {
        vec![self.id.clone(), self.name.clone()]
    }
}

/// Persona distribution: returns the persona weights for market creation
pub fn persona_weights() -> Vec<(PersonaType, f64)> {
    vec![
        (PersonaType::Commuter, 0.25),
        (PersonaType::RemoteWorker, 0.25),
        (PersonaType::BrunchCrowd, 0.10),
        (PersonaType::Student, 0.20),
        (PersonaType::Casuals, 0.10),
        (PersonaType::HealthNut, 0.10),
    ]
}
