use std::collections::HashMap;

use super::base_shader::*;

use cgmath::{Matrix, Matrix4};

//------------------------

pub struct StaticShader {
    pub shader_program: ShaderProgram,
    pub uniform_locations: HashMap<String, i32>
}

const VERTEX_SHADER: &str = "assets/shaders/static_shader.vert";
const FRAGMENT_SHADER: &str = "assets/shaders/static_shader.frag";

//------------------------

impl StaticShader {
    pub fn new() -> StaticShader {
        // creates a new shader program
        let shader_program = ShaderProgram::new(VERTEX_SHADER, FRAGMENT_SHADER);

        // creates an empty hashmap
        let uniform_locations = HashMap::new();

        let mut static_shader = StaticShader {
            shader_program,
            uniform_locations
        };

        // binds the attributes and gets the locations of everything
        static_shader.bind_attributes();
        static_shader.get_uniform_locations();

        static_shader
    }

    //-----------------------

    pub fn bind_attributes(&mut self) {
        // binds the first VAO list to position
        self.shader_program.bind_attribute(
            0,
            "position"
        );
    }
    
    //-----------------------

    // add to the uniform locations hashmap
    pub fn get_uniform_locations(&mut self) {
        self.uniform_locations.insert(
            String::from("transform_matrix"),
            self.shader_program.get_uniform_location("transform_matrix")
        );

        self.uniform_locations.insert(
            String::from("projection_matrix"),
            self.shader_program.get_uniform_location("projection_matrix")
        );

        self.uniform_locations.insert(
            String::from("view_matrix"),
            self.shader_program.get_uniform_location("view_matrix")
        );
    }
    
    //-----------------------

    pub fn set_transformation(&mut self, data: Matrix4<f32>) {
        let location = self.uniform_locations.get(
            &String::from("transform_matrix")
        ).unwrap();

        self.shader_program.load_matrix(*location, data);
    }

    pub fn set_projection(&mut self, data: Matrix4<f32>) {
        let location = self.uniform_locations.get(
            &String::from("projection_matrix")
        ).unwrap();

        self.shader_program.load_matrix(*location, data);
    }

    pub fn set_view(&mut self, data: Matrix4<f32>) {
        let location = self.uniform_locations.get(
            &String::from("view_matrix")
        ).unwrap();

        self.shader_program.load_matrix(*location, data);
    }
}