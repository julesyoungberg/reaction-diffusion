#version 450

layout(location = 0) in vec2 tex_coords;
layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D tex;
layout(set = 0, binding = 1) uniform sampler tex_sampler;
layout(set = 0, binding = 2) uniform Uniforms {
    float width;
    float height;
    float time;
};

const float diffusion_rate_a = 1.0;
const float diffusion_rate_b = 0.5;
const float feed_rate = 0.025;
const float kill_rate = 0.05;
const float reaction_speed = 1.0;

vec3 laplacian(vec3 color) {
    vec3 d = vec3(1.0 / vec2(width, height), 0.0);

    color += texture(sampler2D(tex, tex_sampler), tex_coords - d.xy).rgb * 0.05;
    color += texture(sampler2D(tex, tex_sampler), tex_coords - d.zy).rgb * 0.2;
    color += texture(sampler2D(tex, tex_sampler), tex_coords - d.xy * vec2(-1.0, 1.0)).rgb * 0.05;
    color += texture(sampler2D(tex, tex_sampler), tex_coords - d.xz).rgb * 0.2;

    color += texture(sampler2D(tex, tex_sampler), tex_coords + d.xy).rgb * 0.05;
    color += texture(sampler2D(tex, tex_sampler), tex_coords + d.zy).rgb * 0.2;
    color += texture(sampler2D(tex, tex_sampler), tex_coords + d.xy * vec2(-1.0, 1.0)).rgb * 0.05;
    color += texture(sampler2D(tex, tex_sampler), tex_coords + d.xz).rgb * 0.2;

    return color;
}

void main() {
    vec3 color = vec3(0.0);
    color = clamp(texture(sampler2D(tex, tex_sampler), tex_coords).rgb, vec3(0.0), vec3(1.0));
    float a = color.r;
    float b = color.g;

    vec3 lp = laplacian(-color);
    float a2 = a + (diffusion_rate_a * lp.x - a * b * b + feed_rate * (1.0 - a)) * reaction_speed;
    float b2 = b + (diffusion_rate_b * lp.y + a * b * b - (kill_rate + feed_rate) * b) * reaction_speed;

    color = clamp(vec3(a2, b2, 0.0), vec3(0.0), vec3(1.0));

    f_color = vec4(color, 1.0);
}
