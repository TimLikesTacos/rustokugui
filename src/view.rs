use druid::{widget::Label, Widget, WidgetExt, Env, Color, LensExt, EventCtx, Event, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, Size, PaintCtx, RenderContext, MouseButton};

use crate::data::*;
use druid::widget::{Flex, Button, List, Container, Align, Either};
use druid::{Lens, Data};
use druid::im::Vector;
use std::cell::{RefCell, Cell};
use std::rc::Rc;
use std::ops::{Deref, Index, IndexMut};


pub fn build_ui() -> impl Widget<AppState> {
    let mut flex = Flex::column();
    flex.add_flex_child(build_grid(3),1.0);

    flex
}




pub struct CandWidget {
    label: Align<IndCand>
}

impl CandWidget {
    pub fn new () -> CandWidget {
        let label = Align::centered(Label::new(|data: &IndCand, _:&_| {
            if data.status == Status::Inactive {
                " ".to_string()
            }
            else {
                data.value.to_string()
            }
        })
            .with_text_size(11.)
            .with_text_color(Color::WHITE));
        CandWidget {
            label
        }
    }
}

impl Widget<IndCand> for CandWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut IndCand, env: &Env) {
        match event {

            Event::MouseDown(m) => {
                match m.button {
                    MouseButton::Left => data.status = Status::Selected,
                    MouseButton::Right => data.status = Status::Active,
                    _ => (),
                }
            }
            // Event::MouseUp(_) => {
            //     dbg!(data.value, &data.status);
            // }

            Event::Command(_) => {}
            Event::Notification(_) => {}
            _ => {}

        }

        self.label.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &IndCand, env: &Env) {
        self.label.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &IndCand, data: &IndCand, env: &Env) {
        if old_data.status != data.status {
            ctx.request_paint();
        }
        self.label.update(ctx, old_data, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &IndCand, env: &Env) -> Size {
        self.label.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &IndCand, env: &Env) {
        // Background color
        dbg!("painting");
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &data.status.color());
        self.label.paint(ctx, data, env)
    }
}

impl IndCand {
    pub fn new(value: u8, status: Status) -> IndCand {

        IndCand {
            value,
            status,
        }
    }
}

pub fn build_container (index: usize) -> impl Widget<Square> {
    let mut overall = Flex::column();
    for r in 0..3 {
        let mut row = Flex::row();

        for c in 0..3 {
            let index = r * 3 + c;

            let mut label = CandWidget::new().lens(Square::cands.index(index as usize));
            row.add_flex_child(label, 1.0);

        }
        overall.add_flex_child(row, 1.0);
    }
    overall.border(Color::BLUE, 1.)
}


fn build_square(row: usize, col: usize, box_width: usize) -> impl Widget<Square> {

    let index = row * 9 + col;

    let either = Either::new(
        |data: &Square, _env| data.value.len() > 0,
        Label::raw().with_text_size(18.).lens(Square::value),
        build_container(index),
    );

    Align::centered(either).border(Color::grey(0.50), 1.0)
}

fn build_grid(box_width: usize) -> impl Widget<AppState> {
    let SPACER: f64 = 0.15;
    let width = box_width * box_width;
    let display = Label::new("rustoku")
        .with_text_size(32.0)
        //.lens(CalcState::value)
        .padding(5.0);

    let mut column = Flex::column().with_flex_spacer(SPACER).with_child(display);

    for r in 0..width {

        let mut row = Flex::row();

        for c in 0..width {
            let index = r * width + c;
            if c % box_width == 0 && c != 0 {
                row.add_flex_spacer(SPACER);
            }

            row.add_flex_child(build_square(r,c, box_width).lens(AppState::squares.index(index)), 1.0);
        }

        if r % box_width == 0 && r != 0 {
            column.add_flex_spacer(SPACER);
        }
        column.add_flex_child(row, 1.0);
    }
    column
}