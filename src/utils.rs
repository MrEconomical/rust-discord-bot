// Imports

use std::sync::Arc;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;

use serenity::cache::Cache;
use serenity::http::{ CacheHttp, Http };

// Message embed macro

#[macro_export]
macro_rules! embed {
    ( $($attr:ident : $value:expr),* ) => {
        {
            let mut embed = serenity::builder::CreateEmbed::default();
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

// Get current system time

pub fn get_time() -> Result<u64, String> {
    Ok(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_millis() as u64
    )
}