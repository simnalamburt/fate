#version 410

uniform vec3 light;

smooth in vec3 _normal;
out vec4 result;

void main() {
  result = vec4(clamp(dot(_normal, -light), 0.0f, 1.0f) * vec3(1.0f, 0.93f, 0.56f), 1.0f);
}
