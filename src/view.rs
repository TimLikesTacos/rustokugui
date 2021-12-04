use crate::candidate::CandWidget;
use crate::controller::GridController;
use crate::data::*;
use crate::selectors::*;
use druid::im::Vector;
use druid::widget::{
    Align, Button, Container, CrossAxisAlignment, Either, Flex, FlexParams, Label, LabelText, List,
    MainAxisAlignment, TextBox,
};
use druid::{
    Color, Command, Data, Env, EventCtx, Lens, LensExt, Target, UnitPoint, Widget, WidgetExt,
};
use rustoku::{ChangeType, Move, SudError, Sudoku};

const BOX_WIDTH: usize = 3;
const GRID_WIDTH: usize = BOX_WIDTH * BOX_WIDTH;
const SPACER: f64 = 0.15;

pub fn build_grid() -> impl Widget<AppState> {
    // Title
    let display = Label::new("rustoku")
        .with_text_size(32.0)
        //.lens(CalcState::value)
        .padding(1.);

    let mut column = Flex::column().with_flex_spacer(0.1).with_child(display);

    // Make the squares.  Add spacing to form the 3x3 boxes.
    for r in 0..GRID_WIDTH {
        let mut row = Flex::row();

        for c in 0..GRID_WIDTH {
            let index = r * GRID_WIDTH + c;
            // Spacing to form boxes
            if c % BOX_WIDTH == 0 && c != 0 {
                row.add_flex_spacer(SPACER);
            }

            row.add_flex_child(build_square().lens(AppState::squares.index(index)), 1.0);
        }

        if r % BOX_WIDTH == 0 && r != 0 {
            column.add_flex_spacer(SPACER);
        }
        column.add_flex_child(row.controller(GridController), 1.0);
    }

    // Control Buttons
    let mut button_row = Flex::row();
    let setval = Button::new("Set Value").on_click(setval_button);
    let remove_pot = Button::new("Remove Candidate").on_click(remove_pot_button);
    let undo = Button::new("Undo").on_click(undo_last);

    button_row.add_flex_child(Align::centered(setval), 1.0);
    button_row.add_flex_child(Align::centered(remove_pot), 1.0);
    button_row.add_flex_child(Align::centered(undo), 1.0);

    column.add_flex_child(button_row, 0.5);

    //Hint buttons
    let mut hint_row = Flex::row();
    let hint_button = Align::centered(
        Button::dynamic(|data: &AppState, _: &_| {
            if data.active_hint.is_some() {
                "Apply Hint".to_string()
            } else {
                "Hint".to_string()
            }
        })
        .on_click(hint_button),
    );
    let hint_title = Align::centered(Label::dynamic(|data: &AppState, _: &_| {
        data.hint_name.to_string()
    }));

    let clear_hint = Align::centered(Button::new("Clear Hint").on_click(clear_hint));

    hint_row.add_flex_child(hint_button, 1.0);
    hint_row.add_flex_child(clear_hint, 1.0);
    hint_row.add_flex_child(hint_title, 1.0);

    column.add_flex_child(hint_row, 0.5);

    column.add_flex_child(build_manual_section(), 1.0);

    column.add_flex_child(
        Flex::row().with_flex_child(
            Label::raw()
                .with_text_size(12.)
                .with_text_color(Color::RED)
                .lens(AppState::error_msg),
            1.0,
        ),
        1.0,
    );

    column
}

fn build_square() -> impl Widget<Square> {
    // If square has a value, set it to just a label.  If not, build square with candidates.
    let either = Either::new(
        |data: &Square, _env| data.value.len() > 0,
        Label::raw().with_text_size(18.).lens(Square::value),
        build_container(),
    );

    Align::centered(either).border(Color::grey(0.50), 1.0)
}

pub fn build_container() -> impl Widget<Square> {
    let mut overall = Flex::column();
    for r in 0..BOX_WIDTH {
        let mut row = Flex::row();

        for c in 0..BOX_WIDTH {
            let index = r * BOX_WIDTH + c;
            let mut label = CandWidget::new().lens(Square::cands.index(index as usize));
            row.add_flex_child(label, 1.0);
        }

        overall.add_flex_child(row, 1.0);
    }

    overall
}

fn build_manual_section() -> impl Widget<AppState> {
    let mut container = Flex::row();
    let mut col = Flex::column();
    let label = Align::vertical(
        UnitPoint::LEFT,
        Label::new("Enter a valid puzzle string: ").with_text_size(11.),
    );
    let submit = Align::centered(Button::new("Import Puzzle").on_click(validate_import));
    let manual = Flex::row().with_child(label).with_child(submit);
    let mut textbox = Align::vertical(
        UnitPoint::LEFT,
        TextBox::new()
            .with_text_size(14.0)
            .with_placeholder("Enter your puzzle string here".to_string())
            .expand_width()
            .lens(AppState::original_string),
    );
    col.add_flex_child(manual, 0.8);
    col.add_flex_child(textbox, 0.8);

    container.add_flex_child(col, 1.0);
    container
}

fn validate_import(ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
    if data.original_string.len() != 81 {
        data.error_msg = format!(
            "There are only {} characters, not 81",
            data.original_string.len()
        )
            .into();
        ctx.set_handled()
    } else {
        match Sudoku::new(data.original_string.as_str()) {
            Ok(sud) => match sud.validate() {
                Ok(_) => *data = AppState::new(&data.original_string),
                Err(e) => data.error_msg = e.to_string().into(),
            },
            Err(e) => data.error_msg = e.to_string().into(),
        }
    }
}

pub fn setval_button(ctx: &mut EventCtx, data: &mut AppState, env: &Env) {
    // Do not allow any setting if more than one or zero candidates
    if data.selected_pairs.len() == 1 {
        let selected = data.selected_pairs.iter().next().unwrap();
        let themove = data.sud.set(selected.index, selected.value).unwrap();

        data.selected_pairs.clear();
        update_from_move(ctx, data, env, themove);
    }
}

pub fn remove_pot_button(ctx: &mut EventCtx, data: &mut AppState, env: &Env) {
    let mut moves = vec![];
    for candidate in data.selected_pairs.iter() {
        let themove = data
            .sud
            .remove_potential(candidate.index, candidate.value)
            .unwrap();
        moves.push(themove);
    }
    for amove in moves {
        update_from_move(ctx, data, env, amove);
    }
    data.selected_pairs.clear();
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

fn hint_button(ctx: &mut EventCtx, data: &mut AppState, env: &Env) {
    if data.active_hint.is_some() {
        apply_hint(ctx, data, env)
    } else {
        get_hint(ctx, data, env)
    }
}

fn get_hint(ctx: &mut EventCtx, data: &mut AppState, env: &Env) {
    if let Some(themove) = data.solver.next(&data.sud) {
        clear_selected_candidate(ctx, data, env);

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

        data.hint_name = themove.technique().to_string().into();
        data.active_hint = Some(themove);

        ctx.request_update();
    }
}

fn apply_hint(ctx: &mut EventCtx, data: &mut AppState, env: &Env) {
    if let Some(themove) = data.active_hint.clone() {
        let themove = themove.apply(&mut data.sud);
        clear_hint(ctx, data, env);
        update_from_move(ctx, data, env, themove)
    }
}

pub fn clear_hint(_ctx: &mut EventCtx, data: &mut AppState, env: &Env) {
    data.active_hint = None;
    data.hint_name = "".to_string().into();

    for square in data.squares.iter_mut() {
        for mut cand in square.cands.iter_mut() {
            if cand.status == Status::Involved || cand.status == Status::Removable {
                cand.status = Status::Active;
            }
        }
    }
}

pub fn clear_selected_candidate(ctx: &mut EventCtx, data: &mut AppState, env: &Env) {
    for selected in data.selected_pairs.iter() {
        ctx.submit_command(Command::new(CAND_DESELECT, (), Target::Widget(selected.id)));
    }
    data.selected_pairs.clear();
}
