use vec1::Vec1;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Badges {
    MEMBER,
    VERIFIED,
    OWNER,
    MODERATOR
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ChatAction {
    Message {
        id: String,
        author: String,
        timestamp: u64,
        content: String,
        badges: Option<Vec1<Badges>>
    },
    Support {
        author: String,
        timestamp: u64,
    }
}