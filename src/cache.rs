use crate::session::BmbpSession;
use crate::{BmbpAuthRbac, BmbpAuthToken, BmbpAuthUser};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};

pub(crate) static BMBP_AUTH_CACHE: LazyLock<RwLock<Option<Arc<Box<dyn BmbpAuthRbac>>>>> =
    LazyLock::new(|| RwLock::new(None));
pub(crate) static BMBP_AUTH_TOKEN_CACHE: LazyLock<RwLock<Option<Arc<Box<dyn BmbpAuthToken>>>>> =
    LazyLock::new(|| RwLock::new(None));
pub(crate) static BMBP_AUTH_USER_CACHE: LazyLock<RwLock<Option<Arc<Box<dyn BmbpAuthUser>>>>> =
    LazyLock::new(|| RwLock::new(None));
pub fn register_bmbp_auth_rbac(auth: Box<dyn BmbpAuthRbac>) {
    *((&*BMBP_AUTH_CACHE).write().unwrap()) = Some(Arc::new(auth));
}
pub fn register_bmbp_auth_user(auth_user: Box<dyn BmbpAuthUser>) {
    *((&*BMBP_AUTH_USER_CACHE).write().unwrap()) = Some(Arc::new(auth_user));
}
pub fn register_bmbp_auth_token(auth_token: Box<dyn BmbpAuthToken>) {
    *((&*BMBP_AUTH_TOKEN_CACHE).write().unwrap()) = Some(Arc::new(auth_token));
}

pub(crate) static BMBP_SESSION_CACHE: LazyLock<RwLock<HashMap<String, Arc<BmbpSession>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
