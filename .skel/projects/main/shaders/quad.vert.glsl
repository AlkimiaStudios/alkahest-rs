#version 410 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 tex_coords;
layout (location = 2) in int tex_id;
layout (location = 3) in vec4 color;

uniform mat4 projViewMat;

out vec2 texCoords;
flat out int texId;
out vec4 fragColor;

void main() {
  texCoords = tex_coords;
  texId = tex_id;
  fragColor = color;
  gl_Position = projViewMat * vec4(pos, 1.0);
}
