use crate::{
    beano::{Beano, Beanoz},
    common::Point,
    plane::{load_grid_from_img, Error, Grid},
};
use std::collections::HashMap;
use tauri::api::dialog::blocking::FileDialogBuilder;

#[tauri::command]
pub async fn load_image() -> Result<Grid, Error> {
    let path = FileDialogBuilder::new()
        .add_filter("Image", &["png", "jpg"])
        .set_title("Select image")
        .pick_file()
        .unwrap();

    load_grid_from_img(path)
}

#[tauri::command]
pub fn generate_random(grid: Grid) -> Vec<(Point<usize>, Vec<Beano>)> {
    crate::generator::generate_random(grid)
        .into_iter()
        .collect()
}

#[tauri::command]
pub fn run_iteration(
    beanoz: Vec<(Point<usize>, Beano)>,
    spawner: Vec<(Point<usize>, Vec<Beano>)>,
) -> (Vec<(Point<usize>, Beano)>, Vec<(Point<usize>, Vec<Beano>)>) {
    let mut beanoz = Beanoz::from(beanoz);
    let mut spawner: HashMap<Point<usize>, Vec<Beano>> = spawner.into_iter().collect();

    crate::simulation::run_iteration(&mut beanoz, &mut spawner);

    (beanoz.into(), spawner.into_iter().collect())
}
