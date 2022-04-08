// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]
//
pub mod beano;
mod commands;
pub mod common;
pub mod generator;
pub mod plane;
pub mod simulation;
//
// use crate::beano::{Beano, Kind};
// use crate::common::{Direction, Point};
// use commands::*;
// use std::collections::HashMap;
//
// fn main() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![
//             load_image,
//             generate_random,
//             run_iteration
//         ])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
use crate::beano::{Beano, Beanoz, Kind};
use crate::common::{Direction, Point};
use crate::simulation::run_iteration;
use std::collections::HashMap;

fn main() {
    let mut spawner = HashMap::new();
    spawner.insert(
        Point { x: 2, y: 3 },
        vec![
            Beano::new(
                Kind::Adult,
                Point { x: 1, y: 2 },
                vec![Direction::Up],
                1,
                false,
            ),
            Beano::new(
                Kind::Adult,
                Point { x: 3, y: 2 },
                vec![Direction::Up],
                1,
                false,
            ),
            Beano::new(
                Kind::Adult,
                Point { x: 4, y: 2 },
                vec![Direction::Up],
                1,
                false,
            ),
            Beano::new(
                Kind::Adult,
                Point { x: 1, y: 2 },
                vec![Direction::Up, Direction::Up],
                1,
                false,
            ),
        ],
    );

    let mut beanoz = Beanoz::from(vec![]);

    for i in 0..300 {
        println!("------------ {i} ------------");
        run_iteration(&mut beanoz, &mut spawner);
    }
}
