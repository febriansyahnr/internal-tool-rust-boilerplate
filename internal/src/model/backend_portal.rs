use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: String,
    pub message: String,
    pub data: T,
}

impl<T> Response<T> {
    pub fn new(code: String, message: String, data: T) -> Self {
        Self {
            code,
            message,
            data,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReconItem {
    pub uuid: String,
    pub originalName: String,
    pub status: String,
}
