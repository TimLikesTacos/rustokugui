
#![windows_subsystem = "windows"]



use druid::{AppLauncher, WindowDesc, Widget};

mod data;
use data::AppState;

mod view;
use view::build_ui;
use druid::im::Vector;
use rustoku::Sudoku;
use std::rc::Rc;
use crate::data::{IndCand, Status, Square};


pub fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Todo Tutorial")
        .window_size((500.0, 500.0));

    let sudoku = Sudoku::new(".5267.3.8.3...562767..325.128...61.5.6....2.4714523869827314956.9.267483346958712").unwrap();
//
    let values = sudoku.value_iter().map(|v| v.to_string()).collect::<Vec<String>>();
    let poss: Vec<Vector<u8>> = sudoku.possibilities_iter().map(|v| v.into()).collect();
    let squares = sudoku.value_iter().zip(sudoku.possibilities_iter()).enumerate().map(
        |(i, (v, p))| {

            let mut vec:Vector<IndCand> = Vector::new();
            for x in 1..=9u8 {
                let ind = if p.contains(&x) {
                    IndCand {
                        value: x,
                        status: Status::Active
                    }
                } else {
                    IndCand {
                        value: x,
                        status: Status::Inactive,
                    }
                };
                vec.push_back(ind);
            }
            Square {
                value: if v != 0 {v.to_string()} else {"".to_string()},
                cands: vec,
                index: i
            }
        }
    ).collect();
    let initial_state = AppState {
        squares,
        sud: Rc::new(sudoku),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
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