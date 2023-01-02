#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 aColor;

out vec3 color;

uniform mat4 projViewMat;
uniform mat4 modelMat;

void main() {
  color = aColor;
  gl_Position = projViewMat * modelMat * vec4(position, 1.0);
}
