use std::sync::{Arc, LazyLock, OnceLock, RwLock};
use crate::{BmbpAuth, BmbpAuthToken, BmbpAuthUser};

pub(crate) static BMBP_AUTH_CACHE: LazyLock<RwLock<Option<Arc<Box<dyn BmbpAuth>>>>> = LazyLock::new(|| {
    RwLock::new(None)
});
pub(crate)  static BMBP_AUTH_TOKEN_CACHE: LazyLock<RwLock<Option<Arc<Box<dyn BmbpAuthToken>>>>> = LazyLock::new(|| {
    RwLock::new(None)
});
pub(crate)  static BMBP_AUTH_USER_CACHE: LazyLock<RwLock<Option<Arc<Box<dyn BmbpAuthUser>>>>> = LazyLock::new(|| {
    RwLock::new(None)
});