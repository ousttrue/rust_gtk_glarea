use std::cell::RefCell;
use std::ffi::CStr;
use super::renderer_error::RendererError;
use super::renderer;
use super::shader::Shader;
use super::vertexbuffer::{Vao, Vbo};

//
// shader
//
const VS_SOURCE: &'static str = "#version 420
in vec2 coord2d;
void main () {
   gl_Position = vec4(coord2d, 0.0, 1.0);
}
";

const FS_SOURCE: &'static str = "#version 420
void main (void) {
   gl_FragColor[0] = 0.0;
   gl_FragColor[1] = 0.0;
   gl_FragColor[2] = 1.0;
}";


struct Scene
{
    shader: Shader,
    vao: Vao,
}

impl Scene {
    pub fn new()->Self{
        Scene {
            shader: Shader::empty(),
            vao: Vao::empty()
        }
    }

    pub fn initialize(&mut self)->Result<(), RendererError>
    {
        //
        // shader
        //
        self.shader = Shader::new();
        self.shader.compile(VS_SOURCE, FS_SOURCE)?;

        //
        // vao
        //
        self.vao = Vao::new();
    
        let vbo = Vbo::new();
        let vertices: [f32; 6] = [0.0, 0.8, -0.8, -0.8, 0.8, -0.8];
        vbo.assign(&vertices);
        self.vao.append(vbo);

        Ok(())
    }

    pub fn draw(&self)
    {
        unsafe {
            gl::ClearColor(0.5f32, 1.0f32, 0.5f32, 1.0f32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.shader.activate();
        self.vao.draw();
    }
}


pub struct BasicRenderer {
    scene: RefCell<Scene>,
}


impl renderer::Renderer for BasicRenderer {
    fn new() -> Self {
        BasicRenderer {
            scene: RefCell::new(Scene::new())
        }
    }

    fn initialize(&self)->Result<(), RendererError> {
        let renderer = unsafe {
            let p = gl::GetString(gl::RENDERER);
            CStr::from_ptr(p as *const i8)
        };
        println!("Renderer: {}", renderer.to_string_lossy());

        let version = unsafe {
            let p = gl::GetString(gl::VERSION);
            CStr::from_ptr(p as *const i8)
        };
        println!("OpenGL version supported: {}", version.to_string_lossy());

        self.scene.borrow_mut().initialize()
    }

    fn render(&self) {
        self.scene.borrow().draw();
    }
}
