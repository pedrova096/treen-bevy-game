# Project Name

## Architecture

This project is structured as follows:

- `src/`: Contains the source code for the game.
  - `collision/`: Contains the collision detection system and components.
  - `game/`: Contains the game logic.
    - `player/`: Contains the player-related code.
    - `wagon.rs`: Contains the wagon-related code.
  - `menu/`: Contains the menu-related code.
  - `main.rs`: The entry point for the game.
- `systems/`: Contains additional systems used in the game.
- `.cargo/`: Contains the Cargo configuration file.
- `assets/`: Contains the game assets.

### State Machine

I'm implementing a state machine base of xstate, some of the based features are:

- Each state has a set of events that can be triggered.
- `on_enter` and `on_exit` functions are called when entering and exiting a state.
- Some states are transcendent, meaning that they can be entered from any other state (e.g. `Die`).

Things to Define:

- States:
  - States are defined as structs, and can have data associated with them.
  - State can be a enum, and each variant can have data associated with it.

The state machine needs to be able to handle the following events:

- Cold start: Cold start are timers that needs to be waited before we are able enter that state.
- Timeouts: Similar to cold start, but the timer is to wait to exit the state.
- Triggered timers: Timers that triggers a new state when they expire. These timers can be cancelled.

## Setup

1. Run `cargo build` to build the project.
2. Run `cargo run` to start the game.
3. For code reloading, install `cargo install cargo-watch` and run `cargo watch -x run`.

## Using Bevy

This project uses the [Bevy](https://bevyengine.org/) game engine. Bevy is a simple, ECS-based game engine written in Rust. It is still in early development, so expect some bugs and missing features.
