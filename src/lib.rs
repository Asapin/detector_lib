use std::collections::{HashMap, HashSet};

use author_data::Reason;

use self::{chat_action::ChatAction, detector_params::DetectorParams, stream_data::StreamData};

pub mod chat_action;
pub mod detector_params;
mod author_data;
mod message_data;
mod stream_data;
mod emoji;

pub struct Detector {
    stream_data: StreamData,
    params: DetectorParams,
    message_ids_to_report: HashSet<String>,
}

impl Detector {
    pub fn new(params: DetectorParams) -> Detector {
        Detector {
            params,
            stream_data: StreamData::new(),
            message_ids_to_report: HashSet::with_capacity(100),
        }
    }

    pub fn process_messages(&mut self, mut actions: Vec<ChatAction>) {
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

        let message_ids = self
            .stream_data
            .process_messages(&self.params, actions);

        if !message_ids.is_empty() {
            self.message_ids_to_report.reserve(message_ids.len());
            self.message_ids_to_report.extend(message_ids);
        }
    }

    pub fn check_message_id(&mut self, message_id: &str) -> bool {
        self.message_ids_to_report.remove(message_id)
    }

    pub fn set_slow_mode(&mut self, delay: u32) {
        self.stream_data.set_slow_mode(delay);
    }

    pub fn update_params(&mut self, params: DetectorParams) {
        self.params = params;
        self.message_ids_to_report.clear();
        self.stream_data.clear_authors_to_report();
    }

    pub fn is_author_spammer(&self, author: &str) -> bool {
        self.stream_data.is_author_spammer(author)
    }

    pub fn get_spammers(&self) -> &HashMap<String, Reason> {
        self.stream_data.get_spammers()
    }
}