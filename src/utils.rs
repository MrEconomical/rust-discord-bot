// Imports

use std::sync::Arc;

use serenity::cache::Cache;
use serenity::http::{ CacheHttp, Http };

// Message embed macro

#[macro_export]
macro_rules! embed {
    ( $($attr:ident : $value:expr),* ) => {
        {
            let mut embed = CreateEmbed::default();
            embed$( .$attr($value) )*;
            embed
        }
    }
}

// Cache and http type

pub struct CacheAndHttp {
    pub cache: Arc<Cache>,
    pub http: Arc<Http>
}

impl CacheHttp for CacheAndHttp {
    fn http(&self) -> &Http {
        &self.http
    }

    fn cache(&self) -> Option<&Arc<Cache>> {
        Some(&self.cache)
    }
}