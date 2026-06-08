const pi = 4.0 * atan(1.0);
const two_pi = 2.0 * pi;
const half_pi = 0.5 * pi;

struct Vertex {
    normal: vec3f,
    color: vec3f,
}

@group(0) @binding(0)
var<storage, read_write> indices: array<u32>;

@group(0) @binding(1)
var<storage, read_write> positions: array<vec3f>;

@group(0) @binding(2)
var<storage, read_write> vertices: array<Vertex>;

@compute
@workgroup_size(8, 8, 1)
fn sphere(@builtin(global_invocation_id) id: vec3<u32>) {
    let quads_count = arrayLength(&indices) / 6u;
    let stacks_count = u32(floor(sqrt(0.5 * f32(quads_count))));
    let slices_count = stacks_count << 1u;

    let slice = id.x;
    let stack = id.y;
    if (slice > slices_count || stack > stacks_count) {
        return;
    }

    let position = spherical_point(slices_count, slice, stack);
    let vertex = Vertex(position, vec3(1.0));

    let i = stack * (slices_count + 1u) + slice;
    positions[i] = position;
    vertices[i] = vertex;

    let stack_size = slices_count * 6u;
    let quad_base = stack * stack_size + slice * 6u;
    if (slice < slices_count && stack < stacks_count) {
        indices[quad_base     ] = i;
        indices[quad_base + 4u] = i;
    }
    if (slice > 0u && stack < stacks_count) {
        let quad_base_east = quad_base - 6u;
        indices[quad_base_east + 3u] = i;
    }
    if (stack > 0u && slice < slices_count) {
        let quad_base_north = quad_base - stack_size;
        indices[quad_base_north + 1u] = i;
    }
    if (slice > 0u && stack > 0u) {
        let quad_base_north_east = quad_base - stack_size - 6u;
        indices[quad_base_north_east + 2u] = i;
        indices[quad_base_north_east + 5u] = i;
    }
}

fn spherical_point(slices_count: u32, slice: u32, stack: u32) -> vec3f {
    let delta = two_pi / f32(slices_count);
    let a = f32(slice) * delta;
    let b = f32(stack) * delta;
    let z = cos(b);
    let r = sin(b);
    let x = cos(a) * r;
    let y = sin(a) * r;
    return vec3(x, y, z);
}
