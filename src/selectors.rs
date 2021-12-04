use crate::data::{CandidateInfo, IndCand};
pub use druid::{Selector, WidgetId};

pub(crate) const CAND_SELECTED: Selector<CandidateInfo> = Selector::new("cand.selected");
pub(crate) const CAND_INDEX: Selector<usize> = Selector::new("cand.index");
pub(crate) const CAND_VALUE: Selector<u8> = Selector::new("cand.value");
pub(crate) const CAND_DESELECT: Selector<()> = Selector::new("cand.deselect");
pub(crate) const CAND_SELECT: Selector<()> = Selector::new("cand.select");

pub(crate) const SET_VALUE: Selector<IndCand> = Selector::new("cand.set");
pub(crate) const REMOVE_POT: Selector<IndCand> = Selector::new("cand.remove");

pub(crate) const NEW_PUZZLE: Selector<String> = Selector::new("puzzle.new");
