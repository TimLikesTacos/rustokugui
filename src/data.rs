use druid::{Lens, Data, Widget, Color};
use druid::im::Vector;
use std::ops::{Index, IndexMut};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub cands: Cand,
}

#[derive(Clone, Data, Lens)]
pub struct Cand {
    pub values: Vector<IndCand>,
}


impl Index<usize> for Cand {
    type Output = IndCand;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl IndexMut<usize> for Cand {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
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
impl Cand {
    pub fn new () -> Cand {
        let v: Vec<IndCand> = (1..=9).into_iter().map(|i|
            if i & 1 > 0 {
                IndCand::new(i, Status::Active)
            } else {
                IndCand::new(i, Status::Inactive)
            }).collect();
        Cand {
            values: v.into(),
        }
    }
}