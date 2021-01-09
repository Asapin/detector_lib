use std::collections::HashMap;
use author_data::Reason;
use reg_date_loader::{CachedRegDateLoader, RegDateLoader};
use self::{chat_action::ChatAction, detector_params::DetectorParams, stream_data::StreamData};

pub mod chat_action;
pub mod detector_params;
pub mod reg_date;
pub mod reg_date_loader;
mod author_data;
mod message_data;
mod stream_data;
mod emoji;

pub struct ProcessingResult {
    pub message_id: String,
    pub author: String,
    pub menu_param: String,
    pub reason: Reason
}

pub struct Detector {
    stream_data: StreamData,
    params: DetectorParams,
    reg_date_loader: CachedRegDateLoader
}

impl Detector {
    pub fn new(params: DetectorParams, loader: Box<dyn RegDateLoader>) -> Self {
        let cached_loader = CachedRegDateLoader::new(params.min_reg_date_copy(), loader);
        Detector {
            params,
            reg_date_loader: cached_loader,
            stream_data: StreamData::new()
        }
    }

    pub async fn process_messages(&mut self, mut actions: Vec<ChatAction>) -> Result<Vec<ProcessingResult>, String> {
        actions.sort_unstable_by_key(|action| {
            match action {
                ChatAction::Message { 
                    id: _, 
                    author: _, 
                    timestamp, 
                    content: _, 
                    badges: _,
                    context_params: _,
                } => *timestamp,
                ChatAction::Support { 
                    author: _, 
                    timestamp 
                } => *timestamp,
                ChatAction::RetractedMessage {
                    author: _,
                    timestamp
                } => *timestamp
            }
        });

        self
            .stream_data
            .process_messages(&self.params, &mut self.reg_date_loader, actions)
            .await
    }

    pub fn set_slow_mode(&mut self, delay: u32) {
        self.stream_data.set_slow_mode(delay);
    }

    pub fn update_params(&mut self, params: DetectorParams) {
        self.reg_date_loader.update_default_reg_date(params.min_reg_date_copy());
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