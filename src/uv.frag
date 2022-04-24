in vec3 vert_colour;
in vec2 uv;

out vec4 frag_colour;

uniform sampler2D atlas;

void main() {
    // frag_colour = vec4(1.0, 0.0, 0.0, 1.0);
    // frag_colour = vec4(uv.x, 0.0, uv.y, 1.0);
    frag_colour = texture(atlas, uv);
    // frag_colour = vec4(vert_colour, 1.0);
}