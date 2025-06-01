use std::time::Instant;

use async_trait::async_trait;
use v4::{
    builtin_actions::RegisterUiComponentAction,
    component,
    ecs::{
        actions::ActionQueue,
        component::{ComponentDetails, ComponentSystem, UpdateParams},
    },
    engine_management::font_management::{
        FontFamily, TextAttributes, TextComponentProperties, TextDisplayInfo,
    },
};

#[component]
pub struct FpsComponent {
    #[default(Instant::now())]
    last_frame_time: Instant,
    #[default(Instant::now())]
    second_to_last_frame_time: Instant,
    #[default(0)]
    frames_in_second: u32,
    #[default(0)]
    last_second_frame_count: u32,
    #[default(Instant::now())]
    second_tracker: Instant,
}

#[async_trait]
impl ComponentSystem for FpsComponent {
    fn initialize(&mut self, _device: &wgpu::Device) -> ActionQueue {
        self.set_initialized();
        vec![Box::new(RegisterUiComponentAction {
            component_id: self.id(),
            text_component_properties: Some(TextComponentProperties {
                text: "0.0 ms | 0 fps".to_string(),
                text_attributes: TextAttributes {
                    color: glyphon::Color::rgb(255, 255, 255),
                    family: FontFamily::SansSerif,
                    stretch: glyphon::Stretch::Normal,
                    style: glyphon::Style::Normal,
                    weight: glyphon::Weight(100),
                },
                text_metrics: glyphon::Metrics {
                    font_size: 15.0,
                    line_height: 40.0,
                },
                text_display_info: TextDisplayInfo {
                    on_screen_width: 1000.0,
                    on_screen_height: 1000.0,
                    top_left_pos: [20.0, 20.0],
                    scale: 1.0,
                },
            }),
        })]
    }

    async fn update(
        &mut self,
        UpdateParams { engine_details, .. }: UpdateParams<'_>,
    ) -> ActionQueue {
        if engine_details.last_frame_instant != self.last_frame_time {
            self.second_to_last_frame_time = self.last_frame_time;
            self.last_frame_time = engine_details.last_frame_instant;
        }

        if self.second_tracker.elapsed().as_millis() <= 1000 {
            self.frames_in_second += 1;
        } else {
            self.last_second_frame_count = self.frames_in_second;
            self.second_tracker = Instant::now();
            self.frames_in_second = 0;
        }

        vec![Box::new(v4::builtin_actions::UpdateTextComponentAction {
            component_id: self.id,
            text: Some(format!(
                "{:.04} ms | {} FPS",
                (self.last_frame_time - self.second_to_last_frame_time).as_secs_f32() * 1000.0,
                self.last_second_frame_count
            )),
            text_attributes: None,
            text_metrics: None,
            text_display_info: None,
        })]
    }
}
