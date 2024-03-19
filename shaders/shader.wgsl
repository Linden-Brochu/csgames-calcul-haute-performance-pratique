@group(0) @binding(0) var<storage, read> m1: array<f32>;
@group(0) @binding(1) var<storage, read> m2: array<f32>;
@group(0) @binding(2) var<storage, read_write> m3: array<f32>;
@group(0) @binding(3) var<storage, read> k_max: u32;

@compute @workgroup_size(1) fn main(@builtin(global_invocation_id) id: vec3<u32>, @builtin(num_workgroups) workgroups: vec3<u32>) {
    let i = id.x;
    let j = id.y;
    let index = i * workgroups.y + j;
    for (var k: u32 = 0;; k = k + 1) {
        let m2_index = k * workgroups.y + j;
        let m1_index = i * workgroups.x + k;
        m3[index] = m3[index] + m1[m1_index] * m2[m2_index];
        if (k == k_max - 1) {
            break;
        }
    }
}
