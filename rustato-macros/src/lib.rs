#[macro_export]
macro_rules! get_state {
    ($type:ty) => {{
        use rustato_core::{once_cell, State, GLOBAL_STATE_MANAGER};

        static STATE: once_cell::sync::Lazy<State<$type>> = once_cell::sync::Lazy::new(|| {
            GLOBAL_STATE_MANAGER
                .get_state::<$type>(stringify!($type))
                .unwrap_or_else(|| panic!("Estado '{}' não encontrado. Certifique-se de que create_state!() foi chamado para este tipo.", stringify!($type)))
        });
        &*STATE
    }};
}

#[macro_export]
macro_rules! on_state_change {
    ($type:ty, $callback:expr) => {{
        use rustato_core::{GLOBAL_STATE_MANAGER, StateChangeCallback};
        let callback: StateChangeCallback<$type> = Box::new($callback);
        GLOBAL_STATE_MANAGER.register_callback::<$type>(stringify!($type), callback);
    }};
}