use std::num::NonZeroUsize;

use lru::LruCache;
use tokio::sync::Mutex;

use crate::consts::MESSAGE_CACHE_SIZE;
use crate::prelude::Message;

lazy_static::lazy_static! {
    pub(crate) static ref MESSAGE_CACHE: Mutex<LruCache<String, Message>> 
        = Mutex::new(LruCache::new(NonZeroUsize::new(MESSAGE_CACHE_SIZE).unwrap()));
}
