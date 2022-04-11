mod app;
pub mod beano;
pub mod common;
pub mod generator;
pub mod plane;
pub mod simulation;

use crate::app::{
    destroy_plane, load_plane_ui, render_plane, set_window_icon, setup, show_error,
    sim_controls_ui, update_grid,
};
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
        .add_startup_system(set_window_icon)
        .add_system(load_plane_ui)
        .add_system(show_error)
        .add_system(render_plane)
        .add_system(destroy_plane)
        .add_system(sim_controls_ui)
        .add_system(update_grid)
        .run();
}
