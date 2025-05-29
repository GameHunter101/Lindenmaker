use core::f32;
use std::collections::HashMap;

use spawner_component::SpawnerComponent;
use v4::{ V4, builtin_components::mesh_component::VertexDescriptor, scene};
use wgpu::vertex_attr_array;

mod lindenmayer;
mod lindenmayer_component;
mod spawner_component;

#[tokio::main]
async fn main() {
    let mut rules = HashMap::new();
    rules.insert('X', "F+[[X]-X]-F[-FX]+X".to_string());
    rules.insert('F', "FF".to_string());

    let mut engine = V4::builder()
        .features(wgpu::Features::POLYGON_MODE_LINE)
        .build()
        .await;


    let params = vec![
        Param(0, 0.0, 0.0),
        Param(0, 0.04, 0.0),
        Param(1, 25.0 * f32::consts::PI / 180.0, 0.0),
        Param(1, -25.0 * f32::consts::PI / 180.0, 0.0),
    ];

    let char_number_mapping: HashMap<char, u32> = vec![('X', 0), ('F', 1), ('+', 2), ('-', 3)]
        .into_iter()
        .collect();

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
                SpawnerComponent::new("-X", rules, &['+', '-', '[', ']'], 4, ident("mat"), params, char_number_mapping)
            ],
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
