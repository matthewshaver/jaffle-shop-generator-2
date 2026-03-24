use chrono::NaiveDateTime;
use rand::Rng;
use serde::Serialize;
use uuid::Uuid;

use super::item::Item;

#[derive(Debug, Clone, Serialize)]
pub struct Tweet {
    pub id: String,
    pub user_id: String,
    pub tweeted_at: NaiveDateTime,
    pub content: String,
}

impl Tweet {
    pub fn new(
        user_id: &str,
        ordered_at: NaiveDateTime,
        items: &[Item],
        fan_level: i32,
        rng: &mut impl Rng,
    ) -> Self {
        let tweet_offset_minutes = rng.gen_range(0..=20);
        let tweeted_at = ordered_at + chrono::Duration::minutes(tweet_offset_minutes);

        let content = construct_tweet(items, fan_level);

        Tweet {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            tweeted_at,
            content,
        }
    }

    pub fn to_csv_row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.user_id.clone(),
            self.tweeted_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
            self.content.clone(),
        ]
    }
}

fn construct_tweet(items: &[Item], fan_level: i32) -> String {
    let items_sentence = match items.len() {
        0 => return String::new(),
        1 => format!("Ordered a {}", items[0].name),
        2 => format!("Ordered a {} and a {}", items[0].name, items[1].name),
        _ => {
            let mut parts: Vec<String> = items.iter().map(|i| format!("a {}", i.name)).collect();
            let last = parts.pop().unwrap();
            format!("Ordered {}, and {}", parts.join(", "), last)
        }
    };

    let positive_adj = [
        "awesome",
        "delicious",
        "amazing",
        "fantastic",
        "sooo gooood",
        "my favorite",
        "the best",
    ];
    let negative_adj = [
        "terrible",
        "the worst",
        "awful",
        "disgusting",
        "gross",
        "inedible",
        "my least favorite",
    ];
    let neutral_adj = [
        "okay",
        "fine",
        "alright",
        "average",
        "pretty decent",
        "solid",
        "not bad",
        "just meh",
    ];

    if fan_level > 3 {
        let adj = positive_adj[fan_level as usize % positive_adj.len()];
        format!(
            "Jaffles from the Jaffle Shop are {}! {}",
            adj, items_sentence
        )
    } else if fan_level < 3 {
        let adj = negative_adj[fan_level as usize % negative_adj.len()];
        format!(
            "Jaffle Shop again. {}. This place is {}.",
            items_sentence, adj
        )
    } else {
        let adj = neutral_adj[fan_level as usize % neutral_adj.len()];
        format!("Jaffle shop is {}. {}.", adj, items_sentence)
    }
}
