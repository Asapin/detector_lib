use std::collections::HashMap;
use futures::Future;

use crate::reg_date::RegDate;

pub struct RegDateLoader<F, Fut>
where
    F: Fn(&str) -> Fut,
    Fut: Future<Output = Result<RegDate, String>>
{
    cache: HashMap<String, RegDate>,
    load_callback: F 
}

impl <F, Fut> RegDateLoader<F, Fut>
where
    F: Fn(&str) -> Fut,
    Fut: Future<Output = Result<RegDate, String>>
{
    pub fn new(callback: F) -> Self {
        RegDateLoader {
            cache: HashMap::with_capacity(100),
            load_callback: callback
        }
    }

    pub async fn load_reg_date(&mut self, author: &str) -> Result<RegDate, String> {
        if let Some(reg_date) = self.cache.get(author) {
            return Ok(reg_date.clone());
        }

        let reg_date = &(self.load_callback)(author).await?;
        self.cache.insert(author.to_string(), reg_date.clone());
        Ok(reg_date.clone())
    }
}