pub use once_cell;
mod state_manager;
mod state;

pub use state_manager::{State, StateManager, GLOBAL_STATE_MANAGER, StateChangeCallback};
pub use state::*;