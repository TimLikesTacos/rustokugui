use druid::widget::{Align, Label};
use crate::data::{IndCand, Status, CandidateInfo};
use druid::{Color, Widget, EventCtx, Env, Event, MouseButton, Command, Target, LifeCycle, LifeCycleCtx, UpdateCtx, LayoutCtx, BoxConstraints, Size, PaintCtx, RenderContext, WidgetId};
use crate::selectors::{CAND_SELECTED, CAND_SELECT, CAND_DESELECT};
use std::error::Error;

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
                        let indexvalue = CandidateInfo::new(data.square_index, data.value, ctx.widget_id());
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

                if let Some(_) = command.get(CAND_SELECT) {
                    data.status = Status::Selected;
                }
                if let Some(_) = command.get(CAND_DESELECT) {
                    if data.status == Status::Selected {
                        data.status = Status::Active;
                    }
                }

                ctx.request_update();
            }

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

