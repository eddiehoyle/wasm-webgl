#version 300 es
precision mediump float;

in vec3 position;

uniform vec2 resolution;
uniform vec2 translation;

void main() {

    vec2 zeroToOne = position.xy / resolution;
    vec2 zeroToTwo = zeroToOne * 2.0;
    vec2 clipSpace = zeroToTwo - 1.0;

//    gl_Position = vec4(position, 1.0);
    gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
}