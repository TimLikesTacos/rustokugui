use druid::im::Vector;
use druid::{Color, Data, Lens, Widget, WidgetId};
use rustoku::{HumanSolve, Sudoku, Move, SudError};
use std::ops::{Index, IndexMut};
use std::rc::Rc;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub squares: Vector<Square>,
    pub hint_name: String,
    #[data(ignore)]
    pub multi_select: bool,
    #[data(ignore)]
    pub selected_pairs: HashSet<CandidateInfo>,
    #[data(ignore)]
    pub active_hint: Option<Move>,
    #[data(ignore)]
    pub sud: Sudoku,
    #[data(ignore)]
    pub solver: HumanSolve,
}

impl AppState {
    fn new(input: &str) -> Self {
        if let Ok(sudoku) = Sudoku::new(input) {
            // Validate the puzzle
            if let Err(error) = sudoku.validate() {
                match error {
                    SudError::MultipleSolution(_) => (), // this app will allow multiple solutions
                    _ => return AppState::default() // any invalid puzzle, return the default puzzle.
                }
            }

            let squares = sudoku
                .value_iter()
                .zip(sudoku.possibilities_iter())
                .enumerate()
                .map(|(i, (v, p))| {
                    let mut vec: Vector<IndCand> = Vector::new();
                    for x in 1..=9u8 {
                        let ind = if p.contains(&x) {
                            IndCand {
                                value: x,
                                status: Status::Active,
                                square_index: i,
                            }
                        } else {
                            IndCand {
                                value: x,
                                status: Status::Inactive,
                                square_index: i,
                            }
                        };
                        vec.push_back(ind);
                    }

                    Square {
                        value: if v != 0 {
                            v.to_string()
                        } else {
                            "".to_string()
                        },
                        cands: vec,
                    }
                })
                .collect();

            AppState {
                squares,
                sud: sudoku,
                selected_pairs: HashSet::new(),
                multi_select: false,
                active_hint: None,
                hint_name: "".to_owned(),
                solver: HumanSolve::new(),
            }

        } else {
            AppState::default()
        }
    }


}

impl Default for AppState {
    fn default() -> Self {
        // This is a known good valid puzzle.  If this string is replaced with an invalid puzzle, there will be
        // an infinite loop between `.new()` and `.default()`
        AppState::new(".28..7....16.83.7.....2.85113729.......73........463.729..7.......86.14....3..7..")
    }
}

#[derive(Clone, Data, Debug, PartialEq, Eq)]
pub struct CandidateInfo {
    pub index: usize,
    pub value: u8,
    #[data(ignore)]
    pub id: WidgetId,
}

impl Hash for CandidateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl CandidateInfo {
    pub fn new(index: usize, value: u8, id: WidgetId) -> CandidateInfo {
        CandidateInfo { index, value, id }
    }
}


#[derive(Clone, Data, Lens)]
pub struct Square {
    pub value: String,
    pub cands: Vector<IndCand>,
}

#[derive(Clone, Data, Lens)]
pub struct IndCand {
    pub value: u8,
    pub status: Status,
    pub square_index: usize,
}

#[derive(Clone, PartialEq, Data, Debug)]
pub enum Status {
    Active,
    Inactive,
    Selected,
    Involved,
    Removable,
}

impl Status {
    pub fn color(&self) -> Color {
        match self {
            Status::Active => Color::BLACK,
            Status::Inactive => Color::BLACK,
            Status::Selected => Color::BLUE,
            Status::Involved => Color::GREEN,
            Status::Removable => Color::RED,
        }
    }
}
