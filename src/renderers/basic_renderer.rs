use std::cell::RefCell;
use std::ffi::CStr;
use super::renderer;
use super::shader::Shader;

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


pub struct BasicRenderer {
    shader: RefCell<Shader>,
    vao: RefCell<u32>,
    vbo_triangle: RefCell<u32>,
    //attribute_coord2d: RefCell<i32>,
}

impl renderer::Renderer for BasicRenderer {
    fn new() -> Self {
        BasicRenderer {
            shader: RefCell::new(Shader::new(0)),
            vao: RefCell::new(0),
            vbo_triangle: RefCell::new(0),
            //attribute_coord2d: RefCell::new(0),
        }
    }

    fn initialize(&self) {
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

        //
        // shader
        //
        let shader = Shader::create(VS_SOURCE, FS_SOURCE).unwrap();
        self.shader.replace(shader);

        unsafe {
            //
            // vao
            //
            gl::GenVertexArrays(1, self.vao.as_ptr() as *mut gl::types::GLuint);
            gl::BindVertexArray(*self.vao.as_ptr());

            let triangle_vertices: [f32; 6] = [0.0, 0.8, -0.8, -0.8, 0.8, -0.8];
            gl::GenBuffers(1, self.vbo_triangle.as_ptr() as *mut gl::types::GLuint);
            gl::BindBuffer(gl::ARRAY_BUFFER, *self.vbo_triangle.as_ptr());

            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&triangle_vertices) as isize,
                (&triangle_vertices).as_ptr() as *mut std::ffi::c_void,
                gl::STATIC_DRAW,
            );

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        }
    }

    fn render(&self) {
        unsafe {
            gl::ClearColor(0.5f32, 1.0f32, 0.5f32, 1.0f32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::UseProgram(self.shader.borrow().program);

            gl::BindVertexArray(*self.vao.as_ptr());

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
