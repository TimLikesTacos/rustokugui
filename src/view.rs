use druid::{
    widget::Label, BoxConstraints, Color, Command, Env, Event, EventCtx, LayoutCtx, LensExt,
    LifeCycle, LifeCycleCtx, MouseButton, PaintCtx, RenderContext, Selector, Size, Target,
    UpdateCtx, Widget, WidgetExt,
};

use crate::controller::GridController;
use crate::data::*;
use crate::selectors::{CAND_DESELECT, CAND_SELECT, CAND_SELECTED, CAND_VALUE};
use druid::im::Vector;
use druid::widget::{Align, Button, Container, CrossAxisAlignment, Either, Flex, FlexParams, List};
use druid::{Data, Lens};
use rustoku::{ChangeType, Move};
use std::cell::{Cell, RefCell};
use std::ops::{Deref, Index, IndexMut};
use std::rc::Rc;

pub fn build_ui() -> impl Widget<AppState> {
    let mut flex = Flex::column();
    flex.add_flex_child(build_grid(3), 1.0);

    flex
}

pub struct CandWidget {
    label: Align<IndCand>,
}

impl CandWidget {
    pub fn new() -> CandWidget {
        let label = Align::centered(
            Label::new(|data: &IndCand, _: &_| {
                if data.status == Status::Inactive {
                    " ".to_string()
                } else {
                    data.value.to_string()
                }
            })
            .with_text_size(11.)
            .with_text_color(Color::WHITE),
        );
        CandWidget { label }
    }
}

impl Widget<IndCand> for CandWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut IndCand, env: &Env) {
        match event {
            Event::MouseDown(m) => match m.button {
                MouseButton::Left => {
                    if data.status != Status::Inactive {
                        let indexvalue = IndexValuePair::new(data.square_index, data.value);
                        ctx.submit_notification(Command::new(
                            CAND_SELECTED,
                            indexvalue,
                            Target::Auto,
                        ));
                    }
                }
                _ => (),
            },

            Event::Command(command) => {
                dbg!(&data.status, &data.value);
                if let Some(payload) = command.get(CAND_SELECT) {
                    data.status = Status::Selected;
                }
                if let Some(payload) = command.get(CAND_DESELECT) {
                    if data.status != Status::Inactive {
                        data.status = Status::Active;
                    }
                }
                dbg!(&data.status);
                ctx.request_update();
            }
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

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &IndCand,
        env: &Env,
    ) -> Size {
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
    pub fn new(value: u8, status: Status, square_index: usize) -> IndCand {
        IndCand {
            value,
            status,
            square_index,
        }
    }
}

pub fn build_container(index: usize) -> impl Widget<Square> {
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

            row.add_flex_child(
                build_square(r, c, box_width).lens(AppState::squares.index(index)),
                1.0,
            );
        }

        if r % box_width == 0 && r != 0 {
            column.add_flex_spacer(SPACER);
        }
        column.add_flex_child(row, 1.0);
    }

    // Buttons
    let mut row = Flex::row();
    let setval = Button::new("Set Value").on_click(setval_button);
    let remove_pot = Button::new("Remove Candidate").on_click(remove_pot_button);
    let undo = Button::new("Undo").on_click(undo_last);

    let param = FlexParams::new(1.0, CrossAxisAlignment::Center);
    row.add_flex_child(Align::centered(setval), param);
    row.add_flex_child(Align::centered(remove_pot), param);
    row.add_flex_child(Align::centered(undo), param);
    column.add_flex_child(row, 1.0);

    let mut hint_row = Flex::row();
    let hint_button = Align::centered(
        Button::dynamic(|data: &AppState, _: &_| {
            if data.active_hint.is_some() {
                "Apply".to_string()
            } else {
                "Hint".to_string()
            }
        })
        .on_click(hint_button),
    );
    let hint_title = Align::centered(Label::dynamic(|data: &AppState, _: &_| {
        data.hint_name.clone()
    }));

    let clear_hint = Align::centered(Button::new("Clear Hint").on_click(clear_hint));
    hint_row.add_flex_child(hint_button, 1.0);
    hint_row.add_flex_child(clear_hint, 1.0);
    hint_row.add_flex_child(hint_title, 1.0);

    column.add_flex_child(hint_row, 1.0);
    column.controller(GridController)
}

fn setval_button(ctx: &mut EventCtx, data: &mut AppState, env: &Env) {
    let themove = data
        .sud
        .set(data.selectedPair.index, data.selectedPair.value)
        .unwrap();
    update_from_move(ctx, data, env, themove);
}

fn remove_pot_button(ctx: &mut EventCtx, data: &mut AppState, env: &Env) {
    let themove = data
        .sud
        .remove_potential(data.selectedPair.index, data.selectedPair.value)
        .unwrap();
    update_from_move(ctx, data, env, themove);
}

fn undo_last(ctx: &mut EventCtx, data: &mut AppState, env: &Env) {
    if data.active_hint.is_some() {
        clear_hint(ctx, data, env);
    }
    if let Some(last_move) = data.sud.undo() {
        let changes = last_move.changes_vec();
        for change in changes {
            match change {
                ChangeType::SetValue(v) => {
                    data.squares[v.index()].value = "".to_string();
                }
                ChangeType::RemovedPot(v) => {
                    let values: Vec<usize> = v.values();
                    for value in values {
                        data.squares[v.index()].cands[value - 1].status = Status::Active;
                    }
                }
            }
        }
        ctx.request_update();
    }
}

fn update_from_move(ctx: &mut EventCtx, data: &mut AppState, _env: &Env, themove: Move) {
    let changes = themove.changes_vec();
    for change in changes {
        match change {
            ChangeType::SetValue(v) => {
                data.squares[v.index()].value = v.values::<u8, Vec<u8>>()[0].to_string();
            }
            ChangeType::RemovedPot(v) => {
                let values: Vec<usize> = v.values();
                for value in values {
                    data.squares[v.index()].cands[value - 1].status = Status::Inactive;
                }
            }
        }
    }
    ctx.request_update();
}

fn hint_button (ctx: &mut EventCtx, data: &mut AppState, env: &Env) {
    if data.active_hint.is_some() {
        apply_hint(ctx, data, env)
    } else {
        get_hint(ctx, data, env)
    }
}

fn get_hint(ctx: &mut EventCtx, data: &mut AppState, env: &Env) {
    if let Some(themove) = data.solver.next(&data.sud) {

        // show the involved candidates
        let involved = themove.involved_vec();
        for pair in involved {
            let values: Vec<usize> = pair.values();
            for value in values {
                data.squares[pair.index()].cands[value - 1].status = Status::Involved;
            }
        }

        // show the candiates that can be set or removed
        let changes = themove.changes_vec();
        for change in changes {
            let pair = match change {
                ChangeType::SetValue(p) => p,
                ChangeType::RemovedPot(p) => p,
            };

            let values: Vec<usize> = pair.values();
            for value in values {
                data.squares[pair.index()].cands[value - 1].status = Status::Removable;
            }
        }

        // Set active

        data.hint_name = themove.technique().to_string();
        data.active_hint = Some(themove);

        ctx.request_update();
    }
}

fn apply_hint (ctx: &mut EventCtx, data: &mut AppState, env: &Env) {

    if let Some(themove) = data.active_hint.clone() {
        let themove = themove.apply(&mut data.sud);
        dbg!(&themove);
        clear_hint(ctx, data, env);
        update_from_move(ctx, data, env, themove)
    }

}

fn clear_hint(ctx: &mut EventCtx, data: &mut AppState, env: &Env) {
    data.active_hint = None;
    data.hint_name = "".to_owned();

    for mut square in data.squares.iter_mut() {
        for mut cand in square.cands.iter_mut() {
            if cand.status == Status::Involved || cand.status == Status::Removable {
                cand.status = Status::Active;
            }
        }
    }
}
