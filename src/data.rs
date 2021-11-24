use druid::{Lens, Data, Widget, Color};
use druid::im::Vector;
use std::ops::{Index, IndexMut};
use std::rc::Rc;
use rustoku::Sudoku;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub squares: Vector<Square>,
    #[data(ignore)]
    pub sud: Rc<Sudoku>,
}

#[derive(Clone, Data, Lens)]
pub struct Square {
    pub value: String,
    pub cands: Vector<IndCand>,
    pub index: usize,
}

#[derive(Clone, Data, Lens)]
pub struct IndCand {
    pub value: u8,
    pub status: Status,

}

#[derive(Clone, PartialEq, Data, Debug)]
pub enum Status {
    Active,
    Inactive,
    Selected,
}


impl Status {
    pub fn color(&self) -> Color {
        match self {
            Status::Active => Color::BLUE,
            Status::Inactive => Color::BLACK,
            Status::Selected => Color::YELLOW,
        }
    }
}
// impl Cand {
//     pub fn new (poss_fun: impl Fn((i32, i32)) -> Vec<u8>, row: i32, col: i32) -> Cand {
//         let poss = poss_fun((row,col));
//         let v: Vec<IndCand> = (1..=9).into_iter().map(|i|
//             if poss.contains(&i) {
//                 IndCand::new(i, Status::Active)
//             } else {
//                 IndCand::new(i, Status::Inactive)
//             }).collect();
//         Cand {
//             values: v.into(),
//         }
//     }
// }
//
// #[derive(Clone, Data, Lens)]
// pub struct Cand {
//     pub values: Vector<IndCand>,
// }
//
//
// impl Index<usize> for Cand {
//     type Output = IndCand;
//
//     fn index(&self, index: usize) -> &Self::Output {
//         &self.values[index]
//     }
// }
//
// impl IndexMut<usize> for Cand {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         &mut self.values[index]
//     }
// }