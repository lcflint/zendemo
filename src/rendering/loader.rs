
use raw_model::RawModel;

use super::super::{
    models::raw_model
};

use std::convert::TryFrom;
use std::ptr;

//----------------------

pub struct Loader {
    vaos: Vec<u32>,
    vbos: Vec<u32>,
    textures: Vec<u32>
}

//----------------------

impl Loader {
    // defaults for the loader
    pub fn default() -> Loader {
        Loader {
            vaos: Vec::new(),
            vbos: Vec::new(),
            textures: Vec::new(),
        }
    }

    // loads a set of vertices to a VAO
    pub fn load_to_vao(
        &mut self,
        vertices: &Vec<f32>,
        indices: &Vec<u32>
    ) -> raw_model::RawModel {

        // gets the vertex count
        let vertex_count = (indices.len() as i32);

        // gets a vao id by creating a vao
        let vao_id = self.create_vao();

        // binds the index buffer
        self.bind_index_buffer(indices);

        // stores the vertices in an attribute list
        self.store_attb_list_data(0, vertices, 3);

        // unbinds the vao
        Loader::unbind_vao();

        // creates the raw model
        let raw_model = raw_model::RawModel::new(
            vao_id,
            vertex_count
        );

        raw_model
    }

    // creates a new vao
    fn create_vao(&mut self) -> u32 {
        // creates a new uint for use in generating a vao
        let mut vao_id = 0;

        // creates the new VAO and binds it
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
            gl::BindVertexArray(vao_id);
        }

        // adds the vao id to the vaos
        self.vaos.push(vao_id);

        // returns the vao id
        vao_id
    }

    // unbinds a vao to stop it from being used
    fn unbind_vao() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    //-------------------

    // stores a set of data into a VBO of a certain index
    fn store_attb_list_data(&mut self, attribute_num: u32, data: &Vec<f32>, coordinate_size: i32) {
        // creates a new uint for use in generating a vbo
        let mut vbo_id = 0;

        unsafe {
            // creates a new VBO
            gl::GenBuffers(1, &mut vbo_id);

            // binds the newly created VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);

            // generates buffer data that opengl can use
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                attribute_num,
                coordinate_size,
                gl::FLOAT,
                gl::FALSE,
                0,
                ptr::null()
            );

            // unbinds the buffer
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        // adds the vbo id to the vaos
        self.vbos.push(vbo_id);
    }

    // function specifically for generating an index buffer
    fn bind_index_buffer(&mut self, data: &Vec<u32>) {
        // creates a new uint for use in generating a vbo
        let mut vbo_id = 0;

        unsafe {
            // creates a new VBO
            gl::GenBuffers(1, &mut vbo_id);

            // binds the newly created VBO
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbo_id);

            // generates buffer data that opengl can use
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }

        // adds the vbo id to the vaos
        self.vbos.push(vbo_id);
    }

    //-------------------

    pub fn clean_up(&mut self) {
        // loops over the vaos, vbos and textures for cleanup
        unsafe {
            for vao in self.vaos.iter() {
                gl::DeleteVertexArrays(1, vao);
            }

            for vbo in self.vbos.iter() {
                gl::DeleteBuffers(1, vbo);
            }

            for texture in self.textures.iter() {
                gl::DeleteTextures(1, texture);
            }
        }
    }
}