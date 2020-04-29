uniform mat4 worldViewProjectionMatrix;

in vec3 position;
uniform vec4 color;

out vec4 col;

void main()
{
    col = color;
    gl_Position = worldViewProjectionMatrix * vec4(position, 1.0);
}
