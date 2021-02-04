use std::collections::HashMap;

use cgmath::*;

use crate::{
    models::raw_model::RawModel,
    rendering::loader::Loader,
    utils::constants::*
};

use super::{
    basic_marching_cubes::MCChunk,
    camera::Camera,
    table_reader::TriangulationTable
};

pub struct ChunkManager {
    chunk_data: HashMap<Vector3<i32>, MCChunk>,
    model_data: HashMap<Vector3<i32>, RawModel>
}

//---------------------------

impl ChunkManager {
    pub fn new() -> ChunkManager {
        ChunkManager {
            chunk_data: HashMap::new(),
            model_data: HashMap::new()
        }
    }

    pub fn generate_chunk_models(
        &mut self, 
        camera: &Camera, 
        loader: &mut Loader, 
        triangulation_table: &TriangulationTable
    ) -> Vec<RawModel> {
        // gets the position of the camera
        let camera_position = camera.position;

        // based on the cam position, gets a gridspace that the camera is in
        let camera_gridspace_f = (camera_position/SPACING_PER_VERTEX)/((CHUNK_BOUNDS - 1) as f32);

        // floors the camera position
        let camera_gridspace = Vector3::new(
            camera_gridspace_f.x.floor() as i32,
            camera_gridspace_f.y.floor() as i32,
            camera_gridspace_f.z.floor() as i32
        );

        // cleans the models outside of the chunk space by removing their VAOs
        for (offset, model) in self.model_data.iter_mut() {
            if ChunkManager::check_offset_out_of_gridspace(offset, &camera_gridspace) {
                model.clean_model();
            }
        }

        // remove models outside of gridspace
        self.model_data.retain(|key, value| {
            !ChunkManager::check_offset_out_of_gridspace(key, &camera_gridspace)
        });

        // repeat with chunks (TEMPORARY MEMORY SAVING)
        self.chunk_data.retain(|key, value| {
            !ChunkManager::check_offset_out_of_gridspace(key, &camera_gridspace)
        });

        // initialises a models vector
        let mut models = Vec::<RawModel>::new();

        // based on the camera's gridspace position, loop and generate new chunks
        for x in (camera_gridspace.x - CHUNK_VIEW_LIMIT)..(camera_gridspace.x + CHUNK_VIEW_LIMIT) {
            for y in (camera_gridspace.y - CHUNK_VIEW_LIMIT)..(camera_gridspace.y + CHUNK_VIEW_LIMIT) {
                for z in (camera_gridspace.z - CHUNK_VIEW_LIMIT)..(camera_gridspace.z + CHUNK_VIEW_LIMIT) {

                    // get offset for curr chunk
                    let curr_offset = Vector3::new(x, y, z);

                    // check if the chunk exists
                    let get_chunk = self.chunk_data.get(&curr_offset);

                    match get_chunk {
                        Some(chunk) => {
                            // check if the model exists
                            let get_model = self.model_data.get(&curr_offset);

                            match get_model {
                                Some(model) => {
                                    models.push(*model);
                                }
                                None => {
                                    // uses the loader to create a model
                                    let model = loader.load_to_vao(&chunk.vertices, &chunk.indices);
                                    
                                    models.push(model);

                                    self.model_data.insert(curr_offset, model);
                                }
                            }
                        }
                        None => {
                            // creates a new chunk
                            let new_chunk = MCChunk::new(
                                curr_offset,
                                triangulation_table
                            );

                            // uses the loader to create a model
                            let model = loader.load_to_vao(&new_chunk.vertices, &new_chunk.indices);
                            
                            models.push(model);

                            self.chunk_data.insert(curr_offset, new_chunk);
                            self.model_data.insert(curr_offset, model);
                        }
                    };
                }
            }
        }

        models
    }

    // gets whether the offset satisfies being within the gridspace
    fn check_offset_out_of_gridspace(offset: &Vector3<i32>, camera_gridspace: &Vector3<i32>) -> bool {
        offset.x > camera_gridspace.x + CHUNK_VIEW_LIMIT ||
        offset.x < camera_gridspace.x - CHUNK_VIEW_LIMIT ||
        offset.y > camera_gridspace.y + CHUNK_VIEW_LIMIT ||
        offset.y < camera_gridspace.y - CHUNK_VIEW_LIMIT ||
        offset.z > camera_gridspace.z + CHUNK_VIEW_LIMIT ||
        offset.z < camera_gridspace.z - CHUNK_VIEW_LIMIT
    }
}