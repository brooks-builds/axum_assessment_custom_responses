use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct JsonError {
    code: u16,
    message: String,
}

impl JsonError {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn code(&mut self, code: u16) -> &mut Self {
        self.code = code;
        self
    }

    pub fn message(&mut self, message: impl Into<String>) -> &mut Self {
        self.message = message.into();
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}
