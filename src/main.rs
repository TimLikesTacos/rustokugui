#![windows_subsystem = "windows"]

use druid::{AppLauncher, Widget, WidgetId, WindowDesc};

mod data;
use data::AppState;

mod controller;
mod selectors;
mod view;
mod candidate;

use crate::view::build_grid;

pub fn main() {
    let main_window = WindowDesc::new(build_grid)
        .title("Rustoku")
        .window_size((600.0, 700.0));

    AppLauncher::with_window(main_window)
        .launch(AppState::default())
        .expect("Failed to launch application");
}
