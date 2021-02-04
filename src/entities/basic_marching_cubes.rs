
// generates for basic marching cubes
use core::f64;

use cgmath::*;

use ndarray::{Array3};

use noise::{NoiseFn, Worley};

use crate::{
    entities::table_reader::TriangulationTable,
    utils::constants::*
};

//----------------------

#[derive(Clone, Debug, Copy)]
enum Mask {
    MINUS = 0,
    PLUS = 1
}

pub struct MCChunk {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,

    grid: Array3<Mask>,
    grid_weights: Array3<f32>,

    offset: Vector3<i32>,

    spacing_per_vertex: f32,
    chunk_bounds: usize,
    isovalue: f32,
}

//----------------------

impl MCChunk {
    // generates a new MCChunk
    pub fn new(
        offset: Vector3<i32>,
        tri_table: &TriangulationTable
    ) -> MCChunk {
        // returns a chunk with empty vertices and defaults
        let mut mc_chunk = MCChunk {
            vertices: Vec::new(),
            indices: Vec::new(),

            grid: Array3::<Mask>::from_elem((CHUNK_BOUNDS, CHUNK_BOUNDS, CHUNK_BOUNDS), Mask::MINUS),
            grid_weights: Array3::<f32>::zeros((CHUNK_BOUNDS, CHUNK_BOUNDS, CHUNK_BOUNDS)),

            offset,

            spacing_per_vertex: SPACING_PER_VERTEX,
            chunk_bounds: CHUNK_BOUNDS,
            isovalue: TEST_ISOVALUE
        };

        // labels a set of isovertices using a sampler
        mc_chunk.label_initial_vertices();

        // generates vertices for this particular chunk
        mc_chunk.generate_vertices(tri_table);

        mc_chunk
    }

    //--------------------------

    // generates densities for the first vertices
    fn label_initial_vertices(&mut self) {
        // temp perlin sampler
        let perlin = Worley::new();

        // calculates an offset for the perlin based on the chunk
        // spacing and the space between vertices
        // (ugly f64 conversion, not sure if dirty code?)
        let perlin_offset = Vector3::new(
            self.offset.x as f64 * self.chunk_bounds as f64 - self.offset.x as f64,
            self.offset.y as f64 * self.chunk_bounds as f64 - self.offset.y as f64,
            self.offset.z as f64 * self.chunk_bounds as f64 - self.offset.z as f64,
        );

        // iterates over the data to assign noise values
        for x in 0..CHUNK_BOUNDS {
            for y in 0..CHUNK_BOUNDS {
                for z in 0..CHUNK_BOUNDS {
                    // gets the perlin value for the point
                    let perlin_value = perlin.get([
                        (perlin_offset.x + x as f64)/CHUNK_BOUNDS as f64,
                        (perlin_offset.y + y as f64)/CHUNK_BOUNDS as f64,
                        (perlin_offset.z + z as f64)/CHUNK_BOUNDS as f64
                    ]);

                    // sets the weighted grid
                    self.grid_weights[[x, y, z]] = perlin_value as f32;

                    // if the perlin value is over the isovalue, set the mask
                    // to PLUS, else to minus
                    if perlin_value as f32 >= self.isovalue {
                        self.grid[[x, y, z]] = Mask::PLUS;
                    }
                    else {
                        self.grid[[x, y, z]] = Mask::MINUS;
                    }
                }
            }
        }
    }

    // generates vertices from the labelled vertices
    fn generate_vertices(&mut self, tri_table: &TriangulationTable) {

        // creates tables for vertices and indices
        let mut vertices_table = Vec::<f32>::new();
        let mut indices_table = Vec::<u32>::new();

        // generates a full offset for the position of the cubes
        let full_offset = Vector3::new(
            (self.offset.x as f32 * self.chunk_bounds as f32 * self.spacing_per_vertex) - (self.offset.x as f32 * self.spacing_per_vertex),
            (self.offset.y as f32 * self.chunk_bounds as f32 * self.spacing_per_vertex) - (self.offset.y as f32 * self.spacing_per_vertex),
            (self.offset.z as f32 * self.chunk_bounds as f32 * self.spacing_per_vertex) - (self.offset.z as f32 * self.spacing_per_vertex)
        );

        // temporary indexing thing 
        let mut index: u32 = 0;

        // gets the cubes marching
        for x in 0..(CHUNK_BOUNDS - 1) {
            for y in 0..(CHUNK_BOUNDS - 1) {
                for z in 0..(CHUNK_BOUNDS - 1) {

                    // creates a bitwise representation of the cube that's 
                    // currently being marched on
                    let mut cube_config: u8 = 0;

                    // manual for now, replace later
                    cube_config |= (self.grid[[x, y, z]] as u8) << 0;
                    cube_config |= (self.grid[[x + 1, y, z]] as u8) << 1;
                    cube_config |= (self.grid[[x + 1, y, z + 1]] as u8) << 2;
                    cube_config |= (self.grid[[x, y, z + 1]] as u8) << 3;
                    
                    cube_config |= (self.grid[[x, y + 1, z]] as u8) << 4;
                    cube_config |= (self.grid[[x + 1, y + 1, z]] as u8) << 5;
                    cube_config |= (self.grid[[x + 1, y + 1, z + 1]] as u8) << 6;
                    cube_config |= (self.grid[[x, y + 1, z + 1]] as u8) << 7;

                    // if the cube offset is not 0 or 255 (empty or full), proceed
                    if cube_config != 0 && cube_config != 255 {

                        // gets the cube configuration vertices from the triangulation table
                        let cube_vertices = tri_table.table.get(&(cube_config as u16)).unwrap();

                        // loops over the cube vertices
                        for vert in cube_vertices.iter() {
                            // based on the indices, gets the corner configurations associated
                            // with the vert
                            let corner_a = *tri_table.corner_index_a_from_edge.get(*vert as usize).unwrap();
                            let corner_b = *tri_table.corner_index_b_from_edge.get(*vert as usize).unwrap();

                            // gets the sampled values at the corners that are being assessed
                            let corner_a_weight = self.grid_weights[[
                                x + corner_a.x as usize,
                                y + corner_a.y as usize,
                                z + corner_a.z as usize
                            ]];

                            let corner_b_weight = self.grid_weights[[
                                x + corner_b.x as usize,
                                y + corner_b.y as usize,
                                z + corner_b.z as usize
                            ]];

                            // interpolates between the grid weights
                            let interp_value = (self.isovalue - corner_a_weight) / (corner_b_weight - corner_a_weight);

                            // gets an interpolated value between the two corners
                            let interp_corners = corner_a + (interp_value * (corner_b - corner_a));

                            // calculates the vertex point from a number of different factors
                            let vertex_point = Vector3::new(
                                ((x as f32 + interp_corners.x) * self.spacing_per_vertex) + full_offset.x,
                                ((y as f32 + interp_corners.y) * self.spacing_per_vertex) + full_offset.y,
                                ((z as f32 + interp_corners.z) * self.spacing_per_vertex) + full_offset.z
                            );

                            // adds the vertex point to the vertices
                            vertices_table.push(vertex_point.x);
                            vertices_table.push(vertex_point.y);
                            vertices_table.push(vertex_point.z);

                            // pushes the indices, increments
                            indices_table.push(index);
                            indices_table.push(index + 1);
                            indices_table.push(index + 2);

                            index = index + 3;
                        }
                    }
                }
            }
        }

        // sets the vertices table
        self.vertices = vertices_table;
        self.indices = indices_table;
    }
}