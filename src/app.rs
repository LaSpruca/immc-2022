use crate::generator::generate_random;
use bevy::prelude::*;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use bevy_egui::{
    egui::{self, Align, Layout},
    EguiContext,
};
use native_dialog::FileDialog;
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
/// The position of somthing unscaled
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

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        #[cfg(not(debug_assertions))]
        let path = {
            let mut temp = std::env::current_exe().unwrap();
            temp.pop();
            temp.push("icon.png");
            temp
        };
        #[cfg(debug_assertions)]
        let path = {
            let mut temp = std::env::current_dir().unwrap();
            temp.push("icon.png");
            temp
        };

        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
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
                    ui.add(egui::Slider::new(&mut cfg.tile_size, 5.0..=35.0).text("value"));
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

#[inline(always)]
fn color_from_tile(tile: &GridTile) -> Color {
    match tile {
        GridTile::None => Color::rgb_u8(0, 0, 0),
        GridTile::Walkway => Color::rgb_u8(255, 255, 255),
        GridTile::Seat => Color::rgb_u8(150, 150, 150),
        GridTile::Entry => Color::rgb_u8(0, 255, 0),
    }
}
