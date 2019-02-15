#version 300 es
precision mediump float;

in vec3 aPosition;
uniform mat4 uPerspMatrix;

void main() {
    gl_Position = uPerspMatrix * vec4(aPosition, 1.0);
}