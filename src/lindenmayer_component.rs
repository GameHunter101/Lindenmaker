use v4::{
    builtin_components::mesh_component::MeshComponent, component, ecs::{
        actions::ActionQueue,
        component::{ComponentId, ComponentSystem}, compute::Compute, material::ShaderAttachment,
    }
};
use wgpu::Buffer;

use crate::{Vertex, VertexPositions};

#[component]
pub struct LindenmayerComponent {
    compute_component: ComponentId,
    compute_buffer: Option<Buffer>,
    mesh_component: ComponentId,
    vertex_buffer: Option<Buffer>,
}

#[async_trait::async_trait]
impl ComponentSystem for LindenmayerComponent {
    async fn update(
        &mut self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _input_manager: &winit_input_helper::WinitInputHelper,
        other_components: &[&mut v4::ecs::component::Component],
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
        if self.compute_buffer.is_none() && self.vertex_buffer.is_none() {
            for comp in other_components {
                if comp.id() == self.compute_component {
                    let compute: &Compute = comp.downcast_ref().expect("Bad compute component ID");
                    if let Some(ShaderAttachment::Buffer(attachment)) = compute.output_attachments() {
                        let buffer = attachment.buffer().clone();
                        self.compute_buffer = Some(buffer);
                    }
                }

                if comp.id() == self.mesh_component {
                    let compute: &MeshComponent<Vertex> = comp.downcast_ref().expect("Bad mesh component ID");
                    if let Some(buffers) = compute.vertex_buffer() {
                        self.compute_buffer = Some(buffers[0].clone());
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
            if let Some(vertex_buffer) = &self.vertex_buffer {
                encoder.copy_buffer_to_buffer(compute_buffer, 0, vertex_buffer, 0, std::mem::size_of::<VertexPositions>() as u64);
            }
        }
    }
}
