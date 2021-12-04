use crate::data::{CandidateInfo};
pub use druid::{Selector, WidgetId};

pub(crate) const CAND_SELECTED: Selector<CandidateInfo> = Selector::new("cand.selected");
pub(crate) const CAND_DESELECT: Selector<()> = Selector::new("cand.deselect");
pub(crate) const CAND_SELECT: Selector<()> = Selector::new("cand.select");

