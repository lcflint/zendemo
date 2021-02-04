#[derive(Clone, Debug, Copy)]
pub struct RawModel {
    vao_id: u32,
    vertex_count: i32,
}

//-----------------------

impl RawModel {
    pub fn new(vao_id: u32, vertex_count: i32) -> RawModel {
        RawModel {
            vao_id,
            vertex_count
        }
    }

    pub fn get_vao_id(&mut self) -> u32 {
        self.vao_id
    }

    pub fn get_vertex_count(&mut self) -> i32 {
        self.vertex_count
    }

    pub fn clean_model(&mut self) {
        // removes the model
        // unsafe {
        //     gl::DeleteVertexArrays(1, self.vao_id as *const u32);
        // }

        // drops the data
        drop(self);
    }
}