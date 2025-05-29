use std::collections::HashMap;

use v4::{
    builtin_actions::CreateEntityAction,
    builtin_components::mesh_component::MeshComponent,
    component,
    ecs::{
        actions::{Action, ActionQueue},
        component::{ComponentDetails, ComponentId, ComponentSystem},
        compute::Compute,
        entity::EntityId,
        material::{ShaderAttachment, ShaderBufferAttachment},
    },
};

use crate::{
    Param, Vertex,
    lindenmayer::{Rules, progress, separate_stack_strings},
    lindenmayer_component::LindenmayerComponent,
};

#[component]
pub struct SpawnerComponent {
    strings: Vec<String>,
    compute_path: &'static str,
    material_id: ComponentId,
    parameters: Vec<Param>,
    char_number_mapping: HashMap<char, u32>,
}

impl SpawnerComponent {
    pub const BUF_SIZE: usize = 70;
    pub fn new(
        init_string: &str,
        rules: Rules,
        constants: &[char],
        rank: u32,
        material_id: ComponentId,
        parameters: Vec<Param>,
        char_number_mapping: HashMap<char, u32>,
        compute_path: &'static str,
    ) -> Self {
        let strings = separate_stack_strings(&(0..rank).fold(init_string.to_string(), |acc, _| {
            progress(&acc, constants, &rules)
        }));

        let mut contents = std::fs::read_to_string(compute_path).unwrap();
        if contents.contains("const MESHES: u32 =") {
            contents = contents.split_once("\n").unwrap().1.to_string();
        }
        std::fs::write(
            compute_path,
            format!(
                "const MESHES: u32 = {}; const BUF_SIZE: u32 = {};\n{contents}",
                strings.len(),
                SpawnerComponent::BUF_SIZE
            ),
        )
        .unwrap();

        Self {
            strings,
            material_id,
            id: {
                use std::hash::{Hash, Hasher};
                let mut hasher = std::hash::DefaultHasher::new();
                std::time::Instant::now().hash(&mut hasher);
                hasher.finish()
            },
            parent_entity_id: EntityId::MAX,
            is_initialized: false,
            is_enabled: true,
            parameters,
            char_number_mapping,
            compute_path,
        }
    }
}

impl ComponentSystem for SpawnerComponent {
    fn initialize(&mut self, device: &wgpu::Device) -> ActionQueue {
        let num_arrs: Vec<[u32; SpawnerComponent::BUF_SIZE]> = self
            .strings
            .iter()
            .map(|str| {
                let num_str: Vec<u32> = str.chars().map(|c| self.char_number_mapping[&c]).collect();

                let len = num_str.len();
                let num_arr: [u32; SpawnerComponent::BUF_SIZE] = num_str
                    .into_iter()
                    .chain(vec![100; SpawnerComponent::BUF_SIZE - len])
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                num_arr
            })
            .collect();

        let compute = Compute::builder()
            .shader_path(self.compute_path)
            .input(vec![
                ShaderAttachment::Buffer(ShaderBufferAttachment::new(
                    device,
                    bytemuck::cast_slice(&num_arrs),
                    wgpu::BufferBindingType::Storage { read_only: true },
                    wgpu::ShaderStages::COMPUTE,
                    wgpu::BufferUsages::empty(),
                )),
                ShaderAttachment::Buffer(ShaderBufferAttachment::new(
                    device,
                    bytemuck::cast_slice(&self.parameters),
                    wgpu::BufferBindingType::Uniform,
                    wgpu::ShaderStages::COMPUTE,
                    wgpu::BufferUsages::empty(),
                )),
            ])
            .output(ShaderAttachment::Buffer(ShaderBufferAttachment::new(
                device,
                bytemuck::cast_slice(&vec![
                    [[123456.789_f32; 3]; SpawnerComponent::BUF_SIZE];
                    self.strings.len()
                ]),
                wgpu::BufferBindingType::Storage { read_only: false },
                wgpu::ShaderStages::COMPUTE,
                wgpu::BufferUsages::COPY_SRC,
            )))
            .workgroup_counts((self.strings.len() as u32, 1, 1))
            .build();

        let mesh_comp = MeshComponent::builder()
            .vertices(vec![vec![
                Vertex { pos: [0.0; 3] };
                SpawnerComponent::BUF_SIZE * self.strings.len()
            ]])
            .enabled_models(
                (0..self.strings.len())
                    .map(|i| {
                        (
                            0_usize,
                            Some(
                                (i * SpawnerComponent::BUF_SIZE) as u64
                                    ..((i + 1) * SpawnerComponent::BUF_SIZE) as u64,
                            ),
                        )
                    })
                    .collect(),
            )
            .build();

        let l_comp = LindenmayerComponent::builder()
            .mesh_component(mesh_comp.id())
            .compute_component(compute.id())
            .string_count(self.strings.len())
            .build();

        self.set_initialized();

        vec![Box::new(CreateEntityAction {
            entity_parent_id: Some(self.parent_entity_id),
            components: vec![Box::new(mesh_comp), Box::new(l_comp)],
            computes: vec![compute],
            active_material: Some(self.material_id),
            is_enabled: true,
        }) as Box<dyn Action + Send>]
    }
}
