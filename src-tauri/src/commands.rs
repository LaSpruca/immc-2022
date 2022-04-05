use crate::plane::{load_grid_from_img, Error, Grid};
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
