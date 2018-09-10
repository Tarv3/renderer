#version 440

uniform sampler2D diffuse_tex;
uniform sampler2D specular_tex;
uniform sampler2D normal_tex;
uniform sampler2D depth_tex;

uniform float shininess;

uniform float T1;
uniform float T2;
uniform mat4 inv_projection;
uniform vec3 light_pos;
uniform vec3 light_colour;
uniform vec3 eye;


out vec4 colour;

vec3 frag_position(vec2 frag_coord) {
    float depth = texture(depth_tex, frag_coord) * 2.0 - 1.0;
    vec3 ndcspace = vec3(frag_coord * 2.0 - 1.0, depth);
    float clipspace_w = T2 / (ndcspace.z + T1);
    vec4 clipspace = vec4(ndcspace * clipspace_w, clipspace_w);
    return (inv_projection * clipspace).xyz;
}

void main() {
    vec2 size = textureSize(depth_tex, 0.0);
    vec2 frag_coord = gl_FragCoord.xy / size;
    vec3 frag_pos = frag_position(frag_coord);
    vec3 light_dir = normalize(light_pos - frag_pos);
    vec3 view_dir = normalize(eye_pos - frag_pos);
    vec3 half_way_dir = normalize(light_dir + view_dir);

    vec3 normal = normalize(texture(normal_tex, f_tex) * 2.0 - 1.0);
    vec3 diffuse_colour = texture(diffuse_tex, f_tex);
    vec3 specular_colour = texture(specular_tex, f_tex);

    float diffuse_value = max(dot(light_dir, normal), 0.0);
    float spec = pow(max(dot(normal, half_way_dir), 0.0), shininess);

    colour = vec4((diffuse_value * diffuse_colour + spec * specular_colour) * light_colour, 1.0);


}