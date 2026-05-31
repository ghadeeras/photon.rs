const pi = 4.0 * atan(1.0);
const two_pi = 2.0 * pi;
const half_pi = 0.5 * pi;

struct Vertex {
    normal: vec3<f32>,
    color: vec3<f32>,
}

struct Triangle {
    as_matrix: mat4x4<f32>,
    normal: vec3f,
}

@group(0) @binding(0)
var<storage, read_write> triangles: array<mat4x4<f32>>;

@group(0) @binding(1)
var<storage, read_write> vertices: array<Vertex>;

@compute
@workgroup_size(64, 1, 1)
fn c_main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {
    let i = global_invocation_id.x;
    if (i > 63u) {
        return;
    }

    let half  =  i        & 1u;
    let slice = (i >> 1u) & 7u;
    let stack = (i >> 4u) & 3u;
    var t = calc_triangle(8u, slice, stack, half);

    let i_f32 = bitcast<f32>(i);
    t[0].w = i_f32;
    t[1].w = i_f32;
    t[2].w = i_f32;
    triangles[i] = t;

    let n = vec3(t[0].z, t[1].z, t[2].z);
    vertices[i] = Vertex(n, vec3(0.3, 0.6, 0.9));
}

fn calc_triangle(resolution: u32, slice: u32, stack: u32, half: u32) -> mat4x4<f32> {
    let s_t = spherical_triangle(resolution, slice, stack, half);
    let t = to_triangle(s_t);
    return triangle_as_matrix(t);
}

fn spherical_triangle(resolution: u32, slice: u32, stack: u32, half: u32) -> mat3x2<f32> {
    let delta = two_pi / f32(resolution);

    let a0 = f32(slice) * delta;
    let a1 = a0 + delta;

    let b0 = -f32(stack) * delta + half_pi;
    let b1 = b0 - delta;

    if (half == 0u) {
        return mat3x2(a0, b0, a0, b1, a1, b1);
    } else {
        return mat3x2(a1, b1, a1, b0, a0, b0);
    };
}

fn to_triangle(t: mat3x2<f32>) -> mat3x3<f32> {
    return mat3x3(
        spherical_point(t[0]),
        spherical_point(t[1]),
        spherical_point(t[2])
    );
}

fn triangle_as_matrix(t: mat3x3<f32>) -> mat4x4<f32> {
    let x = t[1] - t[0];
    let y = t[2] - t[0];

    let zz = cross(x, y);
    let z = normalize(zz);

    let xx = cross(y, z);
    let yy = cross(z, x);

    let inverse_det = 1.0 / dot(z, zz);
    let m = inverse_det * transpose(mat3x3(xx, yy, zz));

    let m_3_ = m * (-t[0]);
    return mat4x4(
        vec4(m[0], 0.0),
        vec4(m[1], 0.0),
        vec4(m[2], 0.0),
        vec4(m_3_, 1.0)
    );
}

fn spherical_point(p: vec2f) -> vec3f {
    let z = sin(p.y);
    let r = cos(p.y);
    let x = cos(p.x) * r;
    let y = sin(p.x) * r;
    return vec3(x, y, z);
}
