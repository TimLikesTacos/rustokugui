use crate::data::{AppState, CandidateInfo, Square, Status};
use crate::selectors::*;
use druid::widget::Controller;
use druid::{Command, Env, Event, EventCtx, LifeCycle, LifeCycleCtx, MouseButton, Notification, Selector, Target, UpdateCtx, Widget, WidgetId, KbKey};
use crate::view::{clear_hint, setval_button};

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

        if !ctx.has_focus() {
            if ctx.is_hot() {
                ctx.request_focus();
            }
        }

        match event {

            Event::KeyDown(keyevent) if keyevent.key == KbKey::Control =>  {
                data.multi_select = true;
            },

            Event::KeyUp(keyevent) if keyevent.key == KbKey::Control => {
                data.multi_select = false;
            },

            Event::KeyDown(key) => {
                dbg!("key");
            },

            Event::Notification(notice) => {
                if let Some(indexvalue) = notice.get(CAND_SELECTED) {
                    // Clear the hint if active
                    if data.active_hint.is_some() {
                        clear_hint(ctx, data, env);
                    }

                    if data.multi_select {
                        // if already in the vector, deselect and remove from selected set

                        // insert returns true if value was not in the set, false if it was
                        if !data.selected_pairs.insert(indexvalue.clone()) {
                            data.selected_pairs.remove(indexvalue);
                        }
                    }
                    // Double- click. Sets the value. Same thing as clicking the set value button
                    else if data.selected_pairs.len() == 1 && data.selected_pairs.contains(indexvalue) {
                        setval_button(ctx, data, env);
                    } else {
                        data.selected_pairs.clear();
                        data.selected_pairs.insert(indexvalue.clone());
                    }
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
        if data.selected_pairs != old_data.selected_pairs {
            // Clicked on inactive cand or box will deactive any selected candidate

            let selected: Vec<_> = data.selected_pairs.difference(&old_data.selected_pairs).collect();
            for select in selected {
                ctx.submit_command(Command::new(
                    CAND_SELECT,
                    (),
                    Target::Widget(select.id),
                ));
            }

            let unselected: Vec<_> = old_data.selected_pairs.difference(&data.selected_pairs).collect();
            for unselect in unselected {
                ctx.submit_command(Command::new(
                    CAND_DESELECT,
                    (),
                    Target::Widget(unselect.id),
                ));
            }
            // if data.selected_pairs.is_some() {
            //     ctx.submit_command(Command::new(
            //         CAND_SELECT,
            //         (),
            //         Target::Widget(data.selected_id),
            //     ));
            // }
            //
            //
            // // Activate first, deactivate second. This allows deactivation if the selected cand is clicked.
            // if old_data.selected_pairs.is_some() {
            //     ctx.submit_command(Command::new(
            //         CAND_DESELECT,
            //         (),
            //         Target::Widget(old_data.selected_id),
            //     ));
            // }

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
