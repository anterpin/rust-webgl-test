#version 300 es

uniform mat4 transformationMatrix;
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

layout(location = 0) in vec3 position;

void main() {
  vec4 worldPosition = vec4(position, 1.0);
  gl_Position =
      projectionMatrix * viewMatrix * transformationMatrix * worldPosition;
}
