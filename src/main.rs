#![warn(rust_2018_idioms, clippy::all)]

use amethyst::prelude::*;
use amethyst::{
    core::*,
    input::InputBundle,
    input::StringBindings,
    renderer::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
    },
    utils::*,
    LoggerConfig,
};

mod states;
use states::*;

mod components;
use crate::components::Tile;

mod board;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig::default());

    let app_root = application_root_dir()?;
    let assets_directory = app_root.join("assets");
    let display_config_path = app_root.join("config/display.ron");
    let input_config_path = app_root.join("config/input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(input_config_path)?,
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?;
    let mut builder = Application::build(assets_directory, Starting)?;
    builder.world.register::<Tile>();
    let mut game = builder.build(game_data)?;
    game.run();
    Ok(())
}
