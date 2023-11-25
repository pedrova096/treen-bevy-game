use bevy::prelude::*;

#[derive(Resource)]
pub struct LastCursorMoved(pub Vec2);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum States {
    Idle,
    Moving,
    Jumping,
    Falling,
    Pushing,
    Dead,
}

#[derive(Debug)]
enum Events {
    None,
    Move(Vec2),
    Jump(Vec2),
    Pushed,
    Fall,
    Die,
}

#[derive(Debug, Clone, Copy)]
pub enum Context {
    None,
    Movement(Vec2),
}

#[derive(Component, Debug)]
pub struct PlayerStateMachine {
    pub state: States,
    pub context: Context,
}

impl PlayerStateMachine {
    fn new() -> Self {
        PlayerStateMachine {
            state: States::Idle,
            context: Context::None,
        }
    }

    fn is(&self, state: States) -> bool {
        self.state == state
    }

    fn _transition(&mut self, event: Events) {
        // TODO: REVIEW
        let current_state = &self.state;

        (self.state, self.context) = match (current_state, event) {
            (States::Idle, Events::Move(direction)) => {
                (States::Moving, Context::Movement(direction))
            }
            (States::Idle, Events::Jump(direction)) => {
                (States::Jumping, Context::Movement(direction))
            }
            (States::Idle, Events::Pushed) => (States::Pushing, Context::None), // TODO: Push force
            (States::Idle, Events::Fall) => (States::Falling, Context::None),   // TODO!
            (States::Moving, Events::Move(direction)) => {
                (States::Moving, Context::Movement(direction))
            }
            (States::Moving, Events::Jump(direction)) => {
                (States::Jumping, Context::Movement(direction))
            }
            (States::Moving, Events::Fall) => (States::Falling, Context::None), // TODO!
            (States::Jumping, Events::Move(direction)) => {
                (States::Jumping, Context::Movement(direction))
            }
            (States::Jumping, Events::Fall) => (States::Falling, Context::None), // TODO!
            (States::Falling, Events::Move(direction)) => {
                (States::Falling, Context::Movement(direction)) // TODO: clamp speed
            }
            (States::Pushing, Events::Move(direction)) => {
                (States::Moving, Context::Movement(direction))
            }
            (States::Pushing, Events::Jump(direction)) => {
                (States::Jumping, Context::Movement(direction)) // TODO: clamp speed
            }
            (States::Pushing, Events::Fall) => (States::Falling, Context::None), // TODO!
            (_, Events::Die) => (States::Dead, Context::None),                   // TODO!
        };
    }
}
