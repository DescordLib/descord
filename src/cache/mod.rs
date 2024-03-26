use std::num::NonZeroUsize;

use lru::LruCache;
use tokio::sync::Mutex;

use crate::consts::{MESSAGE_CACHE_SIZE, RATE_LIMITS_CACHE_SIZE};
use crate::prelude::Message;

#[derive(Debug)]
pub struct RateLimitInfo {
    pub limit: u32,
    pub remaining: u32,
    pub reset: f64,
}

lazy_static::lazy_static! {
    pub(crate) static ref MESSAGE_CACHE: Mutex<LruCache<String, Message>>
        = Mutex::new(LruCache::new(NonZeroUsize::new(MESSAGE_CACHE_SIZE).unwrap()));

    pub(crate) static ref RATE_LIMITS: Mutex<LruCache<String, RateLimitInfo>>
        = Mutex::new(LruCache::new(NonZeroUsize::new(RATE_LIMITS_CACHE_SIZE).unwrap()));
    pub(crate) static ref ENDPOINT_BUCKET_MAP: Mutex<LruCache<String, String>>
        = Mutex::new(LruCache::new(NonZeroUsize::new(RATE_LIMITS_CACHE_SIZE).unwrap()));
}
