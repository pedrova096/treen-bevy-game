use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum PlayerState {
    Idle,
    Moving,
    Falling,
    Pushing,
    Invalid,
}

#[derive(Debug)]
pub enum PlayerEvent {
    Move,
    Jump,
    Push,
    Stop,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Idle
    }
}

impl PlayerState {
    pub fn transition(&mut self, event: PlayerEvent) -> bool {
        let (new_state, change) = match (&self, event) {
            (PlayerState::Idle, PlayerEvent::Move) => (PlayerState::Moving, true),
            (PlayerState::Moving, PlayerEvent::Move) => (PlayerState::Moving, true),
            (_, PlayerEvent::Stop) => (PlayerState::Idle, true),
            _ => (PlayerState::Invalid, false),
        };
        *self = new_state;
        change
    }
}
