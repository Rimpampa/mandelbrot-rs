#version 330 core

out vec2 vsPos;

void main() {
    switch(gl_VertexID) {
        case 0: vsPos = vec2( 1.,  1.); break;
        case 1: vsPos = vec2(-1.,  1.); break;
        case 2: vsPos = vec2(-1., -1.); break;
        case 3: vsPos = vec2( 1.,  1.); break;
        case 4: vsPos = vec2( 1., -1.); break;
        case 5: vsPos = vec2(-1., -1.); break;
        default: break;
    }
    gl_Position = vec4(vsPos, 0., 1.);
}