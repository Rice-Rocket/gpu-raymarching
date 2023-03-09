#version 330
#define MAX_OBJECTS 128
#define MAX_CSGS 128
#define MAX_LIGHTS 16

#define MAX_STEPS 64
#define MAX_DIST 100.0
#define MIN_DIST 0.01

#define SPHERE_ID 1
#define AAPLANE_ID 2
#define CUBOID_ID 3


in vec2 fragCoord;
out vec4 fragColor;

uniform float time;
uniform vec2 resolution;
uniform vec4 mouse;

uniform mat4 camera;
uniform scene_data {
    mat4 objects[MAX_OBJECTS];
    vec3 lights[MAX_LIGHTS];
    vec4 csgs[MAX_CSGS];
};

float scene_sd(vec3 p) {
    float dist = 1e6;
    for (int i = 0; i < MAX_OBJECTS; ++i) {
        mat4 obj = objects[i];
        if obj[0][0] == 0 { break };
        
    }
}

float march(vec3 origin, vec3 direction) {
    float dist = 0;

    for (int i = 0; i < MAX_STEPS; ++i) {
        vec3 p = origin + direction * dist;
        float ds = 
    }
}

void main() {
    vec3 color = 0.5 + 0.5 * cos(time + fragCoord.xyx + vec3(0, 2, 4));
    fragColor = vec4(color, 1.0);
}