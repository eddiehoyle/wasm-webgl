#version 300 es
precision mediump float;

in vec3 aPosition;
uniform mat4 uProjection;
uniform mat4 uModel;
uniform mat4 uView;

void main() {
    gl_Position =  uProjection * uModel * uView * vec4(aPosition, 1.0);
}