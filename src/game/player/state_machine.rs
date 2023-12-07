use bevy::prelude::*;

use super::animation::*;

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
            (PlayerState::Idle, PlayerEvent::Jump) => PlayerState::Jumping,
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

impl AnimationState for PlayerState {
    fn get_animation(&self) -> AnimationIndices {
        match self {
            PlayerState::Idle => AnimationIndices::Straight(AnimationStraight {
                last: 3,
                ..default()
            }),
            PlayerState::Pushing => AnimationIndices::Straight(AnimationStraight {
                first: 8,
                last: 12,
                repeat_from: Some(11),
                rate: 0.2,
                ..default()
            }),
            PlayerState::Moving => AnimationIndices::Straight(AnimationStraight {
                first: 24,
                last: 31,
                rate: 0.1,
                cool_down: Some(0.2),
                ..default()
            }),
            _ => AnimationIndices::Straight(AnimationStraight::default()),
        }
    }
}
