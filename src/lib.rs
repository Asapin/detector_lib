use std::collections::HashMap;
use author_data::Reason;
use self::{chat_action::ChatAction, detector_params::DetectorParams, stream_data::StreamData};

pub mod chat_action;
pub mod detector_params;
mod author_data;
mod message_data;
mod stream_data;
mod emoji;

pub struct ProcessingResult {
    pub message_id: String,
    pub author: String
}

impl ProcessingResult {
    fn new(message_id: String, author: String) -> Self {
        ProcessingResult {
            message_id,
            author
        }
    }
}

pub struct Detector {
    stream_data: StreamData,
    params: DetectorParams
}

impl Detector {
    pub fn new(params: DetectorParams) -> Self {
        Detector {
            params,
            stream_data: StreamData::new()
        }
    }

    pub fn process_messages(&mut self, mut actions: Vec<ChatAction>) -> Vec<ProcessingResult> {
        actions.sort_unstable_by_key(|action| {
            match action {
                ChatAction::Message { 
                    id: _, 
                    author: _, 
                    timestamp, 
                    content: _, 
                    badges: _ 
                } => *timestamp,
                ChatAction::Support { 
                    author: _, 
                    timestamp 
                } => *timestamp
            }
        });

        self
            .stream_data
            .process_messages(&self.params, actions)
    }

    pub fn set_slow_mode(&mut self, delay: u32) {
        self.stream_data.set_slow_mode(delay);
    }

    pub fn update_params(&mut self, params: DetectorParams) {
        self.params = params;
        self.stream_data.clear_authors_to_report();
    }

    pub fn is_author_spammer(&self, author: &str) -> bool {
        self.stream_data.is_author_spammer(author)
    }

    pub fn get_spammers(&self) -> &HashMap<String, Reason> {
        self.stream_data.get_spammers()
    }
}