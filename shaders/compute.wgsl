const MESHES: u32 = 1024; const BUF_SIZE: u32 = 110;
struct Vert {
    x: f32,
    y: f32,
    z: f32,
}

struct Param {
    action: i32,
    amount: f32,
    padding: vec2<f32>,
}

@group(0) @binding(0) var<storage, read> input: array<array<u32, BUF_SIZE>, MESHES>;
@group(1) @binding(0) var<uniform> params: array<Param, 3>;
@group(2) @binding(0) var<storage, read_write> output: array<array<Vert, BUF_SIZE>, MESHES>;

fn forward_vec(angle: f32) -> vec3f {
    return vec3f(cos(angle), sin(angle), 0.0);
}

const PI:f32 = 3.14159265;

@compute
@workgroup_size(1)
fn main(@builtin(workgroup_id) workgroup:vec3<u32>) {
    let id = workgroup.x;
    var pos = vec3f(-0.8, -0.8, 0.1);
    output[id][0] = Vert(pos.x, pos.y, pos.z);
    var angle = PI / 2.0;
    var count = 1u;
    for (var i = 0u; i < BUF_SIZE; i++) {
        let val = input[id][i];
        if (val == 100) {
            continue;
        }
        let param = params[val];
        switch param.action {
            case 0: {
                pos += param.amount * forward_vec(angle);
            }
            case 1: {
                angle += param.amount;
            }
            default {}
        }
        

        let last = output[id][count-1];
        let lastV = vec3(last.x, last.y, last.z);
        if (any(lastV != pos)) {
            var out: Vert;
            out.x = pos.x;
            out.y = pos.y;
            out.z = pos.z;

            output[id][count] = out;
            count++;
        }
    }
    for (var i = 1u; i < BUF_SIZE; i++) {
        let cur = output[id][i];
        if (all(vec3(cur.x, cur.y, cur.z) == vec3(123456.789))) {
            output[id][i] = output[id][count-1];
        }
    }
}
