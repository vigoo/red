use crossterm::*;
use riker::actors::*;
use std::time::Duration;
use std::sync::{Mutex, Arc};

use crate::commands;
use crate::state::StateMessage;
use crate::commands::GlobalAction;
use std::collections::HashSet;

pub struct Input {
    state: ActorRef<StateMessage>,
    input: Arc<Mutex<AsyncReader>>,
    quit_queries: Vec<BasicActorRef>,
}

#[derive(Clone, Debug)]
pub enum InputMsg {
    AskForQuit,
    Read
}

impl Input {
    fn actor((state, input): (ActorRef<StateMessage>, Arc<Mutex<AsyncReader>>)) -> Input {
        Input {
            state: state,
            input: input,
            quit_queries: Vec::new()
        }
    }

    pub fn props(state: ActorRef<StateMessage>, input: Arc<Mutex<AsyncReader>>) -> BoxActorProd<Input> {
        Props::new_args(Input::actor, (state, input))
    }
}

impl Actor for Input {
    type Msg = InputMsg;

    fn pre_start(&mut self, ctx: &Context<InputMsg>) {
        ctx.schedule(
            Duration::from_millis(100),
            Duration::from_millis(100),
            ctx.myself(),
            None,
            InputMsg::Read
        );
    }

    fn recv(&mut self, ctx: &Context<InputMsg>, msg: InputMsg, sender: Sender) {
        match msg {
            InputMsg::AskForQuit => {
                match sender {
                    Some(actor) =>
                        self.quit_queries.push(actor.clone()),
                    None => {}
                }
            },
            InputMsg::Read => {
                let mut input = self.input.lock().unwrap();

                while let Some(event) = input.next() {
                    match event {
                        InputEvent::Keyboard(key_event) =>
                            match commands::handle_key_event(key_event, &self.state) {
                                Some(GlobalAction::Quit) => {
                                    for actor in &self.quit_queries {
                                        actor.try_tell(true, None);
                                    }
                                    ctx.stop(ctx.myself());
                                },
                                None => {}
                            }
                        _ => {}
                    }
                }
            }
        }
    }
}