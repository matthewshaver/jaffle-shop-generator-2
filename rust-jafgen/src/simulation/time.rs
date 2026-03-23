use chrono::NaiveDate;

/// Epoch: September 1, 2023
pub fn epoch() -> NaiveDate {
    NaiveDate::from_ymd_opt(2023, 9, 1).unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Fall,
}

#[derive(Debug, Clone)]
pub struct Day {
    pub date_index: i64,
    pub minutes: u32,
    date: NaiveDate,
}

impl Day {
    pub fn new(date_index: i64, minutes: u32, start_date: NaiveDate) -> Self {
        let date = start_date + chrono::Duration::days(date_index);
        Day {
            date_index,
            minutes,
            date,
        }
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn day_of_week(&self) -> u32 {
        // Monday=0, Sunday=6 (matching Python's weekday())
        self.date.weekday().num_days_from_monday()
    }

    pub fn is_weekend(&self) -> bool {
        self.day_of_week() >= 5
    }

    pub fn day_of_year(&self) -> u32 {
        self.date.ordinal()
    }

    pub fn month(&self) -> u32 {
        self.date.month()
    }

    pub fn year(&self) -> i32 {
        self.date.year()
    }

    pub fn season(&self) -> Season {
        let month = self.month();
        let day = self.date.day();

        match month {
            1 | 2 => Season::Winter,
            3 if day <= 20 => Season::Winter,
            3 => Season::Spring,
            4 | 5 => Season::Spring,
            6 if day <= 20 => Season::Spring,
            6 => Season::Summer,
            7 | 8 => Season::Summer,
            9 if day <= 20 => Season::Summer,
            9 => Season::Fall,
            10 | 11 => Season::Fall,
            12 if day <= 20 => Season::Fall,
            12 => Season::Winter,
            _ => Season::Winter,
        }
    }

    pub fn get_effect(&self) -> f64 {
        let annual = self.annual_effect();
        let weekend = self.weekend_effect();
        let growth = self.growth_effect();
        annual * weekend * growth
    }

    fn annual_effect(&self) -> f64 {
        let day_of_year = self.day_of_year() as f64;
        let x = day_of_year / 365.0 * 2.0 * std::f64::consts::PI;
        (x.cos() + 1.0) / 10.0 + 0.8
    }

    fn weekend_effect(&self) -> f64 {
        if self.is_weekend() {
            0.6
        } else {
            1.0
        }
    }

    fn growth_effect(&self) -> f64 {
        let year = self.year();
        let month = self.month();
        let x = ((year - 2016) * 12 + month as i32) as f64;
        1.0 + (x / 12.0) * 0.2
    }
}

use chrono::Datelike;
