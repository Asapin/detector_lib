use std::borrow::Cow;

use crate::emoji::RegexPatterns;

pub struct DetectorParams {
    avg_time_threshold: u32,
    avg_time_min_message_count: u16,
    similar_messages_threshold: u16,
    min_message_len: usize,
    avg_message_length_threshold: usize,
    avg_message_length_message_count: u16,
    regex_patterns: RegexPatterns,
}

impl DetectorParams {
    pub fn new(
        avg_time_threshold: u32,
        avg_time_min_message_count: u16,
        similar_messages_threshold: u16,
        min_message_len: usize,
        avg_message_length_threshold: usize,
        avg_message_length_message_count: u16
    ) -> Self {
        let patterns = RegexPatterns::new();

        DetectorParams {
            avg_time_threshold,
            avg_time_min_message_count,
            similar_messages_threshold,
            min_message_len,
            avg_message_length_threshold,
            avg_message_length_message_count,
            regex_patterns: patterns,
        }
    }

    pub fn is_too_fast(&self, current_delay: u32, sent_messages_count: u16) -> bool {
        self.avg_time_threshold != 0 &&
        current_delay != 0 &&
        current_delay < self.avg_time_threshold &&
        sent_messages_count >= self.avg_time_min_message_count
    }

    pub fn sent_too_many_similar_messages(&self, similar_messages_count: u16) -> bool {
        similar_messages_count >= self.similar_messages_threshold
    }

    pub fn messages_are_too_long(&self, current_average_message_length: f32, sent_messages_count: u16) -> bool {
        self.avg_message_length_message_count != 0 &&
        sent_messages_count >= self.avg_message_length_message_count &&
        current_average_message_length >= self.avg_message_length_threshold as f32
    }

    pub fn should_check_message(&self, message_len: usize) -> bool {
        message_len != 0 && message_len >= self.min_message_len
    }

    pub fn strip_message_from_emoji<'t>(&self, message: &'t str) -> Cow<'t, str> {
        self.regex_patterns.clean_message(message)
    }
}