struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
}

@fragment
fn main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4f(1.0,0.0,0.0, 1.0);
}
