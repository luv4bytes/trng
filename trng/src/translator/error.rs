#[derive(Debug)]
pub enum TranslatorErrorType {
    NotSupported,
    Io,
}

#[derive(Debug)]
pub struct TranslatorError {
    pub _type: TranslatorErrorType,
    pub description: String,
}

impl TranslatorError {
    pub fn new(_type: TranslatorErrorType, description: String) -> TranslatorError {
        Self { _type, description }
    }
}

impl From<std::io::Error> for TranslatorError {
    fn from(value: std::io::Error) -> Self {
        Self::new(TranslatorErrorType::Io, value.to_string())
    }
}
