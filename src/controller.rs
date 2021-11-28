use crate::data::{AppState, IndexValuePair, Square, Status};
use crate::selectors::*;
use druid::widget::Controller;
use druid::{
    Command, Env, Event, EventCtx, LifeCycle, LifeCycleCtx, MouseButton, Notification, Selector,
    Target, UpdateCtx, Widget, WidgetId,
};

pub struct GridController;

impl<W: Widget<AppState>> Controller<AppState, W> for GridController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Notification(notice) => {
                if let (Some(indexvalue)) = notice.get(CAND_SELECTED) {
                    data.selectedId = notice.source();
                    data.selectedPair = indexvalue.clone();
                }
                ctx.set_handled();
            }
            _ => (),
        }

        child.event(ctx, event, data, env)
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        // Update selected candidate
        if data.selectedId != old_data.selectedId {
            // Clicked on inactive cand or box will deactive any selected candidate

            ctx.submit_command(Command::new(
                CAND_SELECT,
                (),
                Target::Widget(data.selectedId),
            ));

            // Activate first, deactivate second. This allows deactivation if the selected cand is clicked.
            ctx.submit_command(Command::new(
                CAND_DESELECT,
                (),
                Target::Widget(old_data.selectedId),
            ));
        }
        child.update(ctx, old_data, data, env)
    }
}

// pub struct SquareController {}
//
// impl <W: Widget<Square>>Controller<Square, W> for SquareController{
//     fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut Square, env: &Env) {
//
//         match event {
//             _ => (),
//
//         }
//
//         child.event(ctx, event, data, env)
//     }
// }
