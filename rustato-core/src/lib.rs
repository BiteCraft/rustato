use std::sync::{Arc, RwLock};
use std::any::TypeId;

mod global_state;
mod signal;

pub use global_state::{GlobalState, register_global_state, get_global_state, unregister_global_state};
pub use signal::Signal;

#[doc(hidden)]
pub fn __rustato_core_helper<T>(f: impl FnOnce() -> T) -> T {
    f()
}

pub struct GlobalStateContainer {
    states: RwLock<std::collections::HashMap<TypeId, Box<dyn std::any::Any + Send + Sync>>>,
}

impl GlobalStateContainer {
    pub fn new() -> Self {
        GlobalStateContainer {
            states: RwLock::new(std::collections::HashMap::new()),
        }
    }

    pub fn get_or_create<T: Default + 'static + Send + Sync>(&self) -> Arc<RwLock<T>> {
        let type_id = TypeId::of::<T>();
        let mut states = self.states.write().unwrap();

        if !states.contains_key(&type_id) {
            let new_state = Arc::new(RwLock::new(T::default()));
            states.insert(type_id, Box::new(new_state.clone()));
            new_state
        } else {
            states.get(&type_id).unwrap().downcast_ref::<Arc<RwLock<T>>>().unwrap().clone()
        }
    }

    pub fn drop<T: 'static>(&self) {
        let type_id = TypeId::of::<T>();
        self.states.write().unwrap().remove(&type_id);
    }
}

lazy_static::lazy_static! {
    pub static ref GLOBAL_STATE: GlobalStateContainer = GlobalStateContainer::new();
}

// Renomeie esta função para evitar conflito com a importação
pub fn get_global_state_container<T: Default + 'static + Send + Sync>() -> Arc<RwLock<T>> {
    GLOBAL_STATE.get_or_create::<T>()
}

pub fn drop_global_state<T: 'static>() {
    GLOBAL_STATE.drop::<T>();
}

pub fn __register_global_state_immediately<T: 'static + Send + Sync>(
    name: &'static str,
    state_creator: impl Fn() -> GlobalState<T> + Send + Sync + 'static
) {
    let register = || {
        register_global_state(name, state_creator());
    };
    register();
}