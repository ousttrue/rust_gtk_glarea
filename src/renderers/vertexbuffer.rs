///
/// VertexBufferObject
///
pub struct Vbo {
    vbo: u32,
}

impl Drop for Vbo {
    fn drop(&mut self) {
        if self.vbo != 0 {
            unsafe {
                gl::DeleteBuffers(1, &self.vbo);
            }
            self.vbo = 0;
        }
    }
}

impl Vbo {
    pub fn new() -> Self {
        let mut vbo: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        Vbo { vbo }
    }

    pub fn activate(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    pub fn assign(&self, values: &[f32]) {
        self.activate();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(values) as isize,
                values.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );
        }
    }
}

///
/// VertexArrayObject
///
pub struct Vao {
    vao: u32,
    buffers: Vec<Vbo>,
}

impl Drop for Vao {
    fn drop(&mut self) {
        if self.vao != 0 {
            unsafe {
                gl::DeleteVertexArrays(1, &self.vao);
            }
            self.vao = 0;
        }
    }
}

impl Vao {
    pub fn empty() -> Self {
        Vao {
            vao: 0,
            buffers: Vec::new(),
        }
    }

    pub fn new() -> Self {
        let mut vao: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        Vao {
            vao,
            buffers: Vec::new(),
        }
    }

    pub fn activate(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn append(&mut self, vbo: Vbo) {
        vbo.activate();
        let attribute = self.buffers.len() as u32;
        self.buffers.push(vbo);

        self.activate();
        unsafe {
            gl::EnableVertexAttribArray(attribute);
            gl::VertexAttribPointer(attribute, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        }
    }

    pub fn draw(&self) {
        self.activate();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
