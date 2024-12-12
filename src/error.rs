use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
}

impl Display for BmbpAuthErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self.kind {
            BmbpAuthErrKind::NotLogin => format!("[{}]{}:{}", "0000", "未登录", self.message),
            BmbpAuthErrKind::TokenValid => format!("[{}]{}:{}", "0004", "token无效", self.message),
            BmbpAuthErrKind::TokenExpire => {
                format!("[{}]{}:{}", "0005", "token已过期", self.message)
            }
            BmbpAuthErrKind::UserNotFound => {
                format!("[{}]{}:{}", "0002", "用户不存在", self.message)
            }
            BmbpAuthErrKind::PasswordNotValid => {
                format!("[{}]{}:{}", "0003", "用户名或密码不正确", self.message)
            }
            BmbpAuthErrKind::NoImpl => format!("[{}]{}:{}", "0000", "缺少实现", self.message),
        };
        write!(f, "{}", str)
    }
}

pub type BmbpAuthResp<T> = Result<T, BmbpAuthErr>;
