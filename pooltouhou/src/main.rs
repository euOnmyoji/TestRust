use amethyst::{
    core::TransformBundle,
    ecs::Entity,
    input::{InputBundle, StringBindings, VirtualKeyCode},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderFlat3D, RenderToWindow},
        RenderingBundle,
        types::DefaultBackend,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};
use amethyst::core::frame_limiter::FrameRateLimitStrategy;

use crate::script::ScriptGameCommand;

mod script;

mod render;
mod ui;
mod handles;
pub mod systems;
mod states;
mod component;
mod input;
mod audio;


// https://doc.rust-lang.org/book/

pub const PLAYER_Z: f32 = 0.0;

pub struct CoreStorage {
    player: Option<Entity>,
    last_input: input::InputData,
    cur_input: input::InputData,
    temp_input: input::InputData,
    commands: Vec<ScriptGameCommand>,
    tick: u128,
    tick_sign: bool,
    al: audio::OpenalData,
}

impl Default for CoreStorage {
    fn default() -> Self {
        Self {
            player: None,
            last_input: input::InputData::empty(),
            cur_input: input::InputData::empty(),
            temp_input: input::InputData::empty(),
            commands: vec![],
            tick: 0,
            tick_sign: false,
            al: audio::OpenalData::default(),
        }
    }
}

impl CoreStorage {
    pub fn swap_input(&mut self) {
        std::mem::swap(&mut self.last_input, &mut self.cur_input);
        std::mem::swap(&mut self.cur_input, &mut self.temp_input);
    }

    pub fn is_pressed(&self, keys: &[VirtualKeyCode]) -> bool {
        let last_input = &self.last_input;
        let cur_input = &self.cur_input;

        let any_last_not_input = keys.iter().any(|key| !last_input.pressing.contains(key));
        let all_cur_input = keys.iter().all(|key| cur_input.pressing.contains(key));

        return any_last_not_input && all_cur_input;
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let res_root = if app_root.join("res").exists() { app_root.join("res") } else { app_root };
    let display_config_path = res_root.join("config").join("display.ron");
    let assets_dir = res_root.join("assets");
    let game_data = GameDataBuilder::default()
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0])
            )
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderFlat3D::default())
            .with_plugin(RenderUi::default())
            .with_plugin(render::RenderInvertColorCircle::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(input::InputDataSystem, "main_input_system", &["input_system"])
        .with(systems::GameSystem, "main_game_system", &["main_input_system"])
        .with(systems::AnimationSystem, "main_anime_system", &[])
        .with(systems::DebugSystem::default(), "debug_system", &[]);
    let mut game = Application::build(assets_dir, states::Loading::default())?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 0)
        .build(game_data)?;
    game.run();
    Ok(())
}
