struct Output {
    verts: array<vec3<f32>, 20>,
    count: u32,
}

struct Param {
    action: i32,
    amount: f32,
    padding: vec2<f32>,
}

@group(0) @binding(0) var<storage, read> input: array<u32, 9>;
@group(1) @binding(0) var<uniform> params: array<Param, 3>;
@group(2) @binding(0) var<storage, read_write> output: Output;

fn forward_vec(angle: f32) -> vec3f {
    return vec3f(cos(angle), sin(angle), 0.0);
}

const PI:f32 = 3.14159265;

@compute
@workgroup_size(1)
fn main() {
    var pos = vec3f(0.0, 0.0, 1.0);
    var angle = 0.0;
    var count = 1u;
    for (var i = 0; i < 9; i++) {
        let val = input[i];
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
        /* if (val == 0) {
            pos += forward_vec(angle);
        } else if (val == 1) {
            angle += PI / 2.0;
        } else if (val == 2) {
            angle -= PI / 2.0;
        } */
        

        if (any(output.verts[count-1] != pos)) {
            output.verts[count] = pos;
            count++;
        }
    }
    output.count = count;
}
