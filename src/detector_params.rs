use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use crate::{emoji::RegexPatterns, reg_date::RegDate};

pub struct TextFieldDescriptor {
    pub name: String,
    pub min: u32,
    pub max: u32,
    pub step: u32,
}

impl TextFieldDescriptor {
    pub fn avg_time_threshold() -> TextFieldDescriptor {
        TextFieldDescriptor {
            name: "avg_time_threshold".to_string(),
            min: 1,
            max: 120000,
            step: 1
        }
    }

    pub fn avg_time_min_message_count() -> TextFieldDescriptor {
        TextFieldDescriptor {
            name: "avg_time_min_message_count".to_string(),
            min: 2,
            max: 99,
            step: 1
        }
    }

    pub fn similarity_message_count() -> TextFieldDescriptor {
        TextFieldDescriptor {
            name: "similarity_message_count".to_string(),
            min: 2,
            max: 99,
            step: 1
        }
    }

    pub fn similarity_min_message_length() -> TextFieldDescriptor {
        TextFieldDescriptor {
            name: "similarity_min_message_length".to_string(),
            min: 1,
            max: 300,
            step: 1
        }
    }

    pub fn avg_length_threshold() -> TextFieldDescriptor {
        TextFieldDescriptor {
            name: "avg_length_threshold".to_string(),
            min: 1,
            max: 300,
            step: 1
        }
    }

    pub fn avg_length_message_count() -> TextFieldDescriptor {
        TextFieldDescriptor {
            name: "avg_length_message_count".to_string(),
            min: 0,
            max: 99,
            step: 1
        }
    }

    pub fn min_reg_date_year() -> TextFieldDescriptor {
        TextFieldDescriptor {
            name: "min_reg_date_year".to_string(),
            min: 2000,
            max: 2100,
            step: 1
        }
    }

    pub fn min_reg_date_month() -> TextFieldDescriptor {
        TextFieldDescriptor {
            name: "min_reg_date_month".to_string(),
            min: 1,
            max: 12,
            step: 1
        }
    }

    pub fn min_reg_date_day() -> TextFieldDescriptor {
        TextFieldDescriptor {
            name: "min_reg_date_day".to_string(),
            min: 1,
            max: 31,
            step: 1
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectorParams {
    avg_time_threshold: u32,
    avg_time_min_message_count: u32,
    similarity_message_count: u32,
    similarity_min_message_length: u32,
    avg_length_threshold: u32,
    avg_length_message_count: u32,
    min_reg_date: RegDate,
    #[serde(skip, default = "DetectorParams::regex")]
    regex_patterns: RegexPatterns,
}

impl DetectorParams {
    pub fn get_default() -> Self {
        let patterns = RegexPatterns::new();

        DetectorParams {
            avg_time_threshold: 2000,
            avg_time_min_message_count: 3,
            similarity_message_count: 3,
            similarity_min_message_length: 10,
            avg_length_threshold: 15,
            avg_length_message_count: 5,
            min_reg_date: RegDate::default(),
            regex_patterns: patterns,
        }
    }

    pub fn new(
        avg_time_threshold: u32,
        avg_time_min_message_count: u32,
        similarity_message_count: u32,
        similarity_min_message_length: u32,
        avg_length_threshold: u32,
        avg_length_message_count: u32,
        min_reg_date: RegDate
    ) -> Self {
        let patterns = RegexPatterns::new();

        DetectorParams {
            avg_time_threshold,
            avg_time_min_message_count,
            similarity_message_count,
            similarity_min_message_length,
            avg_length_threshold,
            avg_length_message_count,
            min_reg_date,
            regex_patterns: patterns,
        }
    }

    pub fn is_too_fast(&self, current_delay: u32, sent_messages_count: u32) -> bool {
        self.avg_time_threshold != 0 &&
        current_delay != 0 &&
        current_delay < self.avg_time_threshold &&
        sent_messages_count >= self.avg_time_min_message_count
    }

    pub fn sent_too_many_similar_messages(&self, similar_messages_count: u32) -> bool {
        similar_messages_count >= self.similarity_message_count
    }

    pub fn messages_are_too_long(&self, current_average_message_length: f32, sent_messages_count: u32) -> bool {
        self.avg_length_message_count != 0 &&
        sent_messages_count >= self.avg_length_message_count &&
        current_average_message_length >= self.avg_length_threshold as f32
    }

    pub fn should_check_message(&self, message_len: u32) -> bool {
        message_len != 0 && message_len >= self.similarity_min_message_length
    }

    pub fn acc_too_young(&self, reg_date: &RegDate) -> bool {
        reg_date >= &self.min_reg_date
    }

    pub fn strip_message_from_emoji<'t>(&self, message: &'t str) -> Cow<'t, str> {
        self.regex_patterns.clean_message(message)
    }

    pub fn regex() -> RegexPatterns {
        RegexPatterns::new()
    }

    pub fn avg_time_threshold(&self) -> (u32, TextFieldDescriptor) {
        (self.avg_time_threshold, TextFieldDescriptor::avg_time_threshold())
    }

    pub fn avg_time_min_message_count(&self) -> (u32, TextFieldDescriptor) {
        (self.avg_time_min_message_count, TextFieldDescriptor::avg_time_min_message_count())
    }

    pub fn similarity_message_count(&self) -> (u32, TextFieldDescriptor) {
        (self.similarity_message_count, TextFieldDescriptor::similarity_message_count())
    }

    pub fn similarity_min_message_length(&self) -> (u32, TextFieldDescriptor) {
        (self.similarity_min_message_length, TextFieldDescriptor::similarity_min_message_length())
    }

    pub fn avg_length_threshold(&self) -> (u32, TextFieldDescriptor) {
        (self.avg_length_threshold, TextFieldDescriptor::avg_length_threshold())
    }

    pub fn avg_length_message_count(&self) -> (u32, TextFieldDescriptor) {
        (self.avg_length_message_count, TextFieldDescriptor::avg_length_message_count())
    }

    pub fn min_reg_date_year(&self) -> (u16, TextFieldDescriptor) {
        (self.min_reg_date.year, TextFieldDescriptor::min_reg_date_year())
    }

    pub fn min_reg_date_month(&self) -> (u8, TextFieldDescriptor) {
        (self.min_reg_date.month, TextFieldDescriptor::min_reg_date_month())
    }

    pub fn min_reg_date_day(&self) -> (u8, TextFieldDescriptor) {
        (self.min_reg_date.day, TextFieldDescriptor::min_reg_date_day())
    }

    pub fn min_reg_date_copy(&self) -> RegDate {
        self.min_reg_date.clone()
    }
}