#version 450

layout(location = 0) in vec2 tex_coords;
layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 0) uniform Uniforms {
    float width;
    float height;
    float time;
    float diffusion_rate_a;
    float diffusion_rate_b;
    float feed_rate;
    float kill_rate;
    float reaction_speed;
};

void main() {
    vec2 st = tex_coords;
    st.y = 1.0 - st.y;
    st -= 0.5;
    st.x *= width / height;

    vec3 color = vec3(0.0);

    float d = length(st);
    color += smoothstep(0.2, 0.19, d) - smoothstep(0.19, 0.18, d);

    f_color = vec4(color, 1.0);
}
