#version 440 

uniform mat4 view;
uniform mat4 model;

in vec3 position;
in vec3 normal;
in vec3 tangent;
in vec3 bitangent;
in vec2 tex_coord;

out mat3 tbn;
out vec3 f_pos;
out vec2 f_tex;

void main() {
    vec3 T = normalize(mat3(model) * tangent);
    vec3 B = normalize(mat3(model) * bitangent);
    vec3 N = normalize(mat3(model) * normal);
    tbn = mat3(T, B, N);
    f_tex = tex_coord;
    f_pos = model * vec4(position, 1.0);
    gl_Position = view * f_pos;
}