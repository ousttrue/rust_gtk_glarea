pub trait Renderer
{
    fn new()->Self;
    fn initialize(&self);
    fn render(&self);
}
