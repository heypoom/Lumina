#version 140

in vec2 attr;
out vec4 color;

uniform float t;

void main() {
  color = vec4(attr, 0.0, 1.0);
}
