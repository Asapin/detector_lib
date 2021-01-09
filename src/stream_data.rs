use std::collections::{HashMap, HashSet};
use futures::Future;

use crate::{ProcessingResult, author_data::Reason, reg_date::RegDate, reg_date_loader::RegDateLoader};
use super::{author_data::AuthorData, chat_action::ChatAction, detector_params::DetectorParams};

pub struct StreamData {
    authors_to_report: HashMap<String, Reason>,
    superchated_authors: HashSet<String>,
    authors: HashMap<String, AuthorData>,
    slow_mode: u32,
}

impl StreamData {
    pub fn new() -> Self {
       StreamData {
           authors_to_report: HashMap::with_capacity(100),
           superchated_authors: HashSet::with_capacity(100),
           authors: HashMap::with_capacity(500),
           slow_mode: 0,
       }
    }

    pub async fn process_messages<F, Fut>(
        &mut self,
        detector_params: &DetectorParams,
        reg_date_loader: &mut RegDateLoader<F, Fut>,
        messages: Vec<ChatAction>
    ) -> Result<Vec<ProcessingResult>, String>
    where
        F: Fn(&str) -> Fut,
        Fut: Future<Output = Result<Option<RegDate>, String>>
    {
        let mut result = Vec::new();
        for message in messages.into_iter() {
            match message {
                ChatAction::Message { 
                    id, 
                    author, 
                    timestamp, 
                    content, 
                    badges ,
                    context_params
                } => {
                    if badges.is_some() || self.superchated_authors.contains(&author) {
                        continue;
                    }

                    if let Some(reason) = self.authors_to_report.get(&author) {
                        result.push(ProcessingResult {
                            message_id: id,
                            author,
                            menu_param: context_params,
                            reason: reason.clone()
                        });
                        continue;
                    }

                    let timestamp = timestamp / 1_000;
                    let cleaned_content = detector_params.strip_message_from_emoji(&content);

                    if let Some(author_data) = self.authors.get_mut(&author) {
                        if let Some(reason) = author_data.check_message(timestamp, cleaned_content, self.slow_mode, detector_params) {
                            let reg_date = reg_date_loader.load_reg_date(&author).await?;
                            if detector_params.acc_too_young(&reg_date) {
                                self.authors_to_report.insert(author.clone(), reason.clone());
                                result.push(ProcessingResult {
                                    message_id: id,
                                    author,
                                    menu_param: context_params,
                                    reason
                                });
                            }
                        }
                    } else {
                        let author_data = AuthorData::new(content, timestamp);
                        self.authors.insert(author, author_data);
                    }
                },
                ChatAction::Support { 
                    author ,
                    timestamp: _,
                } => {
                    self.authors_to_report.remove(&author);
                    self.superchated_authors.insert(author);
                },
                ChatAction::RetractedMessage {
                    author,
                    timestamp: _,
                } => {
                    if let Some(_) = self.authors_to_report.get(&author) {
                        continue;
                    }

                    let reg_date = reg_date_loader.load_reg_date(&author).await?;
                    if detector_params.acc_too_young(&reg_date) {
                        self.authors_to_report.insert(author, Reason::RetractedMessage);
                    }
                }
            }
        }

        Ok(result)
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