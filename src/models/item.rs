use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ItemType {
    Jaffle,
    Beverage,
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItemType::Jaffle => write!(f, "jaffle"),
            ItemType::Beverage => write!(f, "beverage"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Item {
    pub sku: String,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub price: f64,
}

impl Item {
    pub fn price_cents(&self) -> i64 {
        (self.price * 100.0).round() as i64
    }

    pub fn to_csv_row(&self) -> Vec<String> {
        vec![
            self.sku.clone(),
            self.name.clone(),
            self.item_type.to_string(),
            self.price_cents().to_string(),
            self.description.clone(),
        ]
    }
}

pub fn all_jaffles() -> Vec<Item> {
    vec![
        Item {
            sku: "JAF-001".to_string(),
            name: "nutellaphone who dis?".to_string(),
            description: "nutella & banana".to_string(),
            item_type: ItemType::Jaffle,
            price: 11.0,
        },
        Item {
            sku: "JAF-002".to_string(),
            name: "doctor stew".to_string(),
            description: "beef stew".to_string(),
            item_type: ItemType::Jaffle,
            price: 11.0,
        },
        Item {
            sku: "JAF-003".to_string(),
            name: "the krautback".to_string(),
            description: "bratwurst, sauerkraut, mustard".to_string(),
            item_type: ItemType::Jaffle,
            price: 12.0,
        },
        Item {
            sku: "JAF-004".to_string(),
            name: "flame impala".to_string(),
            description: "pulled pork, pineapple, ghost pepper".to_string(),
            item_type: ItemType::Jaffle,
            price: 14.0,
        },
        Item {
            sku: "JAF-005".to_string(),
            name: "mel-bun".to_string(),
            description: "melon, minced beef".to_string(),
            item_type: ItemType::Jaffle,
            price: 12.0,
        },
    ]
}

pub fn all_beverages() -> Vec<Item> {
    vec![
        Item {
            sku: "BEV-001".to_string(),
            name: "tangaroo".to_string(),
            description: "mango, tangerine smoothie".to_string(),
            item_type: ItemType::Beverage,
            price: 6.0,
        },
        Item {
            sku: "BEV-002".to_string(),
            name: "chai and mighty".to_string(),
            description: "oatmilk chai with protein".to_string(),
            item_type: ItemType::Beverage,
            price: 5.0,
        },
        Item {
            sku: "BEV-003".to_string(),
            name: "vanilla ice".to_string(),
            description: "iced coffee with vanilla syrup".to_string(),
            item_type: ItemType::Beverage,
            price: 6.0,
        },
        Item {
            sku: "BEV-004".to_string(),
            name: "for richer or pourover".to_string(),
            description: "hot pourover coffee".to_string(),
            item_type: ItemType::Beverage,
            price: 7.0,
        },
        Item {
            sku: "BEV-005".to_string(),
            name: "adele-ade".to_string(),
            description: "kiwi, lime agua fresca".to_string(),
            item_type: ItemType::Beverage,
            price: 4.0,
        },
    ]
}

pub fn all_items() -> Vec<Item> {
    let mut items = all_jaffles();
    items.extend(all_beverages());
    items
}
