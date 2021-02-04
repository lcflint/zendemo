use std::fs::File;
use std::io::Read;

use std::path::Path;
use std::ffi::CString;

use cgmath::{Matrix, Matrix4};

//-------------------------

// standard class for shader programs
pub struct ShaderProgram {
    program_id: u32,
    vertex_shader: u32,
    fragment_shader: u32
}

//-------------------------

impl ShaderProgram {

    pub fn new(vertex_str: &str, fragment_str: &str) -> ShaderProgram {
        // creates a new vertex shader
        let vertex_path = Path::new(vertex_str);
        let vertex_shader = 
            ShaderProgram::read_shader_code(vertex_path, gl::VERTEX_SHADER).unwrap();

        // creates a new fragment shader
        let frag_path = Path::new(fragment_str);
        let fragment_shader = 
            ShaderProgram::read_shader_code(frag_path, gl::FRAGMENT_SHADER).unwrap();

        // creates a new shader program
        let program_id : u32;

        unsafe {
            program_id = gl::CreateProgram();
        }

        // creates the shader program
        let mut shader_program = ShaderProgram {
            program_id,
            vertex_shader,
            fragment_shader
        };

        // links the shaders to the opengl program
        unsafe {
            gl::AttachShader(program_id, vertex_shader);
            gl::AttachShader(program_id, fragment_shader);
        }

        unsafe {
            // links and validates the shader program
            gl::LinkProgram(program_id);
            gl::ValidateProgram(program_id);
        }
        
        shader_program
    }

    //--------------------------

    // binds attributes to the shader
    pub fn bind_attribute(&mut self, attribute: u32, attribute_name: &str) {
        // converts the input string into a ptr name
        let c_name = CString::new(attribute_name).unwrap();
        let ptr = c_name.as_ptr();

        // binds to a particular attribute location
        unsafe {
            gl::BindAttribLocation(
                self.program_id,
                attribute,
                ptr
            );
        }
    }

    // gets the location of a uniform variable in the shader program
    pub fn get_uniform_location(&mut self, name: &str) -> i32 {
        let uniform_location: i32;

        // converts the input string into a ptr name
        let c_name = CString::new(name).unwrap();
        let ptr = c_name.as_ptr();

        // queries gl for the location
        unsafe {
            uniform_location = gl::GetUniformLocation(
                self.program_id,
                ptr
            );
        }

        uniform_location
    }

    //--------------------------

    // loads shader code from a file
    fn read_shader_code(
        shader_path: &Path,
        shader_type: u32
    ) -> Result<gl::types::GLuint, String> {
        // reads the file text
        let mut shader_file = File::open(shader_path).unwrap();

        // reads the shader file
        let mut shader_string = String::new();
        shader_file.read_to_string(&mut shader_string).unwrap();

        // creates a new source string
        let source = CString::new(shader_string).unwrap();

        // initialises the shader id
        let shader_id: u32;

        unsafe {
            // creates a shader of a particular type
            shader_id = gl::CreateShader(shader_type);

            // reads the shader source, attaching it to the shader with this id
            gl::ShaderSource(shader_id, 1, &source.as_ptr(), std::ptr::null());

            // compiles the shader
            gl::CompileShader(shader_id);
        }

        // checks if the shader compiled successfully
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
        }

        // return an error if shader compilation was unsuccessful
        if success == 0 {
            let mut len: gl::types::GLint = 0;

            unsafe {
                gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = ShaderProgram::create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(
                    shader_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        // returns the shader id if everything was okay
        return Ok(shader_id)
    }

    //--------------------------

    // loads a float into a uniform buffer
    pub fn load_float(&mut self, location: i32, data: f32) {
        unsafe {
            gl::Uniform1f(location, data);
        }
    }

    // loads an integer into a uniform buffer
    pub fn load_integer(&mut self, location: i32, data: i32) {
        unsafe {
            gl::Uniform1i(location, data);
        }
    }

    // loads vectors up to a length of four into an array
    pub fn load_vector(&mut self, location: i32, data: Vec<f32>) {
        println!("{}", location);
        unsafe {
            match data.len() {
                2 => gl::Uniform2f(
                    location,
                    *data.get(0).unwrap(),
                    *data.get(1).unwrap()
                ),
                3 => gl::Uniform3f(
                    location,
                    *data.get(0).unwrap(),
                    *data.get(1).unwrap(),
                    *data.get(2).unwrap()
                ),
                4 => gl::Uniform4f(
                    location,
                    *data.get(0).unwrap(),
                    *data.get(1).unwrap(),
                    *data.get(2).unwrap(),
                    *data.get(3).unwrap()
                ),
                _ => ()
            };
        }
    }

    // loads up a 4x4 matrix into the uniform variable
    pub fn load_matrix(&mut self, location: i32, data: Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                location,
                1, gl::FALSE,
                data.as_ptr()
            )
        }
    }

    //--------------------------

    // starts the current shader
    pub fn start(&mut self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    // stops the current shader
    pub fn stop(&mut self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    // cleans up the shader
    pub fn clean_up(&mut self) {
        // stops the shader
        self.stop();

        // cleans up the shaders
        unsafe {
            // detaches the vertex and fragment shaders
            gl::DetachShader(self.program_id, self.vertex_shader);
            gl::DetachShader(self.program_id, self.fragment_shader);

            // deletes the shaders
            gl::DeleteShader(self.vertex_shader);
            gl::DeleteShader(self.fragment_shader);

            // deletes the program
            gl::DeleteProgram(self.program_id);
        }
    }

    //--------------------------

    fn create_whitespace_cstring_with_len(len: usize) -> CString {
        // allocate buffer of correct size
        let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);

        // fill it with len spaces
        buffer.extend([b' '].iter().cycle().take(len));

        // convert buffer to CString
        unsafe { CString::from_vec_unchecked(buffer) }
    }
}