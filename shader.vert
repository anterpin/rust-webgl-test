#version 300 es

uniform mat4 transformationMatrix;
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

const vec3 lightPosition = vec3(0, 0, 0);

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 textureCoords;
layout(location = 2) in vec3 normals;

out vec3 outNormal;
out vec3 toLightVector;
out vec2 passTextureCoords;

void main() {
  vec4 worldPosition = vec4(position, 1.0);
  gl_Position =
      projectionMatrix * viewMatrix * transformationMatrix * worldPosition;

  vec3 normal = normalize(position);
  outNormal = -(transformationMatrix * vec4(normal, 0.0)).xyz;
  //   outNormal = normal;
  toLightVector = lightPosition - worldPosition.xyz;
  passTextureCoords = textureCoords;
}
