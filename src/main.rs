#![windows_subsystem = "windows"]

use druid::{
    AppLauncher, LocalizedString, MenuDesc,
    WindowDesc,
};

mod data;
use data::AppState;

mod candidate;
mod controller;
mod selectors;
mod view;


use crate::view::build_grid;
use rustoku::basic::*;

pub fn main() {
    let main_window = WindowDesc::new(build_grid)
        .menu(make_menu())
        .with_min_size((600.0, 750.0))
        .title("Rustoku")
        .window_size((600.0, 750.0));

    AppLauncher::with_window(main_window)
        .launch(AppState::default())
        .expect("Failed to launch application");
}

fn make_menu() -> MenuDesc<AppState> {
    MenuDesc::new(LocalizedString::new("Edit"))
        .append(druid::platform_menus::common::copy())
        .append(druid::platform_menus::common::paste())
        .append(druid::platform_menus::common::cut())
}
