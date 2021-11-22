
#![windows_subsystem = "windows"]

use druid::{theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt, WindowDesc, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, Size, PaintCtx, TextLayout, FontDescriptor, FontFamily, TextAlignment};
use rustoku::Sudoku;

use druid::widget::{CrossAxisAlignment, Flex, Label, Painter, Align, Container, Either};
use std::rc::Rc;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::BorrowMut;
use rustoku::square::Square;
use druid::im::Vector;
use druid::LensExt;
use std::convert::TryInto;


#[derive(Clone, Data, Lens)]
struct AppState {
    pub values: Values<String>,
    pub possibilities: Values<Vector<u8>>,
    #[data(ignore)]
    pub sud: Rc<Sudoku>,


}

#[derive(Clone)]
struct Values <T>{
    pub values: Vec<T>,
}

impl <T> Index<usize> for Values<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl <T> IndexMut<usize> for Values<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}


impl <T: Data> Data for Values<T>{
    fn same(&self, other: &Self) -> bool {
        self.values.iter().zip(other.values.iter()).all(|(a, b)| a.same(b))
    }
}




struct SquareGui {
    index: usize,
    poss: Vec<u8>,
    display: Flex<AppState>
}


impl SquareGui {
    pub fn new(index: usize) -> Self {
        SquareGui {
            index,
            poss: vec![],
            display: Self::build_candidates(index),
        }
    }

    fn build_candidates (index: usize)-> Flex<AppState>{
        let mut overall = Flex::column();
        for r in 0..3 {
            let mut row = Flex::row();

            for c in 0..3 {
                let num = r * 3 + c + 1;
                row.add_flex_child(Align::centered(Label::dynamic(move |data: &AppState, _: &_| {
                    if data.possibilities.values[index].contains(&(num as u8)) {
                        num.to_string()
                    } else {
                        " ".to_string()
                    }
                }).with_text_size(10.)).border(Color::BLUE,0.7), 1.0);
            }

            overall.add_flex_child(row, 1.0);
        }

        overall

    }
}

impl Widget<AppState> for SquareGui {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        self.display.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        self.display.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        self.display.update(ctx, old_data , data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size {
        self.display.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.display.paint(ctx, data, env)
    }
}


fn build_square(row: usize, col: usize, box_width: usize) -> impl Widget<AppState> {

    let index = row * 9 + col;

    let either = Either::new(
        move |data, _env| data.sud.get((row, col)).unwrap() > 0,
        Label::raw().with_text_size(18.).lens(AppState::values.index(index)),
        SquareGui::new(index).border(Color::RED, 1.0),
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
            if c % box_width == 0 && c != 0 {
                row.add_flex_spacer(SPACER);
            }

            row.add_flex_child(build_square(r,c, box_width), 1.0);
        }

        if r % box_width == 0 && r != 0 {
            column.add_flex_spacer(SPACER);
        }
        column.add_flex_child(row, 1.0);
    }
    column
}

pub fn main() {
    let window = WindowDesc::new(||build_grid(3))
        .window_size((500., 500.))
        .resizable(true)
        .title(
            LocalizedString::new("Rustoku").with_placeholder("Rustoku - Sudoku in Rust"),
        );
    let sudoku = Sudoku::new(".5267.3.8.3...562767..325.128...61.5.6....2.4714523869827314956.9.267483346958712").unwrap();

    let values = sudoku.value_iter().map(|v| v.to_string()).collect::<Vec<String>>();
    let poss: Vec<Vector<u8>> = sudoku.possibilities_iter().map(|v| v.into()).collect();


    let values: Values<String> =  Values {
        values,
    };

    let poss: Values<Vector<u8>> = Values {
        values: poss,
    };

    let sudoku = AppState {
        sud: Rc::new(sudoku),
        values,
        possibilities: poss,
    };
    AppLauncher::with_window(window)
        .launch(sudoku)
        .expect("launch failed");
}


// fn main() -> Result<(), PlatformError> {
//     AppLauncher::with_window(WindowDesc::new(||build_ui())).launch(())?;
//     Ok(())
// }