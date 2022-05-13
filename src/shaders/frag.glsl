#version 450

layout(location = 0) in vec3 v_normal;
layout(location = 1) in vec2 v_coords;

layout(location = 0) out vec4 f_color;

const vec3 LIGHT = vec3(0.0, 0.0, 1.0);

layout(set = 0, binding = 1) uniform sampler2D tex;

void main() {
    vec3 albedo = texture(tex, v_coords).rgb;
    float brightness = dot(normalize(v_normal), normalize(LIGHT));
    vec3 coord_color = vec3(v_coords.r, v_coords.g, 0);
    coord_color = albedo;
    vec3 dark_color = coord_color * 0.6;
    vec3 regular_color = coord_color;
    f_color = vec4(mix(dark_color, regular_color, brightness), 1.0);

    //    vec4 t_color = texture(tex, vec2(0.5,0.5));
//    f_color = t_color;
    //    f_color = vec4(v_coords, 0, 1);
}
