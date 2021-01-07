use crate::reg_date::RegDate;
use async_trait::async_trait;

#[async_trait]
pub trait RegDateLoader {
    async fn load(&mut self, author: &str) -> RegDate;
}