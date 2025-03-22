struct Output {
    verts: array<vec3<f32>, 20>,
    count: u32,
}

@group(0) @binding(0) var<storage, read> input: array<u32, 9>;
@group(1) @binding(0) var<storage, read_write> output: Output;

fn forward_vec(angle: f32) -> vec3f {
    return 0.1 * vec3f(sin(angle), cos(angle), 0.0);
}

@compute
@workgroup_size(1)
fn main() {
    var pos = vec3f(0.0);
    var angle = 0.0;
    for (var i = 0; i < 9; i++) {
        let val = input[i];
        if (val == 0) {
            pos += forward_vec(angle);
        } else if (val == 1) {
            angle += 90.0;
        } else if (val == 2) {
            angle -= 90.0;
        }
        output.verts[i+1] = pos;
    }
    output.count = 10;
}
