#version 450

layout(location = 0) in vec2 tex_coords;
layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D tex;
layout(set = 0, binding = 1) uniform sampler tex_sampler;

vec3 lines(float d) {
    float scale = 1.0;
    d = fract(d * scale);
    float pos = 0.1;
    float w = 0.05;
    vec3 color = vec3(smoothstep(pos + w, pos, d) - smoothstep(pos, pos - w, d));
    return color;
}

void main() {
    vec3 color = vec3(0.0);
    vec3 values = texture(sampler2D(tex, tex_sampler), tex_coords).rgb;

    // plain
    color += vec3(smoothstep(0.0, 0.1, values.g));

    // lines
    // color = lines(values.g);

    // add noise
    color.r += values.b * 10.0;
    // color.gb -= values.b * 10.0;

    f_color = vec4(color, 1.0);
}
