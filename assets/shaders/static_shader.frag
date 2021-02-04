#version 400

in vec3 out_pos;
in float visibility;

out vec4 out_colour;

void main() {
    out_colour = mix(vec4(0.1, 0.1, 0.1, 1.0), vec4(out_pos, 1.0), visibility);
}