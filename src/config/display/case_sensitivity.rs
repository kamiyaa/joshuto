use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppErrorCode, AppResult};

/// How a search or filter should treat letter case when matching.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum CaseSensitivity {
    #[default]
    #[serde(rename = "sensitive")]
    Sensitive,
    #[serde(rename = "insensitive")]
    Insensitive,
    #[serde(rename = "smart")]
    Smart,
}

impl FromStr for CaseSensitivity {
    type Err = AppError;

    fn from_str(s: &str) -> AppResult<Self> {
        match s {
            "insensitive" => Ok(Self::Insensitive),
            "sensitive" => Ok(Self::Sensitive),
            "smart" => Ok(Self::Smart),
            value => {
                let error_msg = "Unknown CaseSensitivity value`";
                tracing::error!(value, "{error_msg}");
                Err(AppError::new(AppErrorCode::Parse, error_msg.to_string()))
            }
        }
    }
}
