#version 410
#define MAX_OBJECTS 32
#define MAX_BOOL_OPS 32
#define MAX_LIGHTS 8

#define MAX_STEPS 128
#define MAX_DIST 100.0
#define MIN_DIST 0.0001


in vec2 fragCoord;
out vec4 fragColor;

uniform float time;
uniform vec2 resolution;
uniform vec4 mouse;

uniform mat3 camera;
uniform vec3 camera_origin;
uniform float camera_focal_length;

uniform scene_settings {
    vec4 fog_color;
    int num_objects;
};

uniform scene_objects { mat4 objects[MAX_OBJECTS]; };
uniform scene_lights { vec4 lights[MAX_LIGHTS]; };
uniform scene_bool_ops { vec2 bool_ops[MAX_BOOL_OPS]; };


vec4 op_union(vec4 a, vec4 b) {
    return (a.x < b.x) ? a : b;
}
vec4 op_intersect(vec4 a, vec4 b) {
    return (a.x > b.x) ? a : b;
}
vec4 op_difference(vec4 a, vec4 b) {
    return (a.x > -b.x) ? a : b;
}
vec4 op_smooth_union(vec4 a, vec4 b, float k) {
    float t = -(log(exp(k * -a.x) + exp(k * -b.x)) / k);
    return vec4(t, mix(a.yzw, b.yzw, (t - a.x) / (-b.x - a.x)));
}


vec4 get_sd(vec3 p, mat4 obj) {
    vec4 res = vec4(0);
    int obj_type = int(obj[0][0]);
    if (obj_type == 1) { // sphere
        float dist = length(p - obj[1].xyz) - obj[0][1];
        res = vec4(dist, obj[3].xyz);
    }
    if (obj_type == 2) { // plane
        float dist = dot(p, obj[1].xyz) + obj[0][1];
        res = vec4(dist, obj[3].xyz);
    }
    if (obj_type == 3) { // box
        vec3 q = abs(p - obj[1].xyz) - obj[2].xyz;
        float dist = length(max(q, 0.0)) + min(max(q.x, max(q.y, q.z)), 0.0) - obj[0][1];
        res = vec4(dist, obj[3].xyz);
    }
    return res;
}


vec4 bool_op_sd(vec2 op, vec4 a, vec4 b) {
    vec4 res = vec4(0);
    int op_type = int(op.x);
    float k = op.y;
    switch (op_type) {
        case 1:
            res = op_union(a, b);
            break;
        case 2:
            res = op_intersect(a, b);
            break;
        case 3: 
            res = op_difference(a, b);
            break;
        case 4: 
            res = op_smooth_union(a, b, k);
            break;
    }
    // if (op_type < 2.0) {
    //     res = op_union(a, b);
    // }
    // else if (op_type == 2.0) {
    //     res = op_intersect(a, b);
    // }
    // else if (op_type == 3.0) {
    //     res = op_difference(a, b);
    // }
    // else if (op_type == 4.0) {
    //     res = op_smooth_union(a, b, k);
    // }
    return res;
}


vec4 scene_sd(vec3 p) {
    mat4 obj0 = objects[0];
    vec4 pres = get_sd(p, obj0);
    vec4 res = vec4(1e20, -1, -1, -1);

    int prev_bool_op = int(obj0[0][3]) - 1;
    for (int i = 1; i < MAX_OBJECTS; i++) {
        if (i >= num_objects) break;
        mat4 obj = objects[i];
        float obj_type = obj[0][0];
        if (obj_type == 0.0) {
            res = op_union(res, pres);
            break;
        };
        int bool_op_index = int(obj[0][3]) - 1;
        vec4 d = get_sd(p, obj);

        if (bool_op_index != prev_bool_op) {
            res = op_union(res, pres);
            pres = d;
            // prev_bool_op = bool_op_index;
            // continue;
        }
        else if (bool_op_index == -1) {
            pres = bool_op_sd(vec2(1), pres, d);
            // prev_bool_op = bool_op_index;
            // continue;
        } else {
            vec2 bool_op = bool_ops[0];
            pres = bool_op_sd(bool_op, pres, d);
        }
        prev_bool_op = bool_op_index;
    }
    return res;
}

vec4 march(vec3 origin, vec3 direction) {
    float dist = 0;
    vec4 res = vec4(-1, -1, -1, -1);

    for (int i = 0; i < MAX_STEPS; i++) {
        vec3 p = origin + direction * dist;
        vec4 ds = scene_sd(p);
        if (abs(ds.x) < (MIN_DIST * dist)) {
            res.x = dist;
            res.yzw = ds.yzw;
            break;
        }
        dist += ds.x;
        if (dist > MAX_DIST) break;
    }

    return res;
}

float get_soft_shadow(vec3 ro, vec3 rd, float tmin, float tmax) {
    float res = 1.0;
    float t = tmin;
    float w = 4.0;
    for (int i = 0; i < 24; i++) {
        float h = scene_sd(ro + rd * t).x;
        float s = clamp(w * h / t, 0.0, 1.0);
        res = min(res, s);
        t += clamp(h, 0.01, 0.2);
        if (res < 0.004 || t > tmax) break;
    }
    res = clamp(res, 0.0, 1.0);
    return res * res * (3.0 - 2.0 * res);
}

float get_ambient_occlusion(vec3 p, vec3 norm) {
    float occ = 0.0;
    float sca = 1.0;
    for (int i = 0; i < 5; i++) {
        float h = 0.01 + 0.12 * float(i) / 4.0;
        float d = scene_sd(p + h * norm).x;
        occ += (h - d) * sca;
        sca *= 0.95;
        if (occ > 0.35) break;
    }
    return clamp(1.0 - 3.0 * occ, 0.0, 1.0) * (0.5 + 0.5 * norm.y);
}

vec3 get_normal(vec3 p) {
    vec3 n = vec3(0.0);
    for (int i = 0; i < 4; i++) {
        vec3 e = 0.5773 * (2.0 * vec3((((i + 3) >> 1) & 1), ((i >> 1) & 1), (i & 1)) - 1.0);
        n += e * scene_sd(p + 0.0005 * e).x;
    }
    return normalize(n);
    // vec2 e = vec2(1.0, -1.0) * 0.5773 * 0.0005;
    // return normalize( e.xyy * scene_sd(p + e.xyy).x + 
	// 				  e.yyx * scene_sd(p + e.yyx).x + 
	// 				  e.yxy * scene_sd(p + e.yxy).x + 
	// 				  e.xxx * scene_sd(p + e.xxx).x );
}

vec3 get_light(vec3 p, vec3 rd, vec3 normal, vec3 color) {
    float n_lights = 0;
    vec3 total_light = vec3(0);
    float occ = get_ambient_occlusion(p, normal);
    for (int i = 0; i < MAX_LIGHTS; i++) {
        if (lights[i].w == 0) break;
        vec3 light_pos = lights[i].xyz;
        vec3 l = normalize(light_pos - p);
        vec3 hal = normalize(l - rd);

        float dif = clamp(dot(normal, l), 0.0, 1.0);
        dif *= occ;
        dif *= get_soft_shadow(p, l, 0.02, 5.0);

        vec3 directional = vec3(0.9, 0.9, 0.8) * dif;
        vec3 ambient = vec3(0.03, 0.04, 0.1);

        float spec = pow(clamp(dot(normal, hal), 0.0, 1.0), 16.0);
        spec *= dif;
        spec *= 0.04 + 0.96 * pow(clamp(1.0 - dot(hal, l), 0.0, 1.0), 5.0);
        total_light += color * (directional + ambient);
        total_light += 5.00 * spec;
        n_lights += 1;
    }
    total_light /= n_lights;
    return total_light;
}

vec3 render(vec3 rd) {
    vec3 color = fog_color.xyz - max(rd.y, 0.0) * 0.4;

    vec4 res = march(camera_origin, rd);
    float dist = res.x;
    vec3 material = res.yzw;

    if (material.x > -0.5) {
        vec3 pos = camera_origin + rd * dist;
        vec3 normal = get_normal(pos);
        color = get_light(pos, rd, normal, material);

        float fog_start = 4.0;
        float fog_thickness = 8.0;
        float fog_strength = 1.0 / (1 + exp(-(pos.z / fog_thickness) + fog_start));
        color = mix(color, fog_color.xyz, fog_strength);
    }
    // vec3 reflection = reflect(rd, normal);

    // color = clamp(mix(color, background_color.xyz, 1.0 - exp(-0.0001 * dist * dist * dist)), 0.0, 1.0);
    color = pow(color, vec3(0.4545)); // gamma
    return clamp(color, 0.0, 1.0);
}

void main() {
    vec2 uv = ((fragCoord * resolution.xy) - 0.5 * resolution.xy) / resolution.y;
    vec3 ray_direction = camera * normalize(vec3(uv, camera_focal_length));
    vec3 color = render(ray_direction);
    fragColor = vec4(color, 1.0);
}