use std::borrow::Cow;

use super::{detector_params::DetectorParams, message_data::MessageData};

#[derive(Debug)]
pub enum Reason {
    SlowMode,
    TooFast(u32),
    TooLong(f32),
    Similar
}

pub struct AuthorData {
    last_message_timestamp: u64,
    sent_messages: Vec<MessageData>,
    avg_delay: u32,
    total_messages: u32,
    avg_message_length: f32,
}

impl AuthorData {
    pub fn new(first_message: String, message_timestamp: u64) -> Self {
        let message_length = first_message.chars().count();
        let new_message_data = MessageData::new(first_message);
        AuthorData {
            last_message_timestamp: message_timestamp,
            sent_messages: vec![new_message_data],
            avg_delay: 0,
            total_messages: 1,
            avg_message_length: message_length as f32,
        }
    }

    pub fn check_message(
        &mut self, 
        timestamp: u64, 
        content: Cow<str>,
        current_slow_mode: u32,
        detector_params: &DetectorParams,
    ) -> Option<Reason> {
        self.total_messages += 1;

        let time_diff = (timestamp - self.last_message_timestamp) as u32;
        self.last_message_timestamp = timestamp;
        let broke_slow_mode = 
            current_slow_mode != 0 && 
            time_diff < current_slow_mode;
        
        if broke_slow_mode {
            return Some(Reason::SlowMode);
        }

        let total_messages = self.total_messages as u32;
        self.avg_delay = (time_diff + (total_messages - 1) * self.avg_delay) / total_messages;
        if detector_params.is_too_fast(self.avg_delay, self.total_messages) {
            return Some(Reason::TooFast(self.avg_delay));
        }

        self.avg_message_length = (content.chars().count() as f32 + (total_messages - 1) as f32 * self.avg_message_length) / total_messages as f32;
        if detector_params.messages_are_too_long(self.avg_message_length, self.total_messages) {
            return Some(Reason::TooLong(self.avg_message_length));
        }

        if !detector_params.should_check_message(content.chars().count() as u32) {
            return None;
        }

        let mut found_similar_message = false;

        for message_data in self.sent_messages.iter_mut() {
            if message_data.is_message_similar(&content) {
                found_similar_message = true;
                message_data.reconstruct_message(&content);

                if detector_params.sent_too_many_similar_messages(message_data.count()) {
                    return Some(Reason::Similar);
                }

                break;
            }
        }

        if !found_similar_message {
            let new_message_data = MessageData::new(content.to_string());
            self.sent_messages.push(new_message_data);
        }

        None
    }
}