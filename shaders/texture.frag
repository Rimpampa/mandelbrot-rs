#version 400 core

uniform sampler2D tex;

in vec2 vsPos;
out vec3 fsColor;

// const vec3 COLOR1 = vec3(1, .8, .2);
// const vec3 COLOR2 = vec3(1, .6, .2);
// const vec3 COLOR3 = vec3(.6, 0, .3);
// const vec3 COLOR4 = vec3(.5, 0, .4);
const vec3 COLOR1 = vec3(0, 0, 0);
const vec3 COLOR2 = vec3(0, 1, 0);
const vec3 COLOR3 = vec3(1, 0, 0);
const vec3 COLOR4 = vec3(1, 1, 1);

vec3 palette(float factor) {
    vec3 f;
    f.x = fract(factor * 4823492.233);
    f.y = fract(factor * 2837599.352);
    f.z = fract(factor * 8572345.563);
    vec3 col = COLOR1;
    col = mix(col, COLOR2, f.x);
    col = mix(col, COLOR3, f.y);
    col = mix(col, COLOR4, f.z);
    return col;
}

void main() {
    float factor = texture(tex, vsPos * .5 + .5).r;
    fsColor = vec3(factor > .99 ? 0 : factor);
}