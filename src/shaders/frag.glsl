#version 450

layout(location = 0) in vec3 v_normal;
layout(location = 1) in vec2 v_coords;

layout(location = 0) out vec4 f_color;

const vec3 LIGHT = vec3(0.0, 0.0, 1.0);

void main() {
    float brightness = dot(normalize(v_normal), normalize(LIGHT));
    vec3 coord_color = vec3(v_coords.r, v_coords.g, 0);
    vec3 dark_color = coord_color * 0.6;
    vec3 regular_color = coord_color;

    f_color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}
