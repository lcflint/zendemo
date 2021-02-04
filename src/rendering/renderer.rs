
use super::super::{
    shaders::static_shader::*,
    models::raw_model::*,
    utils::math::*,
    entities::camera::*
};

use cgmath::{Deg, Matrix4, SquareMatrix, Vector3};

use std::ptr;

//----------------------

pub struct Renderer {
    pub shader: StaticShader,

    bloggus: f32
}

//-----------------------

impl Renderer {
    // creates a renderer
    pub fn new() -> Renderer {
        // creates a new shader
        let mut shader = StaticShader::new();

        // starts the shader
        shader.shader_program.start();
                
        // generates a projection matrix
        let projection_matrix = Renderer::generate_projection_matrix();

        // injects the projection matrix
        shader.set_projection(projection_matrix);

        // stops the shader program
        shader.shader_program.stop();

        // enables backface culling
        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
        }

        // creates the renderer
        Renderer {
            shader,
            bloggus: 0.0
        }
    }

    //-----------------------

    pub fn render(&mut self, models: &mut Vec<RawModel>, camera: &mut Camera) {
        // prepares the renderer
        self.prepare();

        // starts the shader
        self.shader.shader_program.start();
        
        // generates a projection matrix
        let projection_matrix = Renderer::generate_projection_matrix();

        // injects the projection matrix
        self.shader.set_projection(projection_matrix);

        // gets a view matrix from the camera
        let view_matrix = camera.get_view_matrix();

        // injects the view matrix
        self.shader.set_view(view_matrix);

        // TEST STUFF
        //--------------------------
        //self.bloggus = self.bloggus + 0.001;

        let t_matrix = generate_transformation_matrix(
            Vector3::new(0.0, 0.0, -2.0),
            Vector3::new(0.0, self.bloggus, self.bloggus/2.0),
            1.0
        );

        self.shader.set_transformation(t_matrix);
        //--------------------------

        // iterates over the models in the model array
        for model in models.iter_mut() {
            // binds the rawmodel
            self.bind_model(model);

            // renders the model
            unsafe {
                gl::DrawElements(
                    gl::TRIANGLES,
                    model.get_vertex_count(),
                    gl::UNSIGNED_INT,
                    ptr::null()
                );
            }

            // unbinds the model
            self.unbind_model();
        }

        // ends the shader
        self.shader.shader_program.stop();
    }

    //-----------------------

    // preparation function
    fn prepare(&self) {
        unsafe {
            // enables depth testing
            gl::Enable(gl::DEPTH_TEST);

            // clears the sky colour
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    //-----------------------

    fn bind_model(&mut self, model: &mut RawModel) {
        unsafe {
            // binds to the model's vao
            gl::BindVertexArray(model.get_vao_id());

            // enables the vertex array 0
            gl::EnableVertexAttribArray(0);
        }
    }

    fn unbind_model(&mut self) {
        unsafe {
            // disable the vertex array 0
            gl::DisableVertexAttribArray(0);

            // unbinds the model's vao
            gl::BindVertexArray(0);
        }
    }

    //-----------------------

    // helper to help generate a projection matrix
    fn generate_projection_matrix() -> Matrix4<f32> {
        // temp until a redraw function can be added
        let matrix = cgmath::perspective(
            Deg(70.0),
            800 as f32
                / 600 as f32,
            0.1,
            100.0
        );
        
        matrix
    }

    //-----------------------

    // cleanup will clean the renderer and destroy the shader
    pub fn clean_up(&mut self) {
        // removes the shader
        self.shader.shader_program.clean_up();
    }
}