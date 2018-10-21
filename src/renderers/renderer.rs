use super::renderer_error::RendererError;

pub trait Renderer {
    fn new() -> Self;
    fn initialize(&self) -> Result<(), RendererError>
    {
        Ok(())
    }
    fn resize(&self, w: u32, h: u32) {
        println!("resize: {}x{}", w, h);
    }

    fn render(&self);
}
