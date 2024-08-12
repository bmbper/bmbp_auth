use bmbp_marco_bean::bean;
use serde::{Deserialize, Serialize};

#[bean]
pub struct BmbpAuthErr {
    pub(crate) code: Option<String>,
    pub(crate) msg: Option<String>,
    pub(crate) kind: BmbpAuthErrType,
}

impl BmbpAuthErr {
    pub fn build(kind: BmbpAuthErrType, code: String, msg: String) -> Self {
        BmbpAuthErr {
            code: Some(code),
            msg: Some(msg),
            kind,
        }
    }
}

#[derive(Debug,Default,Clone,Serialize,Deserialize)]
pub enum BmbpAuthErrType {
    #[default]
    NotFoundImpl
}

pub type BmbpAuthResp<T> = Result<T, BmbpAuthErr>;
