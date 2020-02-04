#version 330 core

uniform int iIterations = 1000;
uniform vec2 Aspect = vec2(1., 1.);
uniform vec2 Offset = vec2(0., 0.);
uniform float Zoom = 1.;
uniform ivec2 iSize;

in vec2 vsPos;
out float fsColor;

void main() {
    vec2 start = vsPos * Aspect / Zoom + Offset;
    int i = 0;
    vec2 pos = vec2(0.);
    vec2 sqpos = pos * pos;
    for (; sqpos.x + sqpos.y <= 4 && i < iIterations; i++) {
        pos.y = pos.x * pos.y;
        pos.y += pos.y + start.y;
        pos.x = sqpos.x - sqpos.y + start.x;
        sqpos = pos * pos;
    }
    fsColor = float(i) / float(iIterations);
}