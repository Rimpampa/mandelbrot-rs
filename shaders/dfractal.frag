#version 400 core

uniform int iIterations = 1000;
uniform vec2 Aspect = vec2(1., 1.);
uniform dvec2 Offset = vec2(0., 0.);
uniform float Zoom = 1.;
uniform ivec2 iSize;

in vec2 vsPos;
out vec3 fsColor;

void main() {
    dvec2 start = vsPos * Aspect / Zoom + Offset;
    int i = 0;
    dvec2 pos = dvec2(0.);
    dvec2 sqpos = pos * pos;
    for(; sqpos.x + sqpos.y < 4 && i < iIterations; i++) {
        double sum = pos.x + pos.y;
        pos.y = sum * sum - sqpos.x - sqpos.y + start.y;
        pos.x = sqpos.x - sqpos.y + start.x;
        sqpos = pos * pos;
    }
    float factor = float(i) / float(iIterations);
    fsColor = vec3(factor > .99 ? 0 : factor);
}