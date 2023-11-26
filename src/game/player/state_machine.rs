use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum PlayerState {
    Idle,
    Moving,
    Jumping,
    Pulling,
    Pushing,
    Invalid,
}

#[derive(Debug)]
pub enum PlayerEvent {
    Move,
    Jump,
    Pull,
    Push,
    Stop,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Idle
    }
}

impl PlayerState {
    pub fn is(&self, state: PlayerState) -> bool {
        *self == state
    }
    fn transitions(&self, event: &PlayerEvent) -> PlayerState {
        match (&self, event) {
            (PlayerState::Idle, PlayerEvent::Move) => PlayerState::Moving,
            (PlayerState::Idle, PlayerEvent::Push) => PlayerState::Pushing,
            (PlayerState::Idle, PlayerEvent::Pull) => PlayerState::Pulling,
            (PlayerState::Pushing, PlayerEvent::Move) => PlayerState::Moving,
            (PlayerState::Pushing, PlayerEvent::Pull) => PlayerState::Pulling, // TODO: slow down
            (PlayerState::Moving, PlayerEvent::Move) => PlayerState::Moving,
            (PlayerState::Moving, PlayerEvent::Push) => PlayerState::Pushing,
            (PlayerState::Moving, PlayerEvent::Stop) => PlayerState::Idle,
            (PlayerState::Moving, PlayerEvent::Jump) => PlayerState::Jumping,
            _ => PlayerState::Invalid,
        }
    }
    pub fn can(&self, event: PlayerEvent) -> bool {
        self.transitions(&event) != PlayerState::Invalid
    }
    pub fn transition(&mut self, event: PlayerEvent) -> bool {
        let new_state = self.transitions(&event);
        let change = new_state != PlayerState::Invalid;

        if !change {
            print!("Invalid transition from {:?} with {:?}\n", self, event)
        } else {
            print!("Transition from {:?} to {:?}\n", self, new_state)
        }

        *self = new_state;

        change
    }
}
