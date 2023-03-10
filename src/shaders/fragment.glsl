#version 330
#define MAX_OBJECTS 128
#define MAX_CSGS 128
#define MAX_LIGHTS 16

#define MAX_STEPS 128
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

uniform scene_settings {
    vec3 background_color;
};

uniform scene_objects { mat4 objects[MAX_OBJECTS]; };
uniform scene_lights { vec3 lights[MAX_LIGHTS]; };
uniform scene_csgs { vec4 csgs[MAX_CSGS]; };

float get_sd(vec3 p, mat4 obj) {
    float dist;
    if (obj[0][0] == 1) {
        dist = length(p - obj[1].xyz) - obj[0][1];
    }
    if (obj[0][0] == 2) {
        dist = p[int(obj[0][1])] + obj[0][2];
    }
    return dist;
}

float scene_sd(vec3 p) {
    float dist = get_sd(p, objects[0]);
    for (int i = 1; i < MAX_OBJECTS; i++) {
        mat4 obj = objects[i];
        if (obj[0][0] == 0) break;
        float d = get_sd(p, obj);
        dist = min(d, dist);
    }
    return dist;
}

float march(vec3 origin, vec3 direction) {
    float dist = 0;

    for (int i = 0; i < MAX_STEPS; i++) {
        vec3 p = origin + direction * dist;
        float ds = scene_sd(p);
        dist += ds;
        if (dist > MAX_DIST || ds < MIN_DIST) break;
    }

    return dist;
}

vec3 render(vec3 rd) {
    vec3 color = background_color - max(rd.y, 0.0) * 0.3;
    float d = march(camera_origin, rd);
    d /= 10.;
    vec3 color = vec3(d);
    color = pow(color, vec3(0.4545)); // gamma
    return vec3(color);
}

void main() {
    vec2 uv = ((fragCoord * resolution.xy) - 0.5 * resolution.xy) / resolution.y;
    vec3 ray_direction = camera * normalize(vec3(uv, camera_focal_length));
    vec3 color = render(ray_direction);
    fragColor = vec4(color, 1.0);
}