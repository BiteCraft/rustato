pub use rustato_core::*;

#[macro_export]
macro_rules! create_signal {
    ($initial:expr) => {{
        $crate::__rustato_core_helper(|| $crate::Signal::new($initial, String::from("unnamed")))
    }};
    ($initial:expr, $field_name:expr) => {{
        $crate::__rustato_core_helper(|| $crate::Signal::new($initial, String::from($field_name)))
    }};
}

#[macro_export]
macro_rules! GlobalState {
    ($name:ident) => {
        $crate::__rustato_core_helper(|| {
            $crate::__register_global_state_immediately(
                stringify!($name),
                || $crate::GlobalState::new(<$name>::default())
            )
        })
    };
}

#[macro_export]
macro_rules! get_state {
    ($name:ident) => {{
        $crate::__rustato_core_helper(|| $crate::get_global_state::<$name>(stringify!($name)).unwrap())
    }};
}

#[macro_export]
macro_rules! on_state_change {
    ($state:ident, $callback:expr) => {{
        $crate::__rustato_core_helper(|| {
            let state = $crate::get_global_state::<$state>(stringify!($state))
                .expect("State not found");
            state.on_change($callback);
        })
    }};
}