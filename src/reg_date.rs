use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegDate {
    pub year: u16,
    pub month: u8,
    pub day: u8
}

impl RegDate {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        RegDate {
            year,
            month,
            day
        }
    }
}

impl PartialEq for RegDate {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year &&
        self.month == other.month &&
        self.day == other.day
    }
}

impl PartialOrd for RegDate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.year > other.year {
            Some(std::cmp::Ordering::Greater)
        } else if self.year < other.year {
            Some(std::cmp::Ordering::Less)
        } else if self.month > other.month {
            Some(std::cmp::Ordering::Greater)
        } else if self.month < other.month {
            Some(std::cmp::Ordering::Less)
        } else if self.day > other.day {
            Some(std::cmp::Ordering::Greater)
        } else if self.day < other.day {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl Default for RegDate {
    fn default() -> Self {
        RegDate {
            year: 2020,
            month: 10,
            day: 1
        }
    }
}