use std::ffi::CString;
use super::renderer_error::RendererError;

pub struct Shader {
    pub program: u32,
}

impl Shader {
    pub fn new(program: u32) -> Self {
        Shader { program }
    }

    ///
    /// ref https://github.com/nukep/rust-opengl-util/blob/master/shader.rs
    ///
    fn check_compile_error(target: u32) -> Result<(), RendererError> {
        unsafe {
            let mut ok: i32 = 0;
            gl::GetShaderiv(target, gl::COMPILE_STATUS, &mut ok);
            if ok == 0 {
                let mut len: i32 = 0;
                gl::GetShaderiv(target, gl::INFO_LOG_LENGTH, &mut len);
                if len > 1 {
                    let mut buf = Vec::<u8>::with_capacity(len as usize);
                    gl::GetShaderInfoLog(
                        target,
                        buf.len() as i32,
                        &mut len,
                        buf.as_mut_ptr() as *mut i8,
                    );
                    buf.set_len(len as usize);
                    let cs = CString::from_vec_unchecked(buf);
                    Err(RendererError::new(&cs.to_string_lossy()))
                } else {
                    Err(RendererError::new("shader compile error"))
                }
            } else {
                Ok(())
            }
        }
    }

    fn compile_shader(source: &str, target: u32) -> Result<u32, RendererError> {
        unsafe {
            let shader = gl::CreateShader(target);
            let ptr = source.as_bytes().as_ptr() as *const i8;
            let len = source.len() as i32;
            gl::ShaderSource(shader, 1, &ptr, &len);
            gl::CompileShader(shader);
            Self::check_compile_error(shader)?;
            Ok(shader)
        }
    }

    fn check_link_error(program: u32) -> Result<(), RendererError> {
        unsafe {
            let mut link_ok: i32 = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut link_ok);
            if link_ok == 0 {
                let mut len: i32 = 0;
                gl::GetShaderiv(program, gl::INFO_LOG_LENGTH, &mut len);
                if len > 1 {
                    let mut buf = Vec::with_capacity(len as usize);
                    gl::GetProgramInfoLog(
                        program,
                        buf.len() as i32,
                        &mut len,
                        buf.as_mut_ptr() as *mut i8,
                    );
                    buf.set_len(len as usize);
                    let cs = CString::from_vec_unchecked(buf);
                    Err(RendererError::new(&cs.to_string_lossy()))
                } else {
                    Err(RendererError::new("shader link error"))
                }
            } else {
                Ok(())
            }
        }
    }

    fn link(vs: u32, fs: u32) -> Result<u32, RendererError> {
        let program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);
        }
        Self::check_link_error(program)?;
        Ok(program)
    }

    pub fn create(vs_source: &str, fs_source: &str) -> Result<Self, RendererError> {
        let vs = Self::compile_shader(vs_source, gl::VERTEX_SHADER)?;
        let fs = Self::compile_shader(fs_source, gl::FRAGMENT_SHADER)?;
        let program = Self::link(vs, fs)?;
        Ok(Self::new(program))
    }
}
