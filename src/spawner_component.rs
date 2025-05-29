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
    material_id: ComponentId,
    parameters: Vec<Param>,
    char_number_mapping: HashMap<char, u32>,
}

impl SpawnerComponent {
    pub fn new(
        init_string: &str,
        rules: Rules,
        constants: &[char],
        rank: u32,
        material_id: ComponentId,
        parameters: Vec<Param>,
        char_number_mapping: HashMap<char, u32>,
    ) -> Self {
        Self {
            strings: separate_stack_strings(&(0..rank).fold(init_string.to_string(), |acc, _| {
                progress(&acc, constants, &rules)
            })),
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
        }
    }
}

impl ComponentSystem for SpawnerComponent {
    fn initialize(&mut self, device: &wgpu::Device) -> ActionQueue {
        dbg!(self.strings.len());
        let actions: ActionQueue = self
            .strings
            .iter()
            .map(|str| {
                let num_str: Vec<u32> = str.chars().map(|c| self.char_number_mapping[&c]).collect();
                let len = num_str.len();
                let num_arr: [u32; 250] = num_str
                    .into_iter()
                    .chain(vec![100; 250 - len])
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                let mesh_comp = MeshComponent::builder()
                    .vertices(vec![vec![Vertex { pos: [0.0; 3] }; 250]])
                    .enabled_models(vec![0])
                    .build();
                let compute = Compute::builder()
                    .shader_path("./shaders/compute.wgsl")
                    .input(vec![
                        ShaderAttachment::Buffer(ShaderBufferAttachment::new(
                            device,
                            bytemuck::cast_slice(&num_arr),
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
                        bytemuck::cast_slice(&[[0.0; 3]; 250]),
                        wgpu::BufferBindingType::Storage { read_only: false },
                        wgpu::ShaderStages::COMPUTE,
                        wgpu::BufferUsages::COPY_SRC,
                    )))
                    .workgroup_counts((1, 1, 1))
                    .build();

                let l_comp = LindenmayerComponent::builder()
                    .mesh_component(mesh_comp.id())
                    .compute_component(compute.id())
                    .build();
                Box::new(CreateEntityAction {
                    entity_parent_id: Some(self.parent_entity_id),
                    components: vec![Box::new(mesh_comp), Box::new(l_comp)],
                    computes: vec![compute],
                    active_material: Some(self.material_id),
                    is_enabled: true,
                }) as Box<dyn Action + Send>
            })
            .collect();

        self.set_initialized();

        actions
    }
}
