struct Vert {
    x: f32,
    y: f32,
    z: f32,
}
struct Output {
    verts: array<Vert, 250>,
}

struct Param {
    action: i32,
    amount: f32,
    padding: vec2<f32>,
}

@group(0) @binding(0) var<storage, read> input: array<u32, 249>;
@group(1) @binding(0) var<uniform> params: array<Param, 3>;
@group(2) @binding(0) var<storage, read_write> output: Output;

fn forward_vec(angle: f32) -> vec3f {
    return vec3f(cos(angle), sin(angle), 0.0);
}

const PI:f32 = 3.14159265;

@compute
@workgroup_size(1)
fn main() {
    var pos = vec3f(-0.5, -0.5, 0.1);
    output.verts[0] = Vert(-0.5, -0.5, 0.1);
    var angle = 0.0;
    var count = 1u;
    for (var i = 0; i < 249; i++) {
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
        

        let last = output.verts[count-1];
        let lastV = vec3(last.x, last.y, last.z);
        if (any(lastV != pos)) {
            var out: Vert;
            out.x = pos.x;
            out.y = pos.y;
            out.z = pos.z;

            output.verts[count] = out;
            count++;
        }
    }
    for (var i = 1; i < 250; i++) {
        let cur = output.verts[i];
        if (all(vec3(cur.x, cur.y, cur.z) == vec3(0.0,0.0,0.0))) {
            output.verts[i] = output.verts[count];
        }
    }
}
