use std::sync::{Arc};
use tokio::io::AsyncWriteExt;
pub use bean::*;
pub use err::*;
pub use util::*;
use crate::cache::{BMBP_AUTH_CACHE, BMBP_AUTH_TOKEN_CACHE, BMBP_AUTH_USER_CACHE};

mod bean;
mod cache;
mod err;
mod util;

/// bmbp_auth
/// # example
/// ```rust
///     use bmbp_auth::BmbpAuth;
///     pub struct BmbpAuthImpl;
///     impl BmbpAuth for BmbpAuthImpl {
///         ...
///     }
///     let auth = Box::new(BmbpAuthImpl::new());
///     bmbp_auth::register_bmbp_auth(auth);
/// ```
pub fn register_bmbp_auth(auth: Box<dyn BmbpAuth>) {
    *((&*BMBP_AUTH_CACHE).write().unwrap()) = Some(Arc::new(auth));
}

/// bmbp_auth
/// # example
/// ```rust
///     use bmbp_auth::{BmbpAuthToken};
///     pub struct BmbpAuthTokenImpl;
///     impl BmbpAuthToken for BmbpAuthTokenImpl {
///         ...
///     }
///     let auth = Box::new(BmbpAuthTokenImpl::new());
///     bmbp_auth::register_bmbp_auth_token(auth);
/// ```
pub fn register_bmbp_auth_user(auth_user: Box<dyn BmbpAuthUser>) {
    *((&*BMBP_AUTH_USER_CACHE).write().unwrap()) = Some(Arc::new(auth_user));
}
/// bmbp_auth
/// # example
/// ```rust
///     use bmbp_auth::{ BmbpAuthUser};
///     pub struct BmbpAuthUserImpl;
///     impl BmbpAuthUser for BmbpAuthUserImpl {
///         ...
///     }
///     let user = Box::new(BmbpAuthUserImpl::new());
///     bmbp_auth::register_bmbp_auth_user(user);
/// ```
pub fn register_bmbp_auth_token(auth_token: Box<dyn BmbpAuthToken>) {
    *((&*BMBP_AUTH_TOKEN_CACHE).write().unwrap()) = Some(Arc::new(auth_token));
}
