use std::num::NonZeroUsize;

use lru::LruCache;
use tokio::sync::Mutex;

use crate::consts::{
    CHANNEL_CACHE_SIZE, GUILD_CACHE_SIZE, MESSAGE_CACHE_SIZE, RATE_LIMITS_CACHE_SIZE,
    ROLE_CACHE_SIZE,
};
use crate::prelude::Role;
use crate::prelude::{Channel, Guild, Message};

#[derive(Debug)]
pub struct RateLimitInfo {
    pub remaining: u32,
    pub reset: f64,
}

lazy_static::lazy_static! {
    pub(crate) static ref MESSAGE_CACHE: Mutex<LruCache<String, Message>>
        = Mutex::new(LruCache::new(NonZeroUsize::new(MESSAGE_CACHE_SIZE).unwrap()));
    pub(crate) static ref ROLE_CACHE: Mutex<LruCache<String, Role>>
        = Mutex::new(LruCache::new(NonZeroUsize::new(ROLE_CACHE_SIZE).unwrap()));
    pub(crate) static ref GUILD_CACHE: Mutex<LruCache<String, Guild>>
        = Mutex::new(LruCache::new(NonZeroUsize::new(GUILD_CACHE_SIZE).unwrap()));
    pub(crate) static ref RATE_LIMITS: Mutex<LruCache<String, RateLimitInfo>>
        = Mutex::new(LruCache::new(NonZeroUsize::new(RATE_LIMITS_CACHE_SIZE).unwrap()));
    pub(crate) static ref ENDPOINT_BUCKET_MAP: Mutex<LruCache<String, String>>
        = Mutex::new(LruCache::new(NonZeroUsize::new(RATE_LIMITS_CACHE_SIZE).unwrap()));
    pub(crate) static ref CHANNEL_CACHE: Mutex<LruCache<String, Channel>>
        = Mutex::new(LruCache::new(NonZeroUsize::new(CHANNEL_CACHE_SIZE).unwrap()));
}
