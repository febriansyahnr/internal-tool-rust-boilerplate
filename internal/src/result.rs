use derive_more::From;
use serde::{Deserialize, Serialize};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    GeneralError {
        message: String,
    },

    // -- External errors
    #[from]
    IOError(std::io::Error),
    #[from]
    ReqwestError(reqwest::Error),

    // Note: Only placeholder, need to remove this part to give more context
    #[from]
    SerdeJson(serde_json::Error),
    #[from]
    SerdeYaml(serde_yaml::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: String,
    pub message: String,
    pub data: Option<T>,
}

impl<T> Response<T> {
    pub fn success(&mut self, message: &str) {
        self.code = "00".to_string();
        self.message = message.to_string();
    }
    pub fn with_data(&mut self, data: T) {
        self.data = Some(data);
    }
}

impl<T> Default for Response<T> {
    fn default() -> Self {
        Self {
            code: "".to_string(),
            message: "".to_string(),
            data: None,
        }
    }
}
