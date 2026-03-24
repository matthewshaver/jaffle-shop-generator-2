use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Supply {
    pub id: String,
    pub name: String,
    pub cost: f64,
    pub perishable: bool,
    pub skus: Vec<String>,
}

impl Supply {
    pub fn cost_cents(&self) -> i64 {
        (self.cost * 100.0).round() as i64
    }

    pub fn to_csv_rows(&self) -> Vec<Vec<String>> {
        self.skus
            .iter()
            .map(|sku| {
                vec![
                    self.id.clone(),
                    self.name.clone(),
                    self.cost_cents().to_string(),
                    if self.perishable {
                        "True".to_string()
                    } else {
                        "False".to_string()
                    },
                    sku.clone(),
                ]
            })
            .collect()
    }
}

pub fn all_supplies() -> Vec<Supply> {
    let jaffle_skus: Vec<String> = (1..=5).map(|i| format!("JAF-{:03}", i)).collect();
    let beverage_skus: Vec<String> = (1..=5).map(|i| format!("BEV-{:03}", i)).collect();

    vec![
        // Reusable supplies (non-perishable)
        Supply {
            id: "SUP-001".to_string(),
            name: "compostable cutlery knife".to_string(),
            cost: 0.07,
            perishable: false,
            skus: jaffle_skus.clone(),
        },
        Supply {
            id: "SUP-002".to_string(),
            name: "cutlery fork".to_string(),
            cost: 0.07,
            perishable: false,
            skus: jaffle_skus.clone(),
        },
        Supply {
            id: "SUP-003".to_string(),
            name: "serving boat".to_string(),
            cost: 0.11,
            perishable: false,
            skus: jaffle_skus.clone(),
        },
        Supply {
            id: "SUP-004".to_string(),
            name: "napkin".to_string(),
            cost: 0.04,
            perishable: false,
            skus: jaffle_skus.clone(),
        },
        Supply {
            id: "SUP-005".to_string(),
            name: "16oz compostable cup".to_string(),
            cost: 0.13,
            perishable: false,
            skus: beverage_skus.clone(),
        },
        Supply {
            id: "SUP-006".to_string(),
            name: "16oz compostable lid".to_string(),
            cost: 0.04,
            perishable: false,
            skus: beverage_skus.clone(),
        },
        Supply {
            id: "SUP-007".to_string(),
            name: "biodegradable straw".to_string(),
            cost: 0.13,
            perishable: false,
            skus: beverage_skus.clone(),
        },
        // Perishable supplies
        Supply {
            id: "SUP-008".to_string(),
            name: "chai mix".to_string(),
            cost: 0.98,
            perishable: true,
            skus: vec!["BEV-002".to_string()],
        },
        Supply {
            id: "SUP-009".to_string(),
            name: "bread".to_string(),
            cost: 0.33,
            perishable: true,
            skus: jaffle_skus.clone(),
        },
        Supply {
            id: "SUP-010".to_string(),
            name: "cheese".to_string(),
            cost: 0.20,
            perishable: true,
            skus: vec![
                "JAF-002".to_string(),
                "JAF-003".to_string(),
                "JAF-004".to_string(),
                "JAF-005".to_string(),
            ],
        },
        Supply {
            id: "SUP-011".to_string(),
            name: "nutella".to_string(),
            cost: 0.46,
            perishable: true,
            skus: vec!["JAF-001".to_string()],
        },
        Supply {
            id: "SUP-012".to_string(),
            name: "banana".to_string(),
            cost: 0.13,
            perishable: true,
            skus: vec!["JAF-001".to_string()],
        },
        Supply {
            id: "SUP-013".to_string(),
            name: "beef stew".to_string(),
            cost: 1.69,
            perishable: true,
            skus: vec!["JAF-002".to_string()],
        },
        Supply {
            id: "SUP-014".to_string(),
            name: "lamb and pork bratwurst".to_string(),
            cost: 2.34,
            perishable: true,
            skus: vec!["JAF-003".to_string()],
        },
        Supply {
            id: "SUP-015".to_string(),
            name: "house-pickled cabbage sauerkraut".to_string(),
            cost: 0.43,
            perishable: true,
            skus: vec!["JAF-003".to_string()],
        },
        Supply {
            id: "SUP-016".to_string(),
            name: "mustard".to_string(),
            cost: 0.07,
            perishable: true,
            skus: vec!["JAF-003".to_string()],
        },
        Supply {
            id: "SUP-017".to_string(),
            name: "pulled pork".to_string(),
            cost: 2.15,
            perishable: true,
            skus: vec!["JAF-004".to_string()],
        },
        Supply {
            id: "SUP-018".to_string(),
            name: "pineapple".to_string(),
            cost: 0.26,
            perishable: true,
            skus: vec!["JAF-004".to_string()],
        },
        Supply {
            id: "SUP-019".to_string(),
            name: "melon".to_string(),
            cost: 0.33,
            perishable: true,
            skus: vec!["JAF-005".to_string()],
        },
        Supply {
            id: "SUP-020".to_string(),
            name: "minced beef".to_string(),
            cost: 1.24,
            perishable: true,
            skus: vec!["JAF-005".to_string()],
        },
        Supply {
            id: "SUP-021".to_string(),
            name: "ghost pepper sauce".to_string(),
            cost: 0.20,
            perishable: true,
            skus: vec!["JAF-004".to_string()],
        },
        Supply {
            id: "SUP-022".to_string(),
            name: "mango".to_string(),
            cost: 0.32,
            perishable: true,
            skus: vec!["BEV-001".to_string()],
        },
        Supply {
            id: "SUP-023".to_string(),
            name: "tangerine".to_string(),
            cost: 0.20,
            perishable: true,
            skus: vec!["BEV-001".to_string()],
        },
        Supply {
            id: "SUP-025".to_string(),
            name: "whey protein".to_string(),
            cost: 0.36,
            perishable: true,
            skus: vec!["BEV-002".to_string()],
        },
        Supply {
            id: "SUP-024".to_string(),
            name: "oatmilk".to_string(),
            cost: 0.11,
            perishable: true,
            skus: vec!["BEV-002".to_string()],
        },
        Supply {
            id: "SUP-026".to_string(),
            name: "coffee".to_string(),
            cost: 0.52,
            perishable: true,
            skus: vec!["BEV-003".to_string(), "BEV-004".to_string()],
        },
        Supply {
            id: "SUP-027".to_string(),
            name: "french vanilla syrup".to_string(),
            cost: 0.72,
            perishable: true,
            skus: vec!["BEV-003".to_string()],
        },
        Supply {
            id: "SUP-028".to_string(),
            name: "kiwi".to_string(),
            cost: 0.20,
            perishable: true,
            skus: vec!["BEV-005".to_string()],
        },
        Supply {
            id: "SUP-029".to_string(),
            name: "lime".to_string(),
            cost: 0.13,
            perishable: true,
            skus: vec!["BEV-005".to_string()],
        },
    ]
}
