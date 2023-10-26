# Aurora Flight Computer Platform

[![codecov](https://codecov.io/gh/aris-space/aurora-core/branch/main/graph/badge.svg?token=GML428JLHJ)](https://codecov.io/gh/aris-space/aurora-core)
[![Dependency Status](https://deps.rs/repo/github/aris-space/aurora-core/status.svg)](https://deps.rs/repo/github/aris-space/aurora-core)

The Aurora Flight Computer Platform is a Rust framework for building reliable, automatic systems, particularly for use 
in amateur rocketry flight computers.

Aurora consists of several interdependent Rust crates that together provide the user with the tools to build a flight 
computer for a particular physical system. The following crates are currently considered stable and ready for use:
- `aurora_fsm`: This crate provides helper functionality around creating and using event-driven finite state machines; 
  in particular it provides the `StateMachine<E>` struct and the `State<E>` trait (both generic over some event type `E`)
  which provide the building blocks for a state machine. Further development of the FSM system is happening on the `feature/fsm` branch.
- `event_gen`: This crate provides several "Event Generators", i.e. systems designed to generate state machine events 
  that drive the execution of the state machine forward. Further development of the event generators is happening on the
  `feature/event_gen` branch.

In addition to these crates, there are a few crates that are considered semi-stable or in development, namely:
- `aurora_hal`: This crate is currently in development on the `feature/hal` branch and provides Aurora's 
  Hardware Abstraction Layer (HAL); consisting of helpers to construct an I/O Tree and related types. It is considered 
  semi-stable and replaces the old `aurora_hal` crate currently present on the `main` branch.
- `aurora_controls`: This crate is in early development and aims to provide a system of building blocks for control systems,
  to make the implementation of complex control system more straightforward for people not experienced in Rust but 
  familiar with systems such as MathWorks Simulink. It is being developed on the `feature/controls` branch.

## Overview of branches
- `main`: Stable framework base (Note 2023-10-26: Use `feature/hal` as the base for new binaries using the Aurora FCP to use the new HAL.)
- `admin/ci`: Feature branch for CI changes
- `feature/fsm`: Development of the Finite State Machine
- `feature/hal`: Development of the Hardware Abstraction Layer, considered semi-stable
- `feature/event_gen`: Development of the event generator system
- `feature/controls`: Development of the new Aurora Controls Framework, early development state
- `flight/aurora-i`: Example use of the Aurora Flight Computer Platform, including the new HAL.

## How to use the Aurora FCP for a new rocket
1. Create a new `flight/` branch (e.g. `flight/<your-rocket-name>`), based on `feature/hal` (Usually you would use `main` 
   as the base but until the stabilization of HAL 2.0, you should probably use `feature/hal`.)
2. Create a new binary crate called `fc` in the Aurora Cargo workspace and add it to the workspace-level `Cargo.toml`.
3. Add your Aurora dependencies to the binary's `Cargo.toml`:
```toml
[dependencies]
aurora_fsm = {path = "../aurora_fsm"}
event_gen = {path = "../event_gen"}
aurora_hal = {path = "../aurora_hal"}
```
4. Create a `StateMachine` and your I/O tree in your `main.rs` and loop the FSM's `step()` function to drive it forward:
```rust
#![feature(once_cell)]

use aurora_fsm::state_machine::StateMachine;
use aurora_hal::init_io_tree;

init_io_tree! {
    /* Struct body of your I/O tree */
}

fn main() {
    // Initialize your hardware drivers here
    
    let mut state_machine = StateMachine::new(/* Pass your initial state here */);
    
    loop {
        state_machine.step();
    }
}

```
5. Create types that implement the `State<E>` trait and model your FSM behavior.

Check out the `flight/aurora-i` branch and the documentation for the various Aurora crates for more details!