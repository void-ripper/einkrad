#version 330

// Input vertex attributes
in vec3 vertexPosition;
in vec2 vertexTexCoord;
in vec3 vertexNormal;
in vec4 vertexColor;      // Not required

in mat4 instanceTransform;

// Input uniform values
uniform mat4 mvp;

// Output vertex attributes (to fragment shader)
out vec3 fragPosition;
out vec2 fragTexCoord;
out vec4 fragColor;
out vec3 fragNormal;

void main() {
    // Compute MVP for current instance
    mat4 mvpi = mvp * instanceTransform;

    // Send vertex attributes to fragment shader
    fragPosition = vec3(instanceTransform * vec4(vertexPosition, 1.0));
    fragTexCoord = vertexTexCoord;
    fragColor = vertexColor;
    // fragColor = vec4(vertexNormal, 1.0);
  	mat3 normalMatrix = transpose(inverse(mat3(instanceTransform)));
    fragNormal = normalize(normalMatrix * vertexNormal);

    // Calculate final vertex position
    gl_Position = mvpi * vec4(vertexPosition, 1.0);
}
