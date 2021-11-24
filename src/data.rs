use druid::{Lens, Data, Widget};
use druid::im::Vector;
use crate::view::Cand;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    // pub first: String,
    // pub second: String,
    // pub count: Vector<Counter>,
    pub cands: Cand,
}

#[derive(Clone, Data, Lens)]
pub struct Counter {
    pub count: usize
}

impl Counter {
    pub fn string (&self) -> String {
        self.count.to_string()
    }
}