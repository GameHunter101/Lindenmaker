use core::f32;
use std::collections::{HashMap, HashSet};

use lindenmayer::{progress, separate_stack_strings};
use lindenmayer_component::LindenmayerComponent;
use spawner_component::SpawnerComponent;
use v4::{
    V4,
    builtin_components::mesh_component::{MeshComponent, VertexDescriptor},
    ecs::{
        compute::Compute,
        material::{ShaderAttachment, ShaderBufferAttachment},
    },
    scene,
};
use wgpu::vertex_attr_array;

mod lindenmayer;
mod lindenmayer_component;
mod spawner_component;

#[tokio::main]
async fn main() {
    let mut rules = HashMap::new();
    /* rules.insert('X', "F+[[X]-X]-F[-FX]+X".to_string());
    rules.insert('F', "FF".to_string()); */
    rules.insert('F', "F-G+F+G-F".to_string());
    rules.insert('G', "GG".to_string());

    let string = (0..4).fold("F-G-G".to_string(), |acc, _| {
        progress(&acc, &['+', '-'], &rules)
    });

    let strings = separate_stack_strings(&string);

    let mut engine = V4::builder()
        .features(wgpu::Features::POLYGON_MODE_LINE)
        .build()
        .await;

    // let string = "F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F";
    

    let alphabet: Vec<char> = vec!['F', 'G', '+', '-']; //string.chars().collect::<HashSet<_>>().into_iter().collect();
    let string_as_nums: Vec<u32> = string
        .chars()
        .map(|c| alphabet.iter().position(|e| *e == c).unwrap() as u32)
        .collect();

    let device = engine.rendering_manager().device();

    let params = vec![
        Param(0, 0.05, 0.0),
        Param(0, 0.05, 0.0),
        Param(1, f32::consts::FRAC_PI_3 * 2.0, 0.0),
        Param(1, -f32::consts::FRAC_PI_3 * 2.0, 0.0),
    ];

    let char_number_mapping: HashMap<char, u32> = vec![('F', 0), ('G', 1), ('+', 2), ('-', 3)].into_iter().collect();

    scene! {
        scene: main,
        _ = {
            material: {
                pipeline: {
                    vertex_shader_path: "./shaders/vertex.wgsl",
                    fragment_shader_path: "./shaders/fragment.wgsl",
                    vertex_layouts: [Vertex::vertex_layout()],
                    uses_camera: false,
                    geometry_details: {
                        topology: wgpu::PrimitiveTopology::LineStrip,
                        strip_index_format: Some(wgpu::IndexFormat::Uint16),
                        polygon_mode: wgpu::PolygonMode::Line,
                    }
                },
                ident: "mat",
            },
            components: [
                MeshComponent(
                    vertices: vec![vec![ Vertex { pos: [0.0, 0.0, 0.0], }; 250 ]],
                    indices: vec![(0..250).collect()],
                    enabled_models: vec![0],
                    ident: "mesh"
                ),
                LindenmayerComponent(
                    compute_component: 5,
                    mesh_component: ident("mesh"),
                    compute_buffer: None,
                    vertex_buffer: None,
                    ident: "thing",
                ),
                SpawnerComponent::new("F-G-G", rules, &['+', '-'], 3, ident("mat"), params, char_number_mapping)
            ],
            /* computes: [
                Compute(
                    input: vec![
                        ShaderAttachment::Buffer(
                            ShaderBufferAttachment::new(
                                device,
                                bytemuck::cast_slice(&string_as_nums),
                                wgpu::BufferBindingType::Storage { read_only: true },
                                wgpu::ShaderStages::COMPUTE,
                                wgpu::BufferUsages::empty(),
                            )
                        ),
                        ShaderAttachment::Buffer(
                            ShaderBufferAttachment::new(
                                device,
                                bytemuck::cast_slice(&[params]),
                                wgpu::BufferBindingType::Uniform,
                                wgpu::ShaderStages::COMPUTE,
                                wgpu::BufferUsages::empty(),
                            )
                        )
                    ],
                    output:
                        ShaderAttachment::Buffer(
                            ShaderBufferAttachment::new(
                                device,
                                bytemuck::cast_slice(&[VertexPositions::default()]),
                                wgpu::BufferBindingType::Storage { read_only: false },
                                wgpu::ShaderStages::COMPUTE,
                                wgpu::BufferUsages::COPY_SRC,
                            )
                        ),
                    shader_path: "./shaders/compute.wgsl",
                    workgroup_counts: (1, 1, 1),
                    id: 5,
                )
            ] */
        }
    };

    engine.attach_scene(main);

    engine.main_loop().await;
}

#[repr(C)]
#[derive(Debug, bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct Vertex {
    pos: [f32; 3],
}

impl VertexDescriptor for Vertex {
    const ATTRIBUTES: &[wgpu::VertexAttribute] = &vertex_attr_array![0 => Float32x3];

    fn from_pos_normal_coords(pos: Vec<f32>, _normal: Vec<f32>, _tex_coords: Vec<f32>) -> Self {
        Self {
            pos: pos.try_into().unwrap(),
        }
    }
}

#[repr(C)]
#[derive(Debug, bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct VertexPositions {
    positions: [[f32; 3]; 250],
    // count: u32,
    // padding: [f32; 4],
}

impl Default for VertexPositions {
    fn default() -> Self {
        Self {
            positions: [[0.0; 3]; 250],
            // count: 0,
            // padding: [0.0; 4],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Param(i32, f32, f64);
