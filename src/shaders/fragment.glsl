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

uniform scene_fog_color { vec4 fog_color; };
uniform scene_params { vec4 params; };
uniform scene_consts { vec4 consts; };

uniform scene_objects { mat4 objects[MAX_OBJECTS]; };
uniform scene_lights { vec4 lights[MAX_LIGHTS]; };
uniform scene_bool_ops { vec2 bool_ops[MAX_BOOL_OPS]; };
uniform scene_transforms { mat4 transformations[MAX_BOOL_OPS]; };


vec4 op_union(vec4 a, vec4 b) {
    return (a.x < b.x) ? a : b;
}
vec4 op_intersect(vec4 a, vec4 b) {
    return (a.x > b.x) ? a : b;
}
vec4 op_difference(vec4 a, vec4 b) {
    return (a.x > -b.x) ? a : -b;
}
vec4 op_smooth_union(vec4 a, vec4 b, float k) {
    float t = -(log(exp(k * -a.x) + exp(k * -b.x)) / k);
    return vec4(t, mix(a.yzw, b.yzw, (t - a.x) / (-b.x - a.x)));
}

mat4 rotate_mat(vec3 theta) {
    float yaw = theta.x;
    float pitch = theta.y;
    float roll = theta.z;

    float cos_yaw = cos(yaw);
    float sin_yaw = sin(yaw);
    float cos_pitch = cos(pitch);
    float sin_pitch = sin(pitch);
    float cos_roll = cos(roll);
    float sin_roll = sin(roll);

    return mat4(
        vec4(cos_yaw, -sin_yaw, 0.0, 0.0),
        vec4(sin_yaw, cos_yaw, 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0)
    ) * mat4(
        vec4(cos_pitch, 0.0, sin_pitch, 0.0),
        vec4(0.0, 1.0, 0.0, 0.0),
        vec4(-sin_pitch, 0.0, cos_pitch, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0)
    ) * mat4(
        vec4(1.0, 0.0, 0.0, 0.0),
        vec4(0.0, cos_roll, -sin_roll, 0.0),
        vec4(0.0, sin_roll, cos_roll, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0)
    );
}

float sphere_sd(vec3 p, float r) {
    return length(p) - r;
}

float plane_sd(vec3 p, vec3 n, float h) {
    return dot(p, n) + h;
}

float box_sd(vec3 p, vec3 b, float r) {
    vec3 q = abs(p) - b;
    return length(max(q, 0.0)) + min(max(q.x, max(q.y, q.z)), 0.0) - r;
}

float boxframe_sd(vec3 p, vec3 b, float e) {
    p = abs(p) - b;
    vec3 q = abs(p + e) - e;
    return min(min(
      length(max(vec3(p.x, q.y, q.z), 0.0)) + min(max(p.x, max(q.y, q.z)), 0.0),
      length(max(vec3(q.x, p.y, q.z), 0.0)) + min(max(q.x, max(p.y, q.z)), 0.0)),
      length(max(vec3(q.x, q.y, p.z), 0.0)) + min(max(q.x, max(q.y, p.z)), 0.0));
}

float torus_sd(vec3 p, float tx, float ty) {
    vec2 q = vec2(length(p.xz)-tx,p.y);
    return length(q)-ty;
}

float capped_torus_sd(vec3 p, float scx, float scy, float ra, float rb) {
    p.x = abs(p.x);
    float k = (scy*p.x>scx*p.y) ? dot(p.xy,vec2(scx, scy)) : length(p.xy);
    return sqrt( dot(p,p) + ra*ra - 2.0*ra*k ) - rb;
}

float link_sd(vec3 p, float le, float r1, float r2) {
    vec3 q = vec3( p.x, max(abs(p.y)-le,0.0), p.z );
    return length(vec2(length(q.xy)-r1,q.z)) - r2;
}

float cone_sd(vec3 p, float t, float h) {
    vec2 c = vec2(sin(t), cos(t));
    // c is the sin/cos of the angle, h is height
    // Alternatively pass q instead of (c,h),
    // which is the point at the base in 2D
    vec2 q = h * vec2(c.x / c.y, -1.0);
        
    vec2 w = vec2( length(p.xz), p.y );
    vec2 a = w - q*clamp( dot(w,q)/dot(q,q), 0.0, 1.0 );
    vec2 b = w - q*vec2( clamp( w.x/q.x, 0.0, 1.0 ), 1.0 );
    float k = sign( q.y );
    float d = min(dot( a, a ),dot(b, b));
    float s = max( k*(w.x*q.y-w.y*q.x),k*(w.y-q.y)  );
    return sqrt(d)*sign(s);
}

float hex_prism_sd(vec3 p, float hx, float hy) {
    const vec3 k = vec3(-0.8660254, 0.5, 0.57735);
    p = abs(p);
    p.xy -= 2.0*min(dot(k.xy, p.xy), 0.0)*k.xy;
    vec2 d = vec2(
        length(p.xy-vec2(clamp(p.x,-k.z*hx,k.z*hx), hx))*sign(p.y-hx),
        p.z-hy );
    return min(max(d.x,d.y),0.0) + length(max(d,0.0));
}

float tri_prism_sd(vec3 p, float hx, float hy) {
    vec3 q = abs(p);
    return max(q.z-hy,max(q.x*0.866025+p.y*0.5,-p.y)-hx*0.5);
}

float capsule_sd(vec3 p, float h, float r) {
    p.y -= clamp( p.y, 0.0, h );
    return length( p ) - r;
}

float capped_cylinder_sd(vec3 p, float h, float r) {
    vec2 d = abs(vec2(length(p.xz),p.y)) - vec2(r,h);
    return min(max(d.x,d.y),0.0) + length(max(d,0.0));
}

float round_cylinder_sd(vec3 p, float ra, float rb, float h) {
    vec2 d = vec2( length(p.xz)-2.0*ra+rb, abs(p.y) - h );
    return min(max(d.x,d.y),0.0) + length(max(d,0.0)) - rb;
}

float capped_cone_sd(vec3 p, float h, float r1, float r2) {
    vec2 q = vec2( length(p.xz), p.y );
    vec2 k1 = vec2(r2,h);
    vec2 k2 = vec2(r2-r1,2.0*h);
    vec2 ca = vec2(q.x-min(q.x,(q.y<0.0)?r1:r2), abs(q.y)-h);
    vec2 cb = q - k1 + k2*clamp( dot(k1-q,k2)/dot(k2, k2), 0.0, 1.0 );
    float s = (cb.x<0.0 && ca.y<0.0) ? -1.0 : 1.0;
    return s*sqrt( min(dot(ca, ca),dot(cb, cb)) );
}

float solid_angle_sd(vec3 p, float a, float ra) {
    vec2 c = vec2(sin(a), cos(a));
    // c is the sin/cos of the angle
    vec2 q = vec2( length(p.xz), p.y );
    float l = length(q) - ra;
    float m = length(q - c*clamp(dot(q,c),0.0,ra) );
    return max(l,m*sign(c.y*q.x-c.x*q.y));
}

float cut_sphere_sd(vec3 p, float r, float h) {
    // sampling independent computations (only depend on shape)
    float w = sqrt(r*r-h*h);

    // sampling dependant computations
    vec2 q = vec2( length(p.xz), p.y );
    float s = max( (h-r)*q.x*q.x+w*w*(h+r-2.0*q.y), h*q.x-w*q.y );
    return (s<0.0) ? length(q)-r :
            (q.x<w) ? h - q.y     :
                    length(q-vec2(w,h));
}

float cut_hollow_sphere_sd(vec3 p, float r, float h, float t) {
    // sampling independent computations (only depend on shape)
    float w = sqrt(r*r-h*h);
    
    // sampling dependant computations
    vec2 q = vec2( length(p.xz), p.y );
    return ((h*q.x<w*q.y) ? length(q-vec2(w,h)) : 
                            abs(length(q)-r) ) - t;
}

float death_star_sd(vec3 p2, float ra, float rb, float d) {
    // sampling independent computations (only depend on shape)
    float a = (ra*ra - rb*rb + d*d)/(2.0*d);
    float b = sqrt(max(ra*ra-a*a,0.0));
        
    // sampling dependant computations
    vec2 p = vec2( p2.x, length(p2.yz) );
    if( p.x*b-p.y*a > d*max(b-p.y,0.0) )
        return length(p-vec2(a,b));
    else
        return max( (length(p          )-ra),
                -(length(p-vec2(d,0))-rb));

}

float round_cone_sd(vec3 p, float r1, float r2, float h) {
    // sampling independent computations (only depend on shape)
    float b = (r1-r2)/h;
    float a = sqrt(1.0-b*b);

    // sampling dependant computations
    vec2 q = vec2( length(p.xz), p.y );
    float k = dot(q,vec2(-b,a));
    if( k<0.0 ) return length(q) - r1;
    if( k>a*h ) return length(q-vec2(0.0,h)) - r2;
    return dot(q, vec2(a,b) ) - r1;
}

float ellipsoid_sd(vec3 p, vec3 r) {
    float k0 = length(p/r);
    float k1 = length(p/(r*r));
    return k0*(k0-1.0)/k1;
}

float rhombus_sd(vec3 p, float la, float lb, float h, float ra) {
    p = abs(p);
    vec2 b = vec2(la,lb);
    float f = clamp( (normalize(dot(b,b-2.0*p.xz)))/dot(b,b), -1.0, 1.0 );
    vec2 q = vec2(length(p.xz-0.5*b*vec2(1.0-f,1.0+f))*sign(p.x*b.y+p.z*b.x-b.x*b.y)-ra, p.y-h);
    return min(max(q.x,q.y),0.0) + length(max(q,0.0));
}

float octahedron_sd(vec3 p, float s) {
    p = abs(p);
    float m = p.x+p.y+p.z-s;
    vec3 q;
        if( 3.0*p.x < m ) q = p.xyz;
    else if( 3.0*p.y < m ) q = p.yzx;
    else if( 3.0*p.z < m ) q = p.zxy;
    else return m*0.57735027;
        
    float k = clamp(0.5*(q.z-q.y+s),0.0,s); 
    return length(vec3(q.x,q.y-s+k,q.z-k)); 
}

float pyramid_sd(vec3 p, float h) {
    float m2 = h*h + 0.25;
    
    p.xz = abs(p.xz);
    p.xz = (p.z>p.x) ? p.zx : p.xz;
    p.xz -= 0.5;

    vec3 q = vec3( p.z, h*p.y - 0.5*p.x, h*p.x + 0.5*p.y);
    
    float s = max(-q.x,0.0);
    float t = clamp( (q.y-0.5*p.z)/(m2+0.25), 0.0, 1.0 );
        
    float a = m2*(q.x+s)*(q.x+s) + q.y*q.y;
    float b = m2*(q.x+0.5*t)*(q.x+0.5*t) + (q.y-m2*t)*(q.y-m2*t);
        
    float d2 = min(q.y,-q.x*m2-q.y*0.5) > 0.0 ? 0.0 : min(a,b);
        
    return sqrt( (d2+q.z*q.z)/m2 ) * sign(max(q.z,-p.y));
}

float triangle_sd(vec3 p, vec3 a, vec3 b, vec3 c) {
    vec3 ba = b - a; vec3 pa = p - a;
    vec3 cb = c - b; vec3 pb = p - b;
    vec3 ac = a - c; vec3 pc = p - c;
    vec3 nor = cross( ba, ac );

    vec3 i = ba*clamp(dot(ba,pa)/dot(ba, ba),0.0,1.0)-pa;
    vec3 j = cb*clamp(dot(cb,pb)/dot(cb, cb),0.0,1.0)-pb;
    vec3 k = ac*clamp(dot(ac,pc)/dot(ac, ac),0.0,1.0)-pc;

    return sqrt(
    (sign(dot(cross(ba,nor),pa)) +
        sign(dot(cross(cb,nor),pb)) +
        sign(dot(cross(ac,nor),pc))<2.0)
        ?
        min( min(
        dot(i, i),
        dot(j, j) ),
        dot(k, k) )
        :
        dot(nor,pa)*dot(nor,pa)/dot(nor, nor) );
}



vec4 get_sd(vec3 pos, mat4 obj, int index) {
    float dist = 0;
    int obj_type = int(obj[0][0]);
    mat4 trans_data = transformations[index];
    mat4 rotation = rotate_mat(trans_data[1].xyz);
    vec3 p = (rotation * vec4(pos - transformations[index][0].xyz, 1.0)).xyz;
    if (obj_type == 1) { // sphere
        dist = sphere_sd(p, obj[0][1]);
    } else if (obj_type == 2) { // plane
        dist = plane_sd(p, obj[2].xyz, obj[0][1]);
    } else if (obj_type == 3) { // box
        dist = box_sd(p, obj[2].xyz, obj[0][1]);
    } else if (obj_type == 4) { // box frame
        dist = boxframe_sd(p, obj[2].xyz, obj[0][1]);
    } else if (obj_type == 5) { // torus
        dist = torus_sd(p, obj[0][1], obj[0][2]);
    } else if (obj_type == 6) { // horseshoe
        dist = capped_torus_sd(p, obj[1][2], obj[1][3], obj[1][0], obj[1][1]);
    } else if (obj_type == 7) { // link
        dist = link_sd(p, obj[1][0], obj[1][1], obj[1][2]);
    } else if (obj_type == 8) { // cone
        dist = cone_sd(p, obj[0][1], obj[0][2]);
    } else if (obj_type == 9) { // hex prism
        dist = hex_prism_sd(p, obj[0][1], obj[0][2]);
    } else if (obj_type == 10) { // tri prism
        dist = tri_prism_sd(p, obj[0][1], obj[0][2]);
    } else if (obj_type == 11) { // capsule
        dist = capsule_sd(p, obj[0][1], obj[0][2]);
    } else if (obj_type == 12) { // capped cylinder
        dist = capped_cylinder_sd(p, obj[0][1], obj[0][2]);
    } else if (obj_type == 13) { // rounded cylinder
        dist = round_cylinder_sd(p, obj[1][0], obj[1][1], obj[1][2]);
    } else if (obj_type == 14) { // capped cone
        dist = capped_cone_sd(p, obj[1][0], obj[1][1], obj[1][2]);
    } else if (obj_type == 15) { // solid angle
        dist = solid_angle_sd(p, obj[0][1], obj[0][2]);
    } else if (obj_type == 16) { // cut sphere
        dist = cut_sphere_sd(p, obj[0][1], obj[0][2]);
    } else if (obj_type == 17) { // cut hollow sphere
        dist = cut_hollow_sphere_sd(p, obj[1][0], obj[1][1], obj[1][2]);
    } else if (obj_type == 18) { // death star
        dist = death_star_sd(p, obj[1][0], obj[1][1], obj[1][2]);
    } else if (obj_type == 19) { // round cone
        dist = round_cone_sd(p, obj[1][0], obj[1][1], obj[1][2]);
    } else if (obj_type == 20) { // ellipsoid
        dist = ellipsoid_sd(p, obj[1].xyz);
    } else if (obj_type == 21) { // rhombus
        dist = rhombus_sd(p, obj[1][0], obj[1][1], obj[1][2], obj[1][3]);
    } else if (obj_type == 22) { // octahedron
        dist = octahedron_sd(p, obj[0][1]);
    } else if (obj_type == 23) { // pyramid
        dist = pyramid_sd(p, obj[0][1]);
    } else if (obj_type == 24) { // triangle
        dist = triangle_sd(p, obj[1].xyz, obj[2].xyz, vec3(obj[1][3], obj[2][3], obj[3][3]));
    }
    return vec4(dist, obj[3].xyz);
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
    vec4 pres = get_sd(p, obj0, 0);
    vec4 res = vec4(1e20, -1, -1, -1);

    int prev_bool_op = int(obj0[0][3]) - 1;
    for (int i = 1; i < MAX_OBJECTS; i++) {
        if (i >= int(consts.x)) break;
        mat4 obj = objects[i];
        float obj_type = obj[0][0];
        if (obj_type == 0.0) {
            res = op_union(res, pres);
            break;
        };
        int bool_op_index = int(obj[0][3]) - 1;
        vec4 d = get_sd(p, obj, i);

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
    float w = params.x;
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
    float occ = (params.z != 0.0) ? get_ambient_occlusion(p, normal) : 1.0;
    for (int i = 0; i < MAX_LIGHTS; i++) {
        if (lights[i].w == 0) break;
        vec3 light_pos = lights[i].xyz;
        vec3 l = normalize(light_pos - p);
        vec3 hal = normalize(l - rd);

        float dif = clamp(dot(normal, l), 0.0, 1.0);
        dif *= occ;
        if (params.y != 0.0) {
            dif *= get_soft_shadow(p, l, 0.02, 5.0);
        }

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