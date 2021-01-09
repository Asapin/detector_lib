use std::collections::HashMap;
use async_trait::async_trait;
use crate::reg_date::RegDate;

#[async_trait]
pub trait RegDateLoader {
    async fn load(&self, author: &str) -> Result<Option<RegDate>, String>;
}

pub struct CachedRegDateLoader {
    cache: HashMap<String, RegDate>,
    default_reg_date: RegDate,
    loader: Box<dyn RegDateLoader>
}

impl CachedRegDateLoader {
    pub fn new(default_reg_date: RegDate, loader: Box<dyn RegDateLoader>) -> Self {
        CachedRegDateLoader {
            cache: HashMap::with_capacity(100),
            default_reg_date,
            loader
        }
    }

    pub async fn load_reg_date(&mut self, author: &str) -> Result<RegDate, String> {
        if let Some(reg_date) = self.cache.get(author) {
            return Ok(reg_date.clone());
        }

        let reg_date = self.loader.load(author)
            .await?
            .unwrap_or_else(|| self.default_reg_date.clone());
        self.cache.insert(author.to_string(), reg_date.clone());
        Ok(reg_date)
    }

    pub fn update_default_reg_date(&mut self, new_default_reg_date: RegDate) {
        self.default_reg_date = new_default_reg_date;
    }
}