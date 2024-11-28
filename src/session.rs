use crate::cache::BMBP_SESSION_CACHE;
use crate::{BmbpToken, BmbpUser};
use salvo::routing::get;
use std::ops::Add;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct BmbpSession {
    pub session_id: String,
    pub user_id: String,
    pub user_info: Option<BmbpUser>,
    pub user_token: Option<BmbpToken>,
    pub create_time: u32,
    pub refresh_time: u32,
    pub live_time: u32,
}

impl PartialEq<Self> for BmbpSession {
    fn eq(&self, other: &Self) -> bool {
        self.session_id == other.session_id
    }
}

pub struct BmbpAuthUtil;

impl BmbpAuthUtil {
    pub fn login_by_uid(user_id: String) -> BmbpToken {
        let now = std::time::SystemTime::now();
        let expire_at = now.add(Duration::from_secs(3600 * 2));
        let token_value = uuid::Uuid::new_v4().to_string();
        let token = BmbpToken {
            token: Some(token_value.clone()),
            refresh_token: Some(uuid::Uuid::new_v4().to_string()),
            expire_at: Some(
                expire_at
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u32,
            ),
        };
        let session = BmbpSession {
            session_id: token_value.clone(),
            user_id,
            user_info: None,
            user_token: Some(token.clone()),
            create_time: 0,
            refresh_time: 0,
            live_time: 0,
        };

        BMBP_SESSION_CACHE
            .write()
            .unwrap()
            .insert(token_value, Arc::new(session));
        token
    }
    pub fn login_by_token(user_id: String, token: BmbpToken) -> BmbpToken {
        token
    }
    pub fn logout_by_token(token: String) {
        let session_id = {
            let rw_lock = BMBP_SESSION_CACHE.read().unwrap();
            let session = rw_lock.get(token.as_str());
            if session.is_none() {
                return;
            }
            session.unwrap().session_id.clone()
        };
        BMBP_SESSION_CACHE
            .write()
            .unwrap()
            .remove(session_id.as_str());
    }
    pub fn get_token_value() -> String {
        "".to_string()
    }
    pub fn get_session_by_token(token: String) -> Option<Arc<BmbpSession>> {
        let rw_lock = BMBP_SESSION_CACHE.read().unwrap();
        let session = rw_lock.get(token.as_str()).clone();
        match session {
            Some(session) => Some(session.clone()),
            None => return None,
        }
    }
}
