
use cgmath::*;

//-------------------------

// helps create simple transformation matrices from a set of rotations, positions and scales
pub fn generate_transformation_matrix(
    translation: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: f32
) -> Matrix4<f32> {
    // generates matrices for x, y and z rotations
    let x_matrix = Matrix4::from_angle_x(Rad(rotation.x));
    let y_matrix = Matrix4::from_angle_y(Rad(rotation.y));
    let z_matrix = Matrix4::from_angle_z(Rad(rotation.z));

    // gets a full rotation matrix
    let full_rot_matrix = x_matrix * y_matrix * z_matrix;

    // creates the scale matrix
    let scale_matrix = Matrix4::from_scale(scale);

    // creates the translation matrix
    let translation_matrix = Matrix4::from_translation(translation);

    // returns a final result
    let final_matrix = translation_matrix * scale_matrix * full_rot_matrix;

    final_matrix
}