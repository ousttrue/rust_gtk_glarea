use super::renderer;
use super::renderer_error::RendererError;

pub struct EmptyRenderer {}

impl renderer::Renderer for EmptyRenderer {
    fn new() -> Self {
        EmptyRenderer {}
    }

    fn initialize(&self)->Result<(), RendererError> {
        Ok(())
    }

    fn render(&self) {
        unsafe {
            gl::ClearColor(1.0f32, 1.0f32, 0.5f32, 1.0f32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}
