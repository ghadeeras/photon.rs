struct Vertex {
    normal: vec3<f32>,
    color: vec3<f32>,
}

struct Triangle {
    as_matrix: mat4x4<f32>,
    corners: vec3u,
}

struct Camera {
    view_matrix: mat4x4<f32>,
    focal_ratio: f32,
    gain: f32,
}

struct Ray {
    origin: vec4<f32>,
    direction: vec4<f32>,
}

struct Environment {
    sun_direction: vec3<f32>,
    sun_color: vec3<f32>,
    sky_color: vec3<f32>,
}

struct Varyings {
    @builtin(position) position: vec4<f32>,
    @location(0) sensor_position: vec4<f32>,
    @location(1) time_sec: f32,
}

struct Fragment {
    @location(0) color: vec4<f32>,
}

const max_distance = 65536.0;
const diffuse = 0.75;
const specular = 0.25;
const specularity = 128.0;
const ambient_light = 0.015625;

const camera = Camera(
    mat4x4<f32>(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ),
    2.0,
    1.0
);

const environment = Environment(
    vec3<f32>(-5, 5, 10.0) / sqrt(150.0),
    vec3<f32>(1.0, 1.0, 1.0),
    vec3<f32>(0.02, 0.16, 0.64),
);

const full_screen_triangle = array<vec4<f32>, 3>(
    vec4<f32>(-1.0,  3.0, 0.0, 1.0),
    vec4<f32>(-1.0, -1.0, 0.0, 1.0),
    vec4<f32>( 3.0, -1.0, 0.0, 1.0),
);

const pi = 4.0 * atan(1.0);
const one_to_pi = 1.0 / pi;
const one_to_two_pi = 0.5 * one_to_pi;
const a = pi / 3.0;
const c = cos(a);
const s = sin(a);

@group(0) @binding(0)
var<storage, read> triangles: array<mat4x4<f32>>;

@group(0) @binding(1)
var<storage, read> vertices: array<Vertex>;

@vertex
fn v_main(@builtin(vertex_index) i: u32, @builtin(instance_index) t_ms: u32) -> Varyings {
    let t = f32(t_ms) / 1000.0;
    let v = full_screen_triangle[i];
    return Varyings(v, v, t);
}

@fragment
fn f_main(varyings: Varyings) -> Fragment {
    let aspect_ratio = -dpdyFine(varyings.sensor_position.y) / dpdxFine(varyings.sensor_position.x);
    let ray = primary_ray(varyings.sensor_position.xy, aspect_ratio, varyings.time_sec);

    var hit_triangle = -1;
    var hit = vec4(0.0, 0.0, 1.0, 0.0);
    let triangles_count = i32(arrayLength(&triangles));
    for (var i = 0; i < triangles_count; i++) {
        let t = decode_triangle(i);
        let new_hit = intersect(ray, t);
        let closer_hit = new_hit.z > 0.0 && compare(new_hit.zw, hit.zw) < 0.0;
        hit_triangle = select(hit_triangle, i, closer_hit);
        hit = select(hit, new_hit, closer_hit);
    }
    if (hit_triangle >= 0 && hit_triangle < triangles_count)  {
        let t = decode_triangle(hit_triangle);
        let vertex = interpolate(hit, t);
        let geometric_factor = max(dot(vertex.normal, environment.sun_direction), 0.0);
        let reflection_light_alignment = max(dot(reflect(ray.direction.xyz, vertex.normal), environment.sun_direction), 0.0);
        let diffuse_shade = one_to_pi * diffuse * geometric_factor;
        let specular_shade = select(
            0.0,
            specular * (specularity + 1.0) * one_to_two_pi * pow(reflection_light_alignment, specularity),
            geometric_factor > 0.0
        );
        let ambient_shade = diffuse * ambient_light;
        return Fragment(vec4((diffuse_shade + specular_shade + ambient_shade) * vertex.color, 1.0));
    } else {
        let sun_light = pow(max(dot(ray.direction.xyz, environment.sun_direction), 0.0), 90.0);
        return Fragment(vec4(environment.sky_color + sun_light, 1.0));
    }
}

fn decode_triangle(i: i32) -> Triangle {
    var compact_t = triangles[i];
    let corners = bitcast<u32>(vec3(
        compact_t[0][3],
        compact_t[1][3],
        compact_t[2][3]
    ));
    compact_t[0][3] = 0.0;
    compact_t[1][3] = 0.0;
    compact_t[2][3] = 0.0;
    return Triangle(compact_t, corners);
}

fn compare(ratio1: vec2<f32>, ratio2: vec2<f32>) -> f32 {
    return ratio1.x * ratio2.y - ratio1.y * ratio2.x;
}

fn primary_ray(position: vec2<f32>, aspect_ratio: f32, time_sec: f32) -> Ray {
    let c = cos(time_sec);
    let s = sin(time_sec);
    let m = mat4x4(
          c, 0.0,   s, 0.0,
        0.0, 1.0, 0.0, 0.0,
         -s, 0.0,   c, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    return Ray(
        m * vec4(0.0, 0.0, 5.0, 1.0),
        m * normalize(vec4(
             position.x * aspect_ratio,
             position.y,
            -camera.focal_ratio,
             0.0
        ))
    );
}

fn intersect(ray: Ray, t: Triangle) -> vec4<f32> {
    let new_ray = Ray(t.as_matrix * ray.origin, t.as_matrix * ray.direction);
    let abs_dir_z = abs(new_ray.direction.z);
    let distance = select(new_ray.origin.z, -new_ray.origin.z, new_ray.direction.z >= 0.0);
    let v = abs_dir_z * new_ray.origin + distance * new_ray.direction;
    let denom = select(0.0, abs_dir_z, v.x >= 0.0 && v.y >= 0.0 && (v.x + v.y) <= abs_dir_z);
    return vec4(v.x, v.y, distance, denom);
}

fn interpolate(hit: vec4<f32>, t: Triangle) -> Vertex {
    let w = 1.0 / hit.w;
    let w0 = w * hit.x;
    let w1 = w * hit.y;
    let w2 = 1.0 - w0 - w1;
    let v0 = vertices[t.corners[0]];
    let v1 = vertices[t.corners[1]];
    let v2 = vertices[t.corners[2]];
    let n = normalize(w0 * v0.normal + w1 * v1.normal + w2 * v2.normal);
//    let n = vec3(t.as_matrix[0].z, t.as_matrix[1].z, t.as_matrix[2].z);
    let c = w0 * v0.color  + w1 * v1.color  + w2 * v2.color;
    return Vertex(n, c);
}