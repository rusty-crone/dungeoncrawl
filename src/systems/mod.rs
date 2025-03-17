use crate::prelude::*;

mod player_input;
mod map_renderer;
mod entity_render;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_renderer::map_renderer_system())
        .add_system(entity_render::entity_render_system())
        .build()
}