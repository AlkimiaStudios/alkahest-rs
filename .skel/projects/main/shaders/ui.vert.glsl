#version 410 core

layout (location = 0) in vec4 a_pos;
layout (location = 1) in vec2 a_texCoords;
layout (location = 2) in vec4 a_color;

uniform mat4 u_projViewMat;

out vec2 v_texCoords;
out vec4 v_fragColor;

void main() {
  v_texCoords = a_texCoords;
  v_fragColor = a_color;
  gl_Position = u_projViewMat * a_pos;
}
