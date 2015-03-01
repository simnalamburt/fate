#version 410

uniform mat4 matrix;

in vec3 position;
in vec3 normal;

smooth out vec3 _normal;

void main() {
  gl_Position = matrix * vec4(position, 1.0);
  _normal = normalize(normal);
}
