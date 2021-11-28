use druid::im::Vector;
use druid::{Color, Data, Lens, Widget, WidgetId};
use rustoku::{HumanSolve, Sudoku, Move};
use std::ops::{Index, IndexMut};
use std::rc::Rc;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub squares: Vector<Square>,
    pub selectedPair: IndexValuePair,
    pub hint_name: String,
    #[data(same_fn = "PartialEq::eq")]
    pub selectedId: WidgetId,
    #[data(ignore)]
    pub active_hint: Option<Move>,
    #[data(ignore)]
    pub sud: Sudoku,
    #[data(ignore)]
    pub solver: HumanSolve,
}

impl AppState {
    fn new(input: &str) -> Self {
        let sudoku = Sudoku::new(input)
            .unwrap();
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
            selectedPair: IndexValuePair::default(),
            selectedId: WidgetId::next(),
            active_hint: None,
            hint_name: "".to_owned(),
            solver: HumanSolve::new(),
        }

    }
}

impl Default for AppState {
    fn default() -> Self {
        // AppState::new(".5267.3.8.3...562767..325.128...61.5.6....2.4714523869827314956.9.267483346958712")
        AppState::new("..53.....8......2..7..1.5..4....53...1..7...6..32...8..6.5....9..4....3......97..")
    }
}

#[derive(Clone, Data, Debug, PartialEq)]
pub struct IndexValuePair {
    pub index: usize,
    pub value: u8,
}

impl IndexValuePair {
    pub fn new(index: usize, value: u8) -> IndexValuePair {
        IndexValuePair { index, value }
    }
}

impl Default for IndexValuePair {
    fn default() -> Self {
        IndexValuePair { index: 0, value: 0 }
    }
}

#[derive(Clone, Data, Lens)]
pub struct Square {
    pub value: String,
    pub cands: Vector<IndCand>,
    // pub index: usize,
}

#[derive(Clone, Data, Lens)]
pub struct IndCand {
    pub value: u8,
    pub status: Status,
    pub square_index: usize,
}

impl IndCand {
    pub fn left_mouse_click(&mut self) {
        match self.status {
            Status::Active => self.status = Status::Selected,
            Status::Selected => self.status = Status::Active,
            _ => (),
        }
    }
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
            Status::Active => Color::BLUE,
            Status::Inactive => Color::BLACK,
            Status::Selected => Color::YELLOW,
            Status::Involved => Color::GREEN,
            Status::Removable => Color::AQUA,
        }
    }
}
