#[derive(Debug, Clone)]
pub struct RendererError {
    pub message: String,
}

impl RendererError {
    pub fn new(message: &str) -> Self {
        RendererError {
            message: message.to_string(),
        }
    }
}
