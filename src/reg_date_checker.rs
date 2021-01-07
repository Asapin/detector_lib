use crate::reg_date::RegDate;
use async_trait::async_trait;

#[async_trait]
pub trait RegDateChecker {
    async fn check<'a>(&'a mut self, author: &str) -> &'a RegDate;
}