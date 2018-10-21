use super::renderer;

pub struct EmptyRenderer {}

impl renderer::Renderer for EmptyRenderer {
    fn new() -> Self {
        EmptyRenderer {}
    }

    fn render(&self) {
        unsafe {
            gl::ClearColor(1.0f32, 1.0f32, 0.5f32, 1.0f32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}
