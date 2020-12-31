use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use crate::emoji::RegexPatterns;

pub struct TextFieldDescriptor {
    pub name: String,
    pub min: u32,
    pub max: u32,
    pub step: u32,
    pub value: u32
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
            regex_patterns: patterns,
        }
    }

    pub fn new(
        avg_time_threshold: u32,
        avg_time_min_message_count: u32,
        similarity_message_count: u32,
        similarity_min_message_length: u32,
        avg_length_threshold: u32,
        avg_length_message_count: u32
    ) -> Self {
        let patterns = RegexPatterns::new();

        DetectorParams {
            avg_time_threshold,
            avg_time_min_message_count,
            similarity_message_count,
            similarity_min_message_length,
            avg_length_threshold,
            avg_length_message_count,
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

    pub fn strip_message_from_emoji<'t>(&self, message: &'t str) -> Cow<'t, str> {
        self.regex_patterns.clean_message(message)
    }

    pub fn regex() -> RegexPatterns {
        RegexPatterns::new()
    }

    pub fn avg_time_threshold(&self) -> TextFieldDescriptor {
        TextFieldDescriptor {
            name: "avg_time_threshold".to_string(),
            min: 1,
            max: 120000,
            step: 1,
            value: self.avg_time_threshold
        }
    }

    pub fn avg_time_min_message_count(&self) -> TextFieldDescriptor {
        TextFieldDescriptor {
            name: "avg_time_min_message_count".to_string(),
            min: 2,
            max: 99,
            step: 1,
            value: self.avg_time_min_message_count
        }
    }

    pub fn similarity_message_count(&self) -> TextFieldDescriptor {
        TextFieldDescriptor {
            name: "similarity_message_count".to_string(),
            min: 2,
            max: 99,
            step: 1,
            value: self.similarity_message_count
        }
    }

    pub fn similarity_min_message_length(&self) -> TextFieldDescriptor {
        TextFieldDescriptor {
            name: "similarity_min_message_length".to_string(),
            min: 1,
            max: 300,
            step: 1,
            value: self.similarity_min_message_length
        }
    }

    pub fn avg_length_threshold(&self) -> TextFieldDescriptor {
        TextFieldDescriptor {
            name: "avg_length_threshold".to_string(),
            min: 1,
            max: 300,
            step: 1,
            value: self.avg_length_threshold
        }
    }

    pub fn avg_length_message_count(&self) -> TextFieldDescriptor {
        TextFieldDescriptor {
            name: "avg_length_message_count".to_string(),
            min: 0,
            max: 99,
            step: 1,
            value: self.avg_length_message_count
        }
    }
}