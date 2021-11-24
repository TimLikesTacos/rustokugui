use druid::{widget::Label, Widget, WidgetExt, Env, Color, LensExt, EventCtx, Event, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, Size, PaintCtx, RenderContext};

use crate::data::*;
use druid::widget::{Flex, Button, List, Container, Align};
use druid::{Lens, Data};
use druid::im::Vector;
use std::cell::{RefCell, Cell};
use std::rc::Rc;
use std::ops::{Deref, Index, IndexMut};


pub fn build_ui() -> impl Widget<AppState> {
    let mut flex = Flex::column().with_flex_spacer(1.0);
    flex.add_flex_child(build_container(),1.0);

    flex
}


#[derive(Clone, Data, Lens)]
pub struct Cand {
    values: Vector<IndCand>,
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
    value: u8,
    status: CandStatus,

}

#[derive(Copy, Clone, PartialEq, Data)]
pub enum CandStatus {
    Active,
    Inactive,
}


impl CandStatus {
    pub fn color(&self) -> Color {
        match self {
            CandStatus::Active => Color::BLUE,
            CandStatus::Inactive => Color::BLACK,
        }
    }
}
impl Cand {
    pub fn new () -> Cand {
        let v: Vec<IndCand> = (1..=9).into_iter().map(|i|
            if i & 1 > 0 {
                IndCand::new(i, CandStatus::Active)
            } else {
                IndCand::new(i, CandStatus::Inactive)
            }).collect();
        Cand {
            values: v.into(),
        }
    }
}

pub struct CandWidget {
    label: Label<IndCand>
}

impl CandWidget {
    pub fn new () -> CandWidget {
        let label = Label::new(|data: &IndCand, _:&_| {
            data.value.to_string()
        });
        CandWidget {
            label
        }
    }
}

impl Widget<IndCand> for CandWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut IndCand, env: &Env) {
        self.label.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &IndCand, env: &Env) {
        self.label.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &IndCand, data: &IndCand, env: &Env) {
        self.label.update(ctx, old_data, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &IndCand, env: &Env) -> Size {
        self.label.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &IndCand, env: &Env) {
        // Background color
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &data.status.color());
        self.label.paint(ctx, data, env)
    }
}

impl IndCand {
    pub fn new(value: u8, status: CandStatus) -> IndCand {

        IndCand {
            value,
            status,
        }
    }
}

pub fn build_container () -> impl Widget<AppState> {
    let mut overall = Flex::column();
    for r in 0..3 {
        let mut row = Flex::row();

        for c in 0..3 {
            let index = r * 3 + c;

            let mut label = CandWidget::new().lens(AppState::cands.index(index as usize));
            row.add_flex_child(label, 1.0);

        }
        overall.add_flex_child(row, 1.0);
    }
    overall.border(Color::BLUE, 1.)
}