in vec3 vert_colour;

out vec4 frag_colour;

void main() {
    frag_colour = vec4(vert_colour, 1.0);
}