#version 410 core

in vec2 texCoords;
flat in int texId;
in vec4 fragColor;

out vec4 FragColor;

uniform sampler2D textureSlots[32];
uniform sampler2D quadTex;

void main() {
  FragColor = texture(textureSlots[1], texCoords) * fragColor;
  // FragColor = fragColor;
  // FragColor = vec4(fragColor.xyz, float(texId));
}
