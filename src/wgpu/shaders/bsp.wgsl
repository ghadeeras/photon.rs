const zero3 = vec3(0.0);

struct Node {
    back_first: u32,
    back_last: atomic<u32>,
    front_first: atomic<u32>,
    front_last: u32,
}

struct Accumulator {
    mean: vec3f,
    squares: vec3f,
    crosses: vec3f,
}

fn accumulator(v: vec3f, inverse_count: f32) -> Accumulator {
    let dv = v * inverse_count;
    return Accumulator(
        dv,
        v * dv,
        v.yzx * dv.zxy,
    );
}

fn reduce(a1: Accumulator, a2: Accumulator) -> Accumulator {
    return Accumulator(
        a1.mean + a2.mean,
        a1.squares + a2.squares,
        a1.crosses + a2.crosses,
    );
}

fn plane(a: Accumulator) -> vec4f {
    let n = normalize(
        vec3(a.squares.x, a.crosses.z, a.crosses.y) +
        vec3(a.crosses.z, a.squares.y, a.crosses.x) +
        vec3(a.crosses.y, a.crosses.x, a.squares.z) +
        0.0
    );
    let d = -dot(a.mean, n);
    return vec4(n, d);
}

fn plane_distance(plane: vec4f, v: vec3f) -> f32 {
    return dot(plane.xyz, v) + plane.w;
}

fn triangle_side(plane: vec4f, t: mat3x3<f32>) -> i32 {
    let s = vec3(
        plane_distance(plane, t[0]),
        plane_distance(plane, t[1]),
        plane_distance(plane, t[2]),
    );
    return select(select(0, -1, all(s <= zero3)), 1, all(s >= zero3));
}