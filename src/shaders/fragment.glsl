#version 330

in vec2 fragCoord;
out vec4 fragColor;

uniform float time;
uniform vec2 resolution;
uniform vec4 mouse;

void main() {
    vec3 color = 0.5 + 0.5 * cos(time + fragCoord.xyx + vec3(0, 2, 4));
    fragColor = vec4(color , 1.0);
}