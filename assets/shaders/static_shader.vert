#version 400

in vec3 position;

//-----------------------

out vec3 out_pos;
out float visibility;

//-----------------------

uniform mat4 transform_matrix;
uniform mat4 projection_matrix;
uniform mat4 view_matrix;

//-----------------------

const float fog_density = 0.1;
const float fog_gradient = 2;

//-----------------------

void main() {
    gl_Position = projection_matrix * view_matrix * transform_matrix * vec4(position, 1.0);

    vec3 base_col = vec3((position.x + 12.8)/25.6, (position.y + 12.8)/25.6, (position.z + 12.8)/25.6);

    float camera_dist = length((view_matrix * transform_matrix * vec4(position, 1.0)).xyz);

    visibility = exp(-pow((camera_dist * fog_density), fog_gradient));
    out_pos = base_col;
}