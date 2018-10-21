#[derive(Debug, Clone)]
pub struct RendererError {
    message: String,
}

impl RendererError {
    pub fn new(message: &str) -> Self {
        RendererError {
            message: message.to_string(),
        }
    }
}
