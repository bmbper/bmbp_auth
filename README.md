# BMBP_AUTH
```rust
use bmbp_auth::BmbpAuth;
pub struct BmbpAuthImpl;
impl BmbpAuth for BmbpAuthImpl {
    ...
}
let auth = Box::new(BmbpAuthImpl::new());
bmbp_auth::register_bmbp_auth(auth);
```