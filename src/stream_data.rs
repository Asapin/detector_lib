use std::collections::{HashMap, HashSet};

use crate::author_data::Reason;

use super::{author_data::AuthorData, chat_action::ChatAction, detector_params::DetectorParams};

pub struct StreamData {
    authors_to_report: HashMap<String, Reason>,
    superchated_authors: HashSet<String>,
    authors: HashMap<String, AuthorData>,
    slow_mode: u32
}

impl StreamData {
    pub fn new() -> Self {
       StreamData {
           authors_to_report: HashMap::with_capacity(100),
           superchated_authors: HashSet::with_capacity(100),
           authors: HashMap::with_capacity(500),
           slow_mode: 0
       }
    }

    pub fn process_messages(
        &mut self,
        detector_params: &DetectorParams,
        messages: Vec<ChatAction>
    ) -> HashSet<String> {
        let mut result = HashSet::new();
        for message in messages.into_iter() {
            match message {
                ChatAction::Message { 
                    id, 
                    author, 
                    timestamp, 
                    content, 
                    badges 
                } => {
                    if badges.is_some() || self.superchated_authors.contains(&author) {
                        continue;
                    }

                    if self.authors_to_report.contains_key(&author) {
                        result.insert(id);
                        continue;
                    }

                    let timestamp = timestamp / 1_000;
                    let cleaned_content = detector_params.strip_message_from_emoji(&content);
                    match self.authors.get_mut(&author) {
                        Some(author_data) => {
                            if let Some(reason) = author_data.check_message(timestamp, cleaned_content, self.slow_mode, detector_params) {
                                self.authors_to_report.insert(author, reason);
                                result.insert(id);
                            }
                        }
                        None => {
                            let author_data = AuthorData::new(content, timestamp);
                            self.authors.insert(author, author_data);
                        }
                    }
                },
                ChatAction::Support { 
                    author ,
                    timestamp: _,
                } => {
                    self.authors_to_report.remove(&author);
                    self.superchated_authors.insert(author);
                }
            }
        }

        result
    }

    pub fn set_slow_mode(&mut self, new_delay: u32) {
        self.slow_mode = new_delay;
    }

    pub fn clear_authors_to_report(&mut self) {
        self.authors_to_report.clear();
    }

    pub fn is_author_spammer(&self, author: &str) -> bool {
        self.authors_to_report.contains_key(author)
    }

    pub fn get_spammers(&self) -> &HashMap<String, Reason> {
        &self.authors_to_report
    }
}