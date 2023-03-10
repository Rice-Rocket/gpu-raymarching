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

uniform mat3 camera;
uniform vec3 camera_origin;
uniform float camera_focal_length;
uniform scene_objects {
    mat4 objects[MAX_OBJECTS];
};
uniform scene_lights {
    vec3 lights[MAX_LIGHTS];
};
uniform scene_csgs {
    vec4 csgs[MAX_CSGS];
};

float scene_sd(vec3 p) {
    float dist = 1e6;
    for (int i = 0; i < MAX_OBJECTS; ++i) {
        mat4 obj = objects[i];
        if (obj[0][0] == 0) {
            break;
        };
        if (obj[0][0] == SPHERE_ID) {
            vec3 center = obj[1].xyz;
            float radius = obj[0][1];
            float d = length(p - center) - radius;
            dist = min(dist, d);
        };
    }
    return dist;
}

float march(vec3 origin, vec3 direction) {
    float dist = 0;
    for (int i = 0; i < MAX_STEPS; ++i) {
        vec3 p = origin + direction * dist;
        float ds = scene_sd(p);
        dist += ds;
        if (ds < MIN_DIST) {
            break;
        };
        if (dist > MAX_DIST) {
            break;
        };
    }
    return dist;
}

vec3 render(vec3 rd) {
    float dist = march(camera_origin, rd);
    dist /= 6;
    return vec3(dist, dist, dist);
}

void main() {
    vec3 ray_direction = camera * normalize(vec3(fragCoord, camera_focal_length));
    vec3 color = render(ray_direction);
    fragColor = vec4(color, 1.0);
}