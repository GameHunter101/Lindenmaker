use v4::{
    builtin_components::mesh_component::MeshComponent,
    component,
    ecs::{
        actions::ActionQueue,
        component::{ComponentDetails, ComponentId, ComponentSystem},
        material::ShaderAttachment,
    },
};
use wgpu::Buffer;

use crate::Vertex;

#[component]
pub struct LindenmayerComponent {
    string_count: usize,
    compute_component: ComponentId,
    #[default(None)]
    compute_buffer: Option<Buffer>,
    mesh_component: ComponentId,
    #[default(None)]
    vertex_buffers: Option<Vec<Buffer>>,
}

#[async_trait::async_trait]
impl ComponentSystem for LindenmayerComponent {
    async fn update(
        &mut self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _input_manager: &winit_input_helper::WinitInputHelper,
        other_components: &[&mut v4::ecs::component::Component],
        computes: &[v4::ecs::compute::Compute],
        _materials: &[&mut v4::ecs::material::Material],
        _engine_details: &v4::EngineDetails,
        _workload_outputs: &std::collections::HashMap<
            ComponentId,
            Vec<v4::ecs::scene::WorkloadOutput>,
        >,
        _entities: &std::collections::HashMap<v4::ecs::entity::EntityId, v4::ecs::entity::Entity>,
        _entity_component_groups: std::collections::HashMap<
            v4::ecs::entity::EntityId,
            std::ops::Range<usize>,
        >,
        _active_camera: Option<ComponentId>,
    ) -> ActionQueue {
        if self.compute_buffer.is_none() && self.vertex_buffers.is_none() {
            for compute in computes {
                if compute.id() == self.compute_component {
                    if let Some(ShaderAttachment::Buffer(attachment)) = compute.output_attachments()
                    {
                        let buffer = attachment.buffer().clone();
                        self.compute_buffer = Some(buffer);
                    }
                }
            }
            for comp in other_components {
                if comp.id() == self.mesh_component {
                    let mesh: &MeshComponent<Vertex> =
                        comp.downcast_ref().expect("Bad mesh component ID");
                    if let Some(buffers) = mesh.vertex_buffers() {
                        self.vertex_buffers = Some(buffers.iter().cloned().collect());
                    }
                }
            }
        }

        Vec::new()
    }

    fn command_encoder_operations(
        &self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        _other_components: &[&v4::ecs::component::Component],
    ) {
        if let Some(compute_buffer) = &self.compute_buffer {
            if let Some(vertex_buffers) = &self.vertex_buffers {
                encoder.copy_buffer_to_buffer(
                    compute_buffer,
                    0,
                    &vertex_buffers[0],
                    0,
                    (std::mem::size_of::<
                        [[f32; 3]; crate::spawner_component::SpawnerComponent::BUF_SIZE],
                    >() * self.string_count) as u64,
                );
            }
        }
    }
}
