struct MeshView {
    first_index: u32,
    first_vertex: u32,
    indices_count: u32,
    vertices_count: u32,
}

struct Vertex {
    normal: vec3f,
    color: vec3f,
}

struct Transformation {
    position_mat: mat4x4f,
    normal_mat: mat4x4f,
}

@group(0) @binding(0)
var<uniform> mesh_view: MeshView;

@group(0) @binding(1)
var<uniform> transformation: Transformation;

@group(0) @binding(2)
var<storage, read_write> positions: array<vec3f>;

@group(0) @binding(3)
var<storage, read_write> vertices: array<Vertex>;

@compute
@workgroup_size(64, 1, 1)
fn c_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x;
    if (i >= mesh_view.vertices_count) {
        return;
    }
    let index = mesh_view.first_vertex + i;
    positions[index] = (transformation.position_mat * vec4(positions[index], 1.0)).xyz;
    vertices[index].normal = normalize((transformation.normal_mat * vec4(vertices[index].normal, 0.0)).xyz);
}
