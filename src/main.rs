#![windows_subsystem = "windows"]

use druid::{AppLauncher, Widget, WidgetId, WindowDesc};

mod data;
use data::AppState;

mod controller;
mod selectors;
mod view;

use crate::data::{IndCand, IndexValuePair, Square, Status};
use druid::im::Vector;
use rustoku::Sudoku;
use std::rc::Rc;
use view::build_ui;

pub fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Rustoku")
        .window_size((500.0, 500.0));

    AppLauncher::with_window(main_window)
        .launch(AppState::default())
        .expect("Failed to launch application");
}

// pub fn main() {
//     let window = WindowDesc::new(||build_grid(3))
//         .window_size((500., 500.))
//         .resizable(true)
//         .title(
//             LocalizedString::new("Rustoku").with_placeholder("Rustoku - Sudoku in Rust"),
//         );
//     let sudoku = Sudoku::new(".5267.3.8.3...562767..325.128...61.5.6....2.4714523869827314956.9.267483346958712").unwrap();
//
//     let values = sudoku.value_iter().map(|v| v.to_string()).collect::<Vec<String>>();
//     let poss: Vec<Vector<u8>> = sudoku.possibilities_iter().map(|v| v.into()).collect();
//
//
//     let values: Values<String> =  Values {
//         values,
//     };
//
//     let poss: Values<Vector<u8>> = Values {
//         values: poss,
//     };
//
//     let sudoku = AppState {
//         sud: Rc::new(sudoku),
//         values,
//         possibilities: poss,
//     };
//     AppLauncher::with_window(window)
//         .launch(sudoku)
//         .expect("launch failed");
// }

// fn main() -> Result<(), PlatformError> {
//     AppLauncher::with_window(WindowDesc::new(||build_ui())).launch(())?;
//     Ok(())
// }
