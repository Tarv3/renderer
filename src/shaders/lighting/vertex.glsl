#version 440

uniform mat4 view;
uniform mat4 model;

in vec3 position;

void main() {
    vec4 pos = view * model * vec4(position, 1.0);
    gl_Position = pos;
}
