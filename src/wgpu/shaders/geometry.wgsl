const pi = 4.0 * atan(1.0);
const two_pi = 2.0 * pi;

struct Vertex {
    normal: vec3f,
    color: vec3f,
}

struct VertexLocation {
    id: vec2u,
    max: vec2u,
    invalid: bool,
}

struct VertexAndPos {
    position: vec3f,
    vertex: Vertex,
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
    let location = current_location(id);
    if (location.invalid) {
        return;
    }
    let vp = sphere_vertex(location);
    output(location, vp.position, vp.vertex);
}

fn sphere_vertex(location: VertexLocation) -> VertexAndPos {
    let a = two_pi * (f32(location.id.x) / f32(location.max.x));
    let b = pi * (f32(location.id.y) / f32(location.max.y));
    let z = cos(b);
    let r = sin(b);
    let x = cos(a) * r;
    let y = sin(a) * r;
    let position = vec3(x, y, z);
    return VertexAndPos(position, Vertex(position, vec3(1.0)));
}

fn current_location(id: vec3<u32>) -> VertexLocation {
    let quads_count = f32(arrayLength(&indices) / 6u);
    let vertices_count = f32(min(arrayLength(&positions), arrayLength(&vertices)));
    let b = 0.5 * (quads_count - vertices_count + 1.0);
    let delta = b * b - quads_count;
    if (delta < 0.0) {
        return VertexLocation(id.xy, vec2<u32>(0), true);
    }
    let delta_sqrt = sqrt(delta);
    let max = vec2<u32>(round(vec2(-b + delta_sqrt, -b - delta_sqrt)));
    return VertexLocation(id.xy, max, any(id.xy > max));
}

fn output(location: VertexLocation, position: vec3f, vertex: Vertex) {
    let i = location.id.y * (location.max.x + 1u) + location.id.x;
    positions[i] = position;
    vertices[i] = vertex;

    let x_size = location.max.x * 6u;
    let quad_base = location.id.y * x_size + location.id.x * 6u;
    if (location.id.x < location.max.x && location.id.y < location.max.y) {
        indices[quad_base     ] = i;
        indices[quad_base + 4u] = i;
    }
    if (location.id.x > 0u && location.id.y < location.max.y) {
        let quad_base_east = quad_base - 6u;
        indices[quad_base_east + 3u] = i;
    }
    if (location.id.y > 0u && location.id.x < location.max.x) {
        let quad_base_north = quad_base - x_size;
        indices[quad_base_north + 1u] = i;
    }
    if (location.id.x > 0u && location.id.y > 0u) {
        let quad_base_north_east = quad_base - x_size - 6u;
        indices[quad_base_north_east + 2u] = i;
        indices[quad_base_north_east + 5u] = i;
    }
}
