use core::f32;
use std::collections::{HashMap, HashSet};

use lindenmayer::{progress, separate_stack_strings};
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

#[tokio::main]
async fn main() {
    let mut rules = HashMap::new();
    rules.insert('X', "F+[[X]-X]-F[-FX]+X".to_string());
    rules.insert('F', "FF".to_string());

    let string = (0..4).fold("-X".to_string(), |acc, _| {
        progress(&acc, &['+', '-', '[', ']'], &rules)
    });
    println!("String: {string}");
    let strings = separate_stack_strings(&string);
    let mut l_test = String::new();
    for string in &strings {
        l_test = "[".to_string() + &l_test + "]" + string;
    }
    println!("{l_test}");

    let mut engine = V4::builder()
        .features(wgpu::Features::POLYGON_MODE_LINE)
        .build()
        .await;

    let string = "F+F-F-F+F";

    let alphabet: Vec<char> = vec!['F', '+', '-']; //string.chars().collect::<HashSet<_>>().into_iter().collect();
    let string_as_nums: Vec<u32> = string
        .chars()
        .map(|c| alphabet.iter().position(|e| *e == c).unwrap() as u32)
        .collect();

    let device = engine.rendering_manager().device();

    /* let matrices = [
        nalgebra::Matrix3::new(1.0, 0.0, 0.1, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0),
        nalgebra::Rotation2::new(f32::consts::FRAC_PI_2).to_homogeneous(),
        nalgebra::Rotation2::new(-f32::consts::FRAC_PI_2).to_homogeneous(),
    ]; */

    /* let raw_matrices: [[[f32; 4]; 3]; 3] = matrices
    .into_iter()
    .map(|mat| {
        let arrs = nalgebra::Matrix4x3::from_rows(&[
            mat.row(0).into(),
            mat.row(1).into(),
            mat.row(2).into(),
            nalgebra::RowVector3::new(0.0_f32, 0.0, 0.0),
        ]);
        arrs.into()
    })
    .collect::<Vec<_>>()
    .try_into()
    .unwrap(); */

    let params = [
        Param(0, 0.1, 0.0),
        Param(1, f32::consts::FRAC_PI_2, 0.0),
        Param(1, -f32::consts::FRAC_PI_2, 0.0),
    ];

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
            },
            components: [
                MeshComponent(
                    vertices: vec![vec![
                        Vertex {
                            pos: [-0.2, 0.2, 0.0],
                        },
                        Vertex {
                            pos: [-0.2, -0.2, 0.0],
                        },
                        Vertex {
                            pos: [-0.2, -0.2, 0.0],
                        },
                        Vertex {
                            pos: [-0.2, -0.2, 0.0],
                        },
                    ]],
                    indices: vec![vec![0, 1, 2, 3]],
                    enabled_models: vec![0],
                ),
            ],
            computes: [
                Compute(
                    input: vec![
                        ShaderAttachment::Buffer(
                            ShaderBufferAttachment::new(
                                device,
                                bytemuck::cast_slice(&string_as_nums),
                                wgpu::BufferBindingType::Storage { read_only: true },
                                wgpu::ShaderStages::COMPUTE
                            )
                        ),
                        ShaderAttachment::Buffer(
                            ShaderBufferAttachment::new(
                                device,
                                bytemuck::cast_slice(&[params]),
                                wgpu::BufferBindingType::Uniform,
                                wgpu::ShaderStages::COMPUTE
                            )
                        )
                    ],
                    output:
                        ShaderAttachment::Buffer(
                            ShaderBufferAttachment::new(
                                device,
                                bytemuck::cast_slice(&[VertexPositions::default()]),
                                wgpu::BufferBindingType::Storage { read_only: false },
                                wgpu::ShaderStages::COMPUTE
                            )
                        ),
                    shader_path: "./shaders/compute.wgsl",
                    workgroup_counts: (1, 1, 1),
                )
            ]
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
    positions: [[f32; 4]; 20],
    count: u32,
    padding: [f32; 3],
}

impl Default for VertexPositions {
    fn default() -> Self {
        Self {
            positions: [[0.0; 4]; 20],
            count: 0,
            padding: [0.0; 3],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Param(i32, f32, f64);
