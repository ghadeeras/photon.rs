struct Vertex {
    normal: vec3<f32>,
    color: vec3<f32>,
}

struct Triangle {
    as_matrix: mat4x4<f32>,
    corners: array<u32, 3>,
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
}

struct Fragment {
    @location(0) color: vec4<f32>,
}

const max_distance = 65536.0;

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
    vec3<f32>(0.8, -0.6, 10.0) / sqrt(101.0),
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
const a = pi / 3.0;
const c = cos(a);
const s = sin(a);

const triangles = array<Triangle, 2>(
    Triangle(mat4x4(
          c,   s, 0.0, 0.0,
         -s,   c, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 5.0, 1.0
    ), array(0, 0, 1)),
    Triangle(mat4x4(
        1.0,      0.0,     0.0, 0.0,
        0.0,        c,       s, 0.0,
        0.0,       -s,       c, 0.0,
       -1.0, -5.0 * s, 5.0 * c, 1.0
    ), array(2, 2, 3)),
);

const model_vertexes = array<Vertex, 4>(
    Vertex(vec3(  c,   s, 4.0), vec3(0.3, 0.5, 0.7)),
    Vertex(vec3( -c,  -s, 4.0), vec3(0.3, 0.5, 0.7)),
    Vertex(vec3(0.0,  -s,   c), vec3(0.3, 0.5, 0.7)),
    Vertex(vec3(0.0,  -s,   c), vec3(0.3, 0.5, 0.7)),
);

@vertex
fn v_main(@builtin(vertex_index) i: u32) -> Varyings {
    let v = full_screen_triangle[i];
    return Varyings(v, v);
}

@fragment
fn f_main(varyings: Varyings) -> Fragment {
    let aspect_ratio = -dpdyFine(varyings.sensor_position.y) / dpdxFine(varyings.sensor_position.x);
    let ray = primary_ray(varyings.sensor_position.xy, aspect_ratio);

    var hit_triangle = -1;
    var hit = vec4(0.0, 0.0, 1.0, 0.0);
    for (var i = 0; i < 2; i++) {
        let t = triangles[i];
        let new_hit = intersect(ray, t);
        let closer_hit = new_hit.z > 0.0 && compare(new_hit.zw, hit.zw) < 0.0;
        hit_triangle = select(hit_triangle, i, closer_hit);
        hit = select(hit, new_hit, closer_hit);
    }
    if (hit_triangle >= 0 && hit_triangle < 3) {
        let t = triangles[hit_triangle];
        let vertex = interpolate(hit, t);
        let shade = abs(dot(vertex.normal, environment.sun_direction)) * one_to_pi;
        let specular = pow(dot(reflect(ray.direction.xyz, vertex.normal), environment.sun_direction), 128.0);
        return Fragment(vec4((shade + specular + 0.1) * vertex.color, 1.0));
    } else {
        return Fragment(vec4(environment.sky_color, 1.0));
    }
}

fn compare(ratio1: vec2<f32>, ratio2: vec2<f32>) -> f32 {
    return ratio1.x * ratio2.y - ratio1.y * ratio2.x;
}

fn primary_ray(position: vec2<f32>, aspect_ratio: f32) -> Ray {
    return Ray(
        vec4(vec3(0.0), 1.0),
        normalize(vec4(
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
    let w2 = w * (hit.w - hit.x - hit.y);
    let v0 = model_vertexes[t.corners[0]];
    let v1 = model_vertexes[t.corners[1]];
    let v2 = model_vertexes[t.corners[2]];
    let n = normalize(w0 * v0.normal + w1 * v1.normal + w2 * v2.normal);
    let c = w0 * v0.color  + w1 * v1.color  + w2 * v2.color;
    return Vertex(n, c);
}