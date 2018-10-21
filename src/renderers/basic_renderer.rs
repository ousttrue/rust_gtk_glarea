use super::renderer;
use std::cell::RefCell;
use std::ffi::{CStr, CString};

///
/// port from https://github.com/kion-dgl/GtkGLArea/tree/master/02_add_opengl
///

pub struct BasicRenderer {
    program: RefCell<u32>,
    vao: RefCell<u32>,
    vbo_triangle: RefCell<u32>,
    //attribute_coord2d: RefCell<i32>,
}

unsafe fn check_shader_error(target: u32, status: u32) {
    let mut ok = gl::FALSE as i32;
    gl::GetShaderiv(target, status, (&mut ok) as &mut i32);
    if ok == 0 {
        let mut infoLen: i32 = 0;
        gl::GetShaderiv(target, gl::INFO_LOG_LENGTH, &mut infoLen as *mut i32);
        if infoLen > 1 {
            let mut infoLog = Vec::<u8>::with_capacity(infoLen as usize);
            //std::vector<char> infoLog(infoLen);
            gl::GetShaderInfoLog(
                target,
                infoLen,
                &mut infoLen as *mut i32,
                infoLog.as_mut_ptr() as *mut i8,
            );
            infoLog.set_len(infoLen as usize);
            let cs = CString::from_vec_unchecked(infoLog);
            println!("Error compiling shader: {}", cs.to_string_lossy());
        } else {
            println!("Error in shader");
        }
    //gl::DeleteShader(vs);
    } else {
        println!("compile success")
    }
}

///
/// ref https://github.com/nukep/rust-opengl-util/blob/master/shader.rs
///
unsafe fn compile_shader(source: &str, target: u32) -> u32 {
    let shader = gl::CreateShader(target);
    let ptr: *const u8 = source.as_bytes().as_ptr();
    let ptr_i8: *const i8 = std::mem::transmute(ptr);
    let len = source.len() as i32;
    gl::ShaderSource(shader, 1, &ptr_i8, &len);
    gl::CompileShader(shader);
    check_shader_error(shader, gl::COMPILE_STATUS);
    shader
}

unsafe fn link(vs: u32, fs: u32) -> u32 {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);

    let mut link_ok = gl::FALSE as i32;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut link_ok);

    if link_ok == 0 {
        //return;
        let mut len: i32 = 0;
        gl::GetShaderiv(program, gl::INFO_LOG_LENGTH, &mut len);
        if len > 1 {
            let mut buf = Vec::with_capacity(len as usize);
            let buf_ptr = buf.as_mut_ptr() as *mut gl::types::GLchar;
            unsafe {
                gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), buf_ptr);
                buf.set_len(len as usize);
            };

            let log = match String::from_utf8(buf) {
                Ok(log) => log,
                Err(vec) => panic!("Could not convert link log from buffer: {}", vec),
            };

            println!("Error link shader: {}", log);
        } else {
            println!("Error in link");
        }
    } else {
        println!("success link");
    }

    program
}

impl renderer::Renderer for BasicRenderer {
    fn new() -> Self {
        BasicRenderer {
            program: RefCell::new(0),
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

        unsafe {
            //
            // shader
            //
            let vs_source = "#version 420
in vec2 coord2d;
void main () {
   gl_Position = vec4(coord2d, 0.0, 1.0);
}
";
            let vs = compile_shader(vs_source, gl::VERTEX_SHADER);

            let fs_source = "
void main (void) {
   gl_FragColor[0] = 0.0;
   gl_FragColor[1] = 0.0;
   gl_FragColor[2] = 1.0;
}";

            let fs = compile_shader(fs_source, gl::FRAGMENT_SHADER);

            let program = link(vs, fs);
            self.program.replace(program);

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

            gl::UseProgram(*self.program.as_ptr());

            gl::BindVertexArray(*self.vao.as_ptr());

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
