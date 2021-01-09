use std::collections::HashMap;
use futures::Future;

use crate::reg_date::RegDate;

pub struct RegDateLoader<F, Fut>
where
    F: Fn(&str) -> Fut,
    Fut: Future<Output = Result<Option<RegDate>, String>>
{
    cache: HashMap<String, RegDate>,
    default_reg_date: RegDate,
    load_callback: F 
}

impl <F, Fut> RegDateLoader<F, Fut>
where
    F: Fn(&str) -> Fut,
    Fut: Future<Output = Result<Option<RegDate>, String>>
{
    pub fn new(default_reg_date: RegDate, callback: F) -> Self {
        RegDateLoader {
            cache: HashMap::with_capacity(100),
            default_reg_date,
            load_callback: callback,
        }
    }

    pub async fn load_reg_date(&mut self, author: &str) -> Result<RegDate, String> {
        if let Some(reg_date) = self.cache.get(author) {
            return Ok(reg_date.clone());
        }

        let reg_date = (self.load_callback)(author)
            .await?
            .unwrap_or_else(|| self.default_reg_date.clone());
        self.cache.insert(author.to_string(), reg_date.clone());
        Ok(reg_date)
    }

    pub fn update_default_reg_date(&mut self, new_default_reg_date: RegDate) {
        self.default_reg_date = new_default_reg_date;
    }
}