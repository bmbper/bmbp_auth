use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::sync::PoisonError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmbpAuthErr {
    pub kind: BmbpAuthErrKind,
    pub message: String,
}

impl BmbpAuthErr {
    pub fn no_impl(message: String) -> Self {
        BmbpAuthErr {
            kind: BmbpAuthErrKind::NoImpl,
            message,
        }
    }
    pub fn lock_err(message: String) -> Self {
        BmbpAuthErr {
            kind: BmbpAuthErrKind::LockError,
            message,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BmbpAuthErrKind {
    NotLogin,
    TokenValid,
    TokenExpire,
    UserNotFound,
    PasswordNotValid,
    NoImpl,
    LockError,
}

impl Display for BmbpAuthErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self.kind {
            BmbpAuthErrKind::NotLogin => format!("[{}]{}:{}", "1001", "未登录", self.message),
            BmbpAuthErrKind::TokenValid => format!("[{}]{}:{}", "1004", "token无效", self.message),
            BmbpAuthErrKind::TokenExpire => {
                format!("[{}]{}:{}", "1005", "token已过期", self.message)
            }
            BmbpAuthErrKind::UserNotFound => {
                format!("[{}]{}:{}", "1002", "用户不存在", self.message)
            }
            BmbpAuthErrKind::PasswordNotValid => {
                format!("[{}]{}:{}", "1003", "用户名或密码不正确", self.message)
            }
            BmbpAuthErrKind::NoImpl => format!("[{}]{}:{}", "1000", "缺少实现", self.message),
            BmbpAuthErrKind::LockError => format!("[{}]{}:{}", "0000", "获取锁失败", self.message),
        };
        write!(f, "{}", str)
    }
}

impl<T> From<PoisonError<T>> for BmbpAuthErr {
    fn from(err: PoisonError<T>) -> Self {
        BmbpAuthErr::lock_err(err.to_string())
    }
}

pub type BmbpAuthResp<T> = Result<T, BmbpAuthErr>;
