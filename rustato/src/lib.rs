pub use paste;
pub use ctor;

// Re-export everything from rustato-core
pub use rustato_core::*;

// Re-export macros from rustato-macros
pub use rustato_macros::{create_signal, GlobalState, get_state, on_state_change};

// Re-export auto_state from rustato-proc-macros
pub use rustato_proc_macros::auto_state as AutoState;

pub trait Fields {
    fn fields_mut(&mut self) -> Vec<&mut dyn std::any::Any>;
}

// Define a helper function
#[doc(hidden)]
pub fn __rustato_core_helper<T>(f: impl FnOnce() -> T) -> T {
    f()
}

use std::sync::Once;

pub trait AutoState {}

pub fn __register_global_state_immediately<T: 'static + Send + Sync + AutoState>(
    name: &'static str,
    state_creator: impl Fn() -> GlobalState<T> + Send + Sync + 'static
) {
    static REGISTER: Once = Once::new();
    REGISTER.call_once(|| {
        register_global_state(name, state_creator());
    });
}