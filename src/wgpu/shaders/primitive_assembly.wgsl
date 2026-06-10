@group(0) @binding(0)
var<storage, read> indices: array<u32>;

@group(0) @binding(1)
var<storage, read> positions: array<vec3f>;

@group(0) @binding(2)
var<storage, read_write> triangles: array<mat4x4<f32>>;

@compute
@workgroup_size(64, 1, 1)
fn c_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x;
    let triangles_count = arrayLength(&triangles);
    if (i >= triangles_count) {
        return;
    }
    triangles[i] = calc_triangle(i);
}

fn calc_triangle(triangle_index: u32) -> mat4x4<f32> {
    let is = vertex_indices(triangle_index);
    let t = get_triangle(is);
    var m = triangle_as_matrix(t);
    m[3] = vec4(bitcast<f32>(is), 1.0);
    return m;
}

fn vertex_indices(triangle_index: u32) -> vec3<u32> {
    let i = triangle_index * 3u;
    return vec3(
        indices[i     ],
        indices[i + 1u],
        indices[i + 2u],
    );
}

fn get_triangle(is: vec3<u32>) -> mat3x3<f32> {
    return mat3x3(
        positions[is.x],
        positions[is.y],
        positions[is.z],
    );
}

fn triangle_as_matrix(t: mat3x3<f32>) -> mat4x4<f32> {
    let x = t[0] - t[2];
    let y = t[1] - t[2];

    let zz = cross(x, y);
    let z = normalize(zz);

    let xx = cross(y, z);
    let yy = cross(z, x);

    let inverse_det = 1.0 / dot(z, zz);
    let m = inverse_det * mat3x3(xx, yy, zz);

    let v = -t[2] * m;
    return mat4x4(
        vec4(     m[0], v.x),
        vec4(     m[1], v.y),
        vec4(     m[2], v.z),
        vec4(vec3(0.0), 1.0)
    );
}
