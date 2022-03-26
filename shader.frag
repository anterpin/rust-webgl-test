# version 300 es

precision highp float;

in vec3 outNormal;
in vec3 toLightVector;
in vec2 passTextureCoords;

out vec4 outColor;

const vec3 lightColor = vec3(1,1,1);

uniform sampler2D image;

void main() {
    float brightness = max(dot(normalize(outNormal),normalize(toLightVector)),0.0);
    
    vec3 diffuse = brightness * lightColor;
    outColor = vec4(diffuse,1.0) * vec4(0,1,1,1);
    outColor.x = abs(outNormal.x);
    outColor.y = abs(outNormal.y);
    outColor.z = abs(outNormal.z);
    outColor.w = 1.0;
    outColor = texture(image, passTextureCoords);
}
