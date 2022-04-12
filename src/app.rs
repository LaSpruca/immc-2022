use crate::generator::generate_random;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ElementState;
use bevy::{prelude::*, window::WindowId, winit::WinitWindows};
use bevy_egui::egui::{RichText, WidgetText};
use bevy_egui::{
    egui::{self, Align, Layout},
    EguiContext,
};
use image::io::Reader as ImageReader;
use native_dialog::FileDialog;
use std::io::Cursor;
use winit::window::Icon;

use crate::plane::{load_grid_from_img, GridTile};

/// Component to tell bevy that a plane needs rendering
#[derive(Component)]
pub struct RenderPlane;
/// Component to tell bevy to yeet a plane outa existence
#[derive(Component)]
pub struct DestoryPlane;

/// Component to store a grid of points
#[derive(Component)]
pub struct Plane(Vec<Vec<GridTile>>);
/// An error component that will create a popup
#[derive(Component)]
pub struct Error(String, String);
/// The position of something unscaled
#[derive(Component)]
pub struct UnscaledPosition(f32, f32);

/// The configuration for a plane
#[derive(Component)]
pub struct PlaneConfig {
    pub tile_size: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for PlaneConfig {
    fn default() -> Self {
        Self {
            tile_size: 7.0f32,
            width: 0.0,
            height: 0.0,
        }
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let image_bytes = include_bytes!("../icon.png");

    let (icon_rgba, icon_width, icon_height) = {
        let image = ImageReader::new(Cursor::new(image_bytes))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary.set_window_icon(Some(icon));
}

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
                                    let width = val.len() as f32;
                                    let height = val[0].len() as f32;
                                    commands
                                        .spawn()
                                        .insert(Plane(val))
                                        .insert(RenderPlane)
                                        .insert(PlaneConfig {
                                            width,
                                            height,
                                            ..Default::default()
                                        });
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

pub fn render_plane(
    query: Query<(Entity, &Plane, &PlaneConfig), With<RenderPlane>>,
    mut commands: Commands,
) {
    if let Some((
        entity,
        Plane(plane),
        PlaneConfig {
            tile_size,
            width,
            height,
        },
    )) = query.iter().next()
    {
        commands.entity(entity).remove::<RenderPlane>();

        for (x, column) in plane.iter().enumerate() {
            let x_pos = (x as f32 - (width / 2f32)) * (tile_size + 4.0);

            for (y, tile) in column.iter().enumerate() {
                let y_pos = -(y as f32 - (height / 2f32)) * (tile_size + 4.0);

                let child = commands
                    .spawn()
                    .insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: color_from_tile(tile),
                            custom_size: Some(Vec2::new(*tile_size, *tile_size)),
                            ..Default::default()
                        },
                        global_transform: GlobalTransform {
                            translation: Vec3::new(x_pos, y_pos, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(UnscaledPosition(x as f32, y as f32))
                    .id();

                commands.entity(entity).add_child(child);
            }
        }
    }
}

pub fn destroy_plane(query: Query<Entity, With<DestoryPlane>>, mut commands: Commands) {
    if let Some(plane) = query.iter().next() {
        commands.entity(plane).despawn_recursive();
    }
}

pub fn sim_controls_ui(
    mut query: Query<&mut PlaneConfig>,
    mut egui_context: ResMut<EguiContext>,
    windows: Res<Windows>,
) {
    if let Some(cfg) = query.iter_mut().next() {
        let win = windows.get_primary().unwrap();

        let cfg = cfg.into_inner();

        egui::Window::new("Simulation Controls")
            .default_pos((win.width() - 220.0, 80.0))
            .show(egui_context.ctx_mut(), |ui| {
                ui.vertical(|ui| {
                    ui.label("Tile size");
                    ui.add(egui::Slider::new(&mut cfg.tile_size, 5.0..=35.0));
                })
            });
    }
}

pub fn update_grid(
    mut tf_query: Query<(&mut GlobalTransform, &mut Sprite, &UnscaledPosition)>,
    plane_cfg: Query<&PlaneConfig>,
) {
    if let Some(PlaneConfig {
        tile_size,
        width,
        height,
    }) = plane_cfg.iter().next()
    {
        for (transform, sprite, UnscaledPosition(x, y)) in tf_query.iter_mut() {
            let x_pos = (x - (width / 2f32)) * (tile_size + 4.0);
            let y_pos = -(y - (height / 2f32)) * (tile_size + 4.0);

            transform.into_inner().translation = Vec3::new(x_pos, y_pos, 0.0);
            sprite.into_inner().custom_size = Some(Vec2::new(*tile_size, *tile_size));
        }
    }
}

pub fn update_camera_text(
    camera: Query<&Transform, With<Camera>>,
    mut egui_context: ResMut<EguiContext>,
) {
    if let Some(camera) = camera.iter().next() {
        egui::Window::new("Camera Position").show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("X: ").strong());
                ui.label(camera.translation.x.to_string());
            });

            ui.horizontal(|ui| {
                ui.label(RichText::new("Y: ").strong());
                ui.label(camera.translation.y.to_string());
            });
        });
    }
}

pub fn move_camera(
    mut key_evr: EventReader<KeyboardInput>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    if let Some(camera) = camera.iter_mut().next() {
        let camera = camera.into_inner();
        for ev in key_evr.iter() {
            match ev.state {
                ElementState::Pressed => {
                    if let Some(key) = ev.key_code {
                        match key {
                            KeyCode::W => {
                                camera.translation.y += 2.5;
                            }
                            KeyCode::S => {
                                camera.translation.y -= 2.5;
                            }
                            KeyCode::A => {
                                camera.translation.x -= 2.5;
                            }
                            KeyCode::D => {
                                camera.translation.x += 2.5;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

#[inline(always)]
fn color_from_tile(tile: &GridTile) -> Color {
    match tile {
        GridTile::None => Color::rgb_u8(255, 0, 0),
        GridTile::Walkway => Color::rgb_u8(255, 255, 255),
        GridTile::Seat => Color::rgb_u8(150, 150, 150),
        GridTile::Entry => Color::rgb_u8(0, 255, 0),
    }
}
