use cgmath::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

//------------------------

// hashmaps are fine for implementation since they scale well w/ O(1) gets
pub struct TriangulationTable {
    pub table: HashMap<u16, Vec<u16>>,

    pub corner_index_a_from_edge: Vec<Vector3<f32>>,
    pub corner_index_b_from_edge: Vec<Vector3<f32>>
}

//------------------------

impl TriangulationTable {
    pub fn new() -> TriangulationTable {
        // creates a new table for the triangulation table
        let mut table = HashMap::new();

        // loads up the triangulation table from the files
        let triangulation_file = File::open("assets/triangulation/triangulation_table.txt");

        // creates an iterator over the file
        let line_iterator = BufReader::new(triangulation_file.unwrap()).lines();

        // sets an initial line counter
        let mut line_counter = 0;

        // loops over the lines to add to the hashmap
        for line in line_iterator {
            // unwraps the line
            let line_result = line.unwrap();

            if line_result != "N" {
                // creates a vector across the line
                let str_vector: Vec<&str> = line_result.split(",").collect();

                // converts the num vector
                let num_vector: Vec<u16> = str_vector.into_iter().map(|e| e.parse::<u16>().unwrap()).collect();

                // inserts to the table
                table.insert(line_counter, num_vector);
            }
            else {
                // fills with an empty entry
                table.insert(line_counter, Vec::new());
            }

            // adds to the line counter
            line_counter += 1;
        }

        // initialises corner_index_a_from_edge and the other corner index table
        let corner_index_a_from_edge: Vec<Vector3<f32>> = vec![
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 1.0),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(0.0, 1.0, 1.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 1.0),
            Vector3::new(0.0, 0.0, 1.0)
        ];

        let corner_index_b_from_edge: Vec<Vector3<f32>> = vec![
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 1.0),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(0.0, 1.0, 1.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(0.0, 1.0, 1.0)
        ];

        TriangulationTable {
            table,
            corner_index_a_from_edge,
            corner_index_b_from_edge
        }
    }
}