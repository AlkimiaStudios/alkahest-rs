#version 410 core

in vec2 v_texCoords;
in vec4 v_fragColor;

out vec4 FragColor;

uniform sampler2D u_panelTexture;

void main() {
  FragColor = texture(u_panelTexture, v_texCoords) * v_fragColor;
}
