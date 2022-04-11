use crate::generator::generate_random;
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align, Layout},
    EguiContext,
};
use native_dialog::FileDialog;

const TILE_SIZE: f32 = 7.0;

use crate::plane::{load_grid_from_img, GridTile};

#[derive(Component)]
pub struct Plane(Vec<Vec<GridTile>>);
#[derive(Component)]
pub struct Error(String, String);
#[derive(Component)]
pub struct RenderPlane;
#[derive(Component)]
pub struct DestoryPlane;

pub fn load_plane_ui(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    windows: Res<Windows>,
    plane_query: Query<Entity, With<Plane>>,
) {
    let win = windows.get_primary().unwrap();

    egui::Window::new("Load plane")
        .default_pos((win.width() - 220.0, 20.0))
        .show(egui_context.ctx_mut(), |ui| {
            if let Some(entity) = plane_query.iter().next() {
                if ui.button("Unload Plane").clicked() {
                    commands.entity(entity).insert(DestoryPlane);
                }
            } else {
                if ui.button("Load plane").clicked() {
                    match FileDialog::new()
                        .add_filter("Image files", &["png", "jpg", "jpeg"])
                        .set_location(&std::env::current_dir().unwrap())
                        .show_open_single_file()
                    {
                        Ok(Some(path)) => {
                            match load_grid_from_img(path) {
                                Ok(val) => {
                                    commands.spawn().insert(Plane(val)).insert(RenderPlane);
                                }
                                Err(err) => {
                                    commands.spawn().insert(Error(
                                        "Error loading plane".into(),
                                        err.to_string(),
                                    ));
                                }
                            };
                        }
                        Ok(_) => {}
                        Err(err) => {
                            commands
                                .spawn()
                                .insert(Error("Error opening file picker".into(), err.to_string()));
                        }
                    }
                }
            }
        });
}

pub fn show_error(
    mut commands: Commands,
    errors: Query<(Entity, &Error)>,
    mut egui_context: ResMut<EguiContext>,
    windows: Res<Windows>,
) {
    let win = windows.get_primary().unwrap();

    for (entity, error) in errors.iter() {
        egui::Window::new(&error.0)
            .default_pos((win.width() / 2.0, win.height() / 2.0))
            .show(egui_context.ctx_mut(), |ui| {
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.horizontal(|ui| {
                        ui.label(&error.1);

                        if ui.button("Ok").clicked() {
                            commands.entity(entity).despawn();
                        }
                    });
                })
            });
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn render_plane(query: Query<(Entity, &Plane), With<RenderPlane>>, mut commands: Commands) {
    if let Some((entity, Plane(plane))) = query.iter().next() {
        commands.entity(entity).remove::<RenderPlane>();
        let width = plane.len();

        for (x, column) in plane.iter().enumerate() {
            let x_pos = (x as f32 - (width as f32 / 2f32)) * (TILE_SIZE + 4.0);
            let height = column.len();

            for (y, tile) in column.iter().enumerate() {
                let y_pos = -(y as f32 - (height as f32 / 2f32)) * (TILE_SIZE + 4.0);
                info!("{}", Vec3::new(x_pos, y_pos, 0.0));

                let child = commands
                    .spawn()
                    .insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: color_from_tile(tile),
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..Default::default()
                        },
                        global_transform: GlobalTransform {
                            translation: Vec3::new(x_pos, y_pos, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .id();

                commands.entity(entity).add_child(child);
            }
        }
    }
}

pub fn destory_plane(query: Query<Entity, With<DestoryPlane>>, mut commands: Commands) {
    if let Some(plane) = query.iter().next() {
        commands.entity(plane).despawn_recursive();
    }
}

#[inline(always)]
fn color_from_tile(tile: &GridTile) -> Color {
    match tile {
        GridTile::None => Color::rgb_u8(0, 0, 0),
        GridTile::Walkway => Color::rgb_u8(255, 255, 255),
        GridTile::Seat => Color::rgb_u8(150, 150, 150),
        GridTile::Entry => Color::rgb_u8(0, 255, 0),
    }
}
