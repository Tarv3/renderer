#version 440

uniform sampler2D normal_map;
uniform sampler2D depth_map;
uniform sampler2D diffuse_map;
uniform sampler2D specular_map;
uniform float depth_scale;

uniform vec3 eye;

in mat3 tbn;
in vec3 f_pos;
in vec2 f_tex;

out vec4 diffuse;
out vec4 normal;
out vec4 specular;

vec2 parallax_mapping(vec2 tex_coords, vec3 view_dir) {
    float depth = texture(depth_map, tex_coords).x;
    vec2 p = view_dir.xy / view_dir.z * (depth * depth_scale);
    return tex_coords - p;
}

void main() {
    vec3 view_dir = normalize(transpose(tbn) * (eye - f_pos));
    vec2 tex_coords = parallax_mapping(f_tex, view_dir); 
    if(tex_coords.x > 1.0 || tex_coords.y > 1.0 || tex_coords.x < 0.0 || tex_coords.y < 0.0){
        discard;
    }

    diffuse = texture(diffuse_map, tex_coords);
    normal = (normalize(tbn * (texture(normal_map, tex_coords) * 2.0 - 1.0)) + 1.0) * 0.5;
    specular = texture(specular_map, tex_coords);

}