
use cgmath::*;

use glutin::event::VirtualKeyCode;

use crate::{
    input::input_manager::*,
    utils::constants::*
};

use std::f32::consts::PI;

//---------------------

pub struct Camera {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>
}

//---------------------

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0)
        }
    }

    // translates the camera
    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.position = self.position + translation;
    }

    // rotates the camera
    pub fn rotate(&mut self, rotation: Vector3<f32>) {
        self.rotation = self.rotation + rotation;
    }

    // obtains the camera's view matrix based on the cam position
    pub fn get_view_matrix(&mut self) -> Matrix4<f32> {
        // gets matrices for pitch and yaw
        let pitch_matrix = Matrix4::from_angle_x(Rad(self.rotation.x));
        let yaw_matrix = Matrix4::from_angle_y(Rad(self.rotation.y));

        // gets a total rotation matrix
        let total_rotation = pitch_matrix * yaw_matrix;

        // gets a negative version of the camera matrix
        let negative_pos = self.position * (-1.0);

        // creates a position matrix
        let translate_matrix = Matrix4::from_translation(negative_pos);

        total_rotation * translate_matrix
    }

    // updates the camera based on the input manager
    pub fn update(&mut self, input_manager: &mut InputManager, delta: &f32) {
        // sets a base speed for the camera
        let camera_speed: f32 = CAMERA_SPEED * delta;

        // sets a base rotation speed for the camera
        let rotation_speed: f32 = CAMERA_ROT * delta;

        // polls the input checker to check for keys
        if input_manager.poll_key(VirtualKeyCode::W) {
            // calculates the y component of the movement
            let y_component = Rad::sin(Rad(self.rotation.x)) * camera_speed;

            // gets the xz component of the movement
            let xz_component = Rad::cos(Rad(self.rotation.x)) * camera_speed;

            // based on the xz components, gets the x and z movement
            let x_component = Rad::sin(Rad(self.rotation.y)) * xz_component;
            let z_component = Rad::cos(Rad(self.rotation.y)) * xz_component;

            // translates the cam
            self.translate(Vector3::new(x_component, -y_component, -z_component));
        }

        if input_manager.poll_key(VirtualKeyCode::S) {
            // calculates the y component of the movement
            let y_component = Rad::sin(Rad(self.rotation.x)) * camera_speed;

            // gets the xz component of the movement
            let xz_component = Rad::cos(Rad(self.rotation.x)) * camera_speed;

            // based on the xz components, gets the x and z movement
            let x_component = Rad::sin(Rad(self.rotation.y)) * xz_component;
            let z_component = Rad::cos(Rad(self.rotation.y)) * xz_component;

            // translates the cam
            self.translate(Vector3::new(-x_component, y_component, z_component));
        }

        if input_manager.poll_key(VirtualKeyCode::A) {
            // based on the camera_speed, gets the x and z movement
            let x_component = Rad::sin(Rad(self.rotation.y + PI/2.0)) * camera_speed;
            let z_component = Rad::cos(Rad(self.rotation.y + PI/2.0)) * camera_speed;

            // translates the cam
            self.translate(Vector3::new(-x_component, 0.0, z_component));
        }

        if input_manager.poll_key(VirtualKeyCode::D) {
            // based on the camera_speed, gets the x and z movement
            let x_component = Rad::sin(Rad(self.rotation.y + PI/2.0)) * camera_speed;
            let z_component = Rad::cos(Rad(self.rotation.y + PI/2.0)) * camera_speed;

            // translates the cam
            self.translate(Vector3::new(x_component, 0.0, -z_component));
        }

        // rotates the camera using the arrow keys
        if input_manager.poll_key(VirtualKeyCode::Left) {
            self.rotate(Vector3::new(0.0, -rotation_speed, 0.0));
        }

        if input_manager.poll_key(VirtualKeyCode::Right) {
            self.rotate(Vector3::new(0.0, rotation_speed, 0.0));
        }

        if input_manager.poll_key(VirtualKeyCode::Up) {
            self.rotate(Vector3::new(-rotation_speed, 0.0, 0.0));
        }

        if input_manager.poll_key(VirtualKeyCode::Down) {
            self.rotate(Vector3::new(rotation_speed, 0.0, 0.0));
        }
    }
}