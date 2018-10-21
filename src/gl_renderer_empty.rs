//use std::rc::Rc;
mod gl_renderer;

pub struct GlRenderer {
    program: u32,
    vao: u32,
    vbo_triangle: u32,
    attribute_coord2d: i32,
}

impl gl_renderer::GlRen for GlRenderer {
    pub fn new() -> GlRenderer {
        GlRenderer {
            program: 0,
            vao: 0,
            vbo_triangle: 0,
            attribute_coord2d: 0,
        }
    }

    pub fn initialize(&self) {
        /*
        let renderer = unsafe {
            let p = gl::GetString(gl::RENDERER);
            std::ffi::CString::from_raw(p)
        };
        */
        //let renderer = 
        //println!("Renderer: {}", renderer);

        //let version = gl::GetString(gl::VERSION);
        //printf("OpenGL version supported %s\n", version);

        unsafe{
            gl::ClearColor(0.5f32, 1.0f32, 0.5f32, 1.0f32);
        }

        /*
        glGenVertexArrays(1, &vao);
        glBindVertexArray(vao);
        
        GLfloat triangle_vertices[] = {
            0.0, 0.8,
            -0.8, -0.8,
            0.8, -0.8};
        glGenBuffers(1, &vbo_triangle);
        glBindBuffer(GL_ARRAY_BUFFER, vbo_triangle);
        glBufferData(
            GL_ARRAY_BUFFER,
            sizeof(triangle_vertices),
            triangle_vertices,
            GL_STATIC_DRAW);
        
        glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 0, 0);
        glEnableVertexAttribArray(0);
        glDisableVertexAttribArray(0);
        
        GLint compile_ok = GL_FALSE;
        GLint link_ok = GL_FALSE;
        
        const char *vs_source =
            "#version 130\n" // OpenGL 3
            "attribute vec2 coord2d; \n"
            "void main (void) { \n"
            "   gl_Position = vec4(coord2d, 0.0, 1.0); \n"
            "}";
        
        const char *fs_source =
            "#version 130\n" // OpenGL 3
            "void main (void) {\n"
            "   gl_FragColor[0] = 0.0;\n"
            "   gl_FragColor[1] = 0.0;\n"
            "   gl_FragColor[2] = 1.0;\n"
            "}";
        
        GLuint fs = glCreateShader(GL_FRAGMENT_SHADER);
        glShaderSource(fs, 1, &fs_source, nullptr);
        glCompileShader(fs);
        glGetShaderiv(fs, GL_COMPILE_STATUS, &compile_ok);
        if (!compile_ok)
        {
            //fprintf(stderr, "Error in fragment shader\n");
            return;
        }
        
        GLuint vs = glCreateShader(GL_VERTEX_SHADER);
        glShaderSource(vs, 1, &vs_source, nullptr);
        glCompileShader(vs);
        glGetShaderiv(vs, GL_COMPILE_STATUS, &compile_ok);
        if (!compile_ok)
        {
            //fprintf(stderr, "Error in vertex shader\n");
            return;
        }
        
        program = glCreateProgram();
        glAttachShader(program, vs);
        glAttachShader(program, fs);
        glLinkProgram(program);
        glGetProgramiv(program, GL_LINK_STATUS, &link_ok);
        if (!link_ok)
        {
            //fprintf(stderr, "Error when linking program\n");
            return;
        }
        
        const char *attribute_name = "coord2d";
        attribute_coord2d = glGetAttribLocation(program, attribute_name);
        if (attribute_coord2d == -1)
        {
            //fprintf(stderr, "Could not bind attribute %s\n", attribute_name);
            return;
        }
        */
    }

    pub fn render(&self) {

        unsafe{
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        /*
        
        glUseProgram(program);
        
        glBindVertexArray(vao);
        glEnableVertexAttribArray(attribute_coord2d);
        
        glBindBuffer(GL_ARRAY_BUFFER, vbo_triangle);
        glVertexAttribPointer(
            attribute_coord2d,
            2,
            GL_FLOAT,
            GL_FALSE,
            0,
            0);
        
        glDrawArrays(GL_TRIANGLES, 0, 3);
        glDisableVertexAttribArray(attribute_coord2d);
        */

    }
}
