struct Vertex {
    normal: vec3f,
    color: vec3f,
}

struct Transformation {
    position_mat: mat4x4f,
    normal_mat: mat4x4f,
}

@group(0) @binding(0)
var<uniform> transformation: Transformation;

@group(0) @binding(1)
var<storage, read_write> positions: array<vec3f>;

@group(0) @binding(2)
var<storage, read_write> vertices: array<Vertex>;

@compute
@workgroup_size(64, 1, 1)
fn c_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x;
    let positions_count = arrayLength(&positions);
    if (i >= positions_count) {
        return;
    }
    positions[i] = (transformation.position_mat * vec4(positions[i], 1.0)).xyz;
    vertices[i].normal = normalize((transformation.normal_mat * vec4(vertices[i].normal, 0.0)).xyz);
}
