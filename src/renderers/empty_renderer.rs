use super::renderer;

pub struct EmptyRenderer {}

impl renderer::Renderer for EmptyRenderer {
    fn new() -> Self {
        EmptyRenderer {}
    }

    fn initialize(&self) {
        unsafe {
            gl::ClearColor(1.0f32, 1.0f32, 0.5f32, 1.0f32);
        }
    }

    fn render(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}
