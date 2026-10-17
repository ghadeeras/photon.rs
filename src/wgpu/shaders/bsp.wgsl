const zero3 = vec3(0.0);
const zero_accumulator = Accumulator(zero3, zero3, zero3, 0u);

/*
 * If a scene or a model is made up of V vertices, then constructing the Binary Space Partitioning tree is performed at
 * L = floor(log2(max(V, 16) / 16)) levels, with level 0 representing the root of the tree and level L - 1 its leaf
 * nodes. The construction is parallelized over N lanes (workers), where N = 2^L.
 *
 * At each level l, the lanes are divided into c = 2^l partitions. The workers in each partition produce a new node in
 * the BSP at that level l. So, there are n = 2^(L - l) workers in each partition.
 *
 * Starting from level 0 and ending with level L - 1, the following is done:
 * 1. N statistical accumulators are produced from slices of vertices that correspond to each lane.
 * 2. The accumulators in each partition are folded into its first accumulator.
 * 3. A plane is produced from each accumulator.
 * 4. The vertices in each partition are moved within the partition such that a slice of them is on one side of the
 *    plane, while the remaining slice is on the other side.
 *
 * Step 2 itself spans one or more reduction sub-steps such that each worker reduces a maximum of 256 accumulator
 * produced from a previous step. This means that at a given level l there are ceil(log256(n)) sub-steps.
 *
 * Partitioning planes are produced in a linear array as if the BSP tree was traversed breadth-first. They occupy the
 * following indices (the vertical bars represent separators between levels):
 *     1 | 2, 3 | 4, 5, 6, 7 | 8, 9, 10, 11, 12, 13, 14, 15 | ...
 * This means there are 2 * N - 1 planes. If we have a plane with index p, we can find its level l = floor(log2(p)) and
 * its parent plane at index q = floor(p / 2) and its index within its level i = p - 2^l.
 */

const INIT = 0u;
const ACCUMULATE = 1u;
const REDUCE = 2u;
const PRODUCE_PLANE = 3u;
const SORT_VERTICES = 4u;
const DONE = 5u;

struct State {
    state: u32,
    bsp_level: u32,
    reduction_level: u32,
    done_workers: atomic<u32>,
}

struct WorkerView {
    global_id: u32, // worker index, between 0 and N - 1
    local_id: u32,  // worker index within a partition, between 0 and n - 1
    plane_id: u32,  // plane index in the entire BSP tree, between 1 and  2 * N - 1
    bsp_level: u32, // l
    bsp_depth: u32, // L - l
    bsp_depth_max: u32, // L
    reduction_level: u32, // between 0 and ceil(log256(n))
    reduction_step: u32, // 256^reduction_level
    reduction_size: u32, // 256
    partition_size: u32, // n
    partition_count: u32, // c
    partition_index: u32, // partition index within a leve, between 0 and c - 1
}

struct Plane {
    plane: vec4f,
    back_first: u32,
    back_last: atomic<u32>,
    front_first: atomic<u32>,
    front_last: u32,
}

struct Accumulator {
    sum: vec3f,
    squares: vec3f,
    crosses: vec3f,
    count: u32,
}

@group(0) @binding(0)
var<storage, read> vertex_positions: array<vec3f>;

@group(0) @binding(1)
var<storage, read_write> sorted_indices: array<vec2u>;

@group(0) @binding(2)
var<storage, read_write> accumulators: array<Accumulator>;

@group(0) @binding(3)
var<storage, read_write> planes: array<Plane>;

@group(0) @binding(4)
var<storage, read_write> state: State;

override wg_size: u32 = 256;

@compute
@workgroup_size(wg_size, 1, 1)
fn main(@builtin(global_invocation_id) id: vec3u, @builtin(num_workgroups) wg_count: vec3u) {
    let view = worker_view(id.x, state.bsp_level, state.reduction_level, state.state == REDUCE);
    if view.global_id < (1u << view.bsp_depth_max) {
        switch state.state {
            case ACCUMULATE { accumulate(view); }
            case REDUCE { reduce(view); }
            case PRODUCE_PLANE { produce_plane(view); }
            case SORT_VERTICES { sort_vertices(view); }
            default {}
        }
    }
    let count = atomicAdd(&state.done_workers, 1u);
    if (count == wg_count.x * wg_size) {
        update_state(view);
        atomicStore(&state.done_workers, 0u);
    }
}

fn update_state(view: WorkerView) {
    let next_state = min(state.state + 1u, DONE);
    switch state.state {
        case REDUCE {
            if view.reduction_step >= view.partition_size {
                state.reduction_level = 0u;
                state.state = next_state;
            } else {
                state.reduction_level++;
            }
        }
        case SORT_VERTICES {
            state.state = select(ACCUMULATE, next_state, view.bsp_depth == 0u);
        }
        default {
            state.state = next_state;
        }
    }
}

fn worker_view(worker_id: u32, bsp_level: u32, reduction_level: u32, is_reducer: bool) -> WorkerView {
    let bsp_depth_max = firstLeadingBit(arrayLength(&accumulators));
    let bsp_depth = bsp_depth_max - bsp_level;
    let reduction_step = 1u << (reduction_level * 8u);
    let reduction_size = 1u << 8u;
    let reduction_range = reduction_step * reduction_size;
    let partition_size = 1u << bsp_depth;
    let partition_count = 1u << bsp_level;
    let workers_per_partition = select(partition_size, (partition_size + reduction_range - 1u) / reduction_range, is_reducer);
    let global_id = (worker_id / workers_per_partition) * partition_size + (worker_id % workers_per_partition) * reduction_range;
    let partition_index =  global_id >> bsp_depth;
    let local_id = worker_id & (partition_size - 1u);
    let plane_id = partition_count + partition_index;
    return WorkerView(
        global_id,
        local_id,
        plane_id,
        bsp_level,
        bsp_depth,
        bsp_depth_max,
        reduction_level,
        reduction_step,
        reduction_size,
        partition_size,
        partition_count,
        partition_index,
    );
}

fn accumulate(view: WorkerView) {
    let range = vertices_slice(view);
    var a = zero_accumulator;
    for (var i = range[0]; i < range[1]; i++) {
        let index = sorted_indices[i][view.bsp_level & 1u];
        let position = vertex_positions[index];
        a = add(a, accumulator(position));
    }
    accumulators[view.global_id] = a;
}

fn reduce(view: WorkerView) {
    var a = zero_accumulator;
    let max_index = view.global_id + min(view.reduction_step * view.reduction_size, view.partition_size);
    for (var i = view.global_id; i < max_index; i += view.reduction_step) {
        a = add(a, accumulators[i]);
    }
    accumulators[view.global_id] = a;
}

fn produce_plane(view: WorkerView) {
    let accumulator = accumulators[view.global_id];
    planes[view.plane_id].plane = plane(accumulator);

    let range = partition_slice(view);
    planes[view.plane_id].back_first = range[0];
    atomicStore(&planes[view.plane_id].back_last, range[1]);
    atomicStore(&planes[view.plane_id].front_first, range[0]);
    planes[view.plane_id].front_last = range[1];
}

fn sort_vertices(view: WorkerView) {
    let range = vertices_slice(view);
    for (var i = range[0]; i < range[1]; i++) {
        let vertex_index = sorted_indices[i][view.bsp_level & 1u];
        let new_i = new_sorted_index(vertex_index, view.plane_id);
        sorted_indices[new_i][1u - (view.bsp_level & 1u)] = vertex_index;
    }
}

fn new_sorted_index(vertex_index: u32, plane_index: u32) -> u32 {
    let d = plane_distance(planes[plane_index].plane, vertex_positions[vertex_index]);
    let in_front = d > 0.0 || (d == 0.0 && (vertex_index & 1u) == 0u);
    if in_front {
        return atomicSub(&planes[plane_index].back_last, 1u);
    } else {
        return atomicAdd(&planes[plane_index].front_first, 1u);
    }
}

fn vertices_slice(view: WorkerView) -> vec2u {
    let range = partition_slice(view);
    let range_size = range[1] - range[0];
    let id_range = vec2(view.local_id, view.local_id + 1u);
    return range[0] + vec2u(round(f32(range_size) * (vec2f(id_range) / f32(view.partition_size))));
}

fn partition_slice(view: WorkerView) -> vec2u {
    let parent_plane_id = view.plane_id >> 1u;
    if parent_plane_id == 0 {
        return vec2(0u, arrayLength(&sorted_indices));
    } else if (view.partition_index & 1u) == 0u {
        return vec2(planes[parent_plane_id].back_first, atomicLoad(&planes[parent_plane_id].back_last) + 1u);
    } else {
        return vec2(atomicLoad(&planes[parent_plane_id].front_first), planes[parent_plane_id].front_last + 1u);
    }
}

fn accumulator(v: vec3f) -> Accumulator {
    return Accumulator(
        v,
        v * v,
        v.yzx * v.zxy,
        1u,
    );
}

fn add(a1: Accumulator, a2: Accumulator) -> Accumulator {
    return Accumulator(
        a1.sum + a2.sum,
        a1.squares + a2.squares,
        a1.crosses + a2.crosses,
        a1.count + a2.count,
    );
}

fn plane(a: Accumulator) -> vec4f {
    let factor = 1.0 / f32(a.count);
    let mean = factor * a.sum;
    let s = factor * a.squares - mean * mean;
    let c = factor * a.crosses - mean.yzx * mean.zxy;
    let n = normalize(s + c.yzx + c.zxy);
    let d = -dot(mean, n);
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