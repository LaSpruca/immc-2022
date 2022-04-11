mod app;
pub mod beano;
pub mod common;
pub mod generator;
pub mod plane;
pub mod simulation;

use crate::app::{destory_plane, load_plane_ui, render_plane, setup, show_error};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Beno Simulator 2022".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb_u8(10, 5, 20)))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_system(load_plane_ui)
        .add_system(show_error)
        .add_system(render_plane)
        .add_system(destory_plane)
        .run();
}
