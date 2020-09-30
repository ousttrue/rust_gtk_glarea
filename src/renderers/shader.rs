use super::renderer_error::RendererError;
use std::ffi::CString;

pub struct Shader {
    program: u32,
}

impl Drop for Shader {
    fn drop(&mut self) {
        if self.program != 0 {
            unsafe {
                gl::DeleteProgram(self.program);
            }
            println!("deleteProgram: {}", self.program);
            self.program = 0;
        }
    }
}

impl Shader {
    pub fn empty() -> Self {
        Shader { program: 0 }
    }

    pub fn new() -> Self {
        let program = unsafe { gl::CreateProgram() };
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
                    let mut buf = Vec::with_capacity(len as usize);
                    let buf_ptr = buf.as_mut_ptr() as *mut gl::types::GLchar;
                    gl::GetShaderInfoLog(target, len, std::ptr::null_mut(), buf_ptr);
                    buf.set_len(len as usize);
                    let s = String::from_utf8(buf).unwrap();
                    Err(RendererError::new(s.as_str()))
                } else {
                    Err(RendererError::new("shader compiler unknown error"))
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
            let r = Self::check_compile_error(shader);
            match r {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }
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
                    Err(RendererError::new("shader link with unknown error"))
                }
            } else {
                Ok(())
            }
        }
    }

    fn link(&self, vs: u32, fs: u32) -> Result<(), RendererError> {
        unsafe {
            gl::AttachShader(self.program, vs);
            gl::AttachShader(self.program, fs);
            gl::LinkProgram(self.program);
        }
        Self::check_link_error(self.program)?;
        Ok(())
    }

    pub fn compile(&self, vs_source: &str, fs_source: &str) -> Result<(), RendererError> {
        let vs = Self::compile_shader(vs_source, gl::VERTEX_SHADER)?;
        let fs = Self::compile_shader(fs_source, gl::FRAGMENT_SHADER)?;
        let r = self.link(vs, fs);
        r
    }

    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }
}
