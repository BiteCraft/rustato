use std::sync::Arc;
use std::collections::HashMap;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use once_cell::sync::Lazy;
use std::any::Any;

pub struct GlobalState<T: ?Sized + Send + Sync> {
    state: Arc<RwLock<T>>,
    callbacks: Arc<RwLock<Vec<Box<dyn Fn(&str, &T) + Send + Sync>>>>,
    changed_fields: Arc<RwLock<Vec<String>>>,
}

impl<T: Send + Sync + 'static> GlobalState<T> {
    pub fn new(initial: T) -> Self {
        Self {
            state: Arc::new(RwLock::new(initial)),
            callbacks: Arc::new(RwLock::new(Vec::new())),
            changed_fields: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl<T: ?Sized + Send + Sync + 'static> GlobalState<T> {
    pub fn register_change(&self, field: &str) {
        self.changed_fields.write().push(field.to_string());
    }

    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.state.read()
    }

    pub fn write(&self) -> GlobalStateWriteGuard<'_, T> {
        GlobalStateWriteGuard {
            state: Some(self.state.write()),
            global_state: self,
            changed_fields: Vec::new(),
        }
    }

    pub fn on_change<F>(&self, callback: F)
    where
        F: Fn(&str, &T) + Send + Sync + 'static,
    {
        self.callbacks.write().push(Box::new(callback));
    }

    pub(crate) fn run_callbacks(&self, field_changed: &str) {
        let state = self.state.read();
        let callbacks = self.callbacks.read();
        for callback in callbacks.iter() {
            callback(field_changed, &*state);
        }
    }
}

pub struct GlobalStateWriteGuard<'a, T: ?Sized + Send + Sync + 'static> {
    state: Option<RwLockWriteGuard<'a, T>>,
    global_state: &'a GlobalState<T>,
    changed_fields: Vec<String>,
}

impl<'a, T: ?Sized + Send + Sync + 'static> GlobalStateWriteGuard<'a, T> {
    pub fn register_change(&mut self, field: &str) {
        self.changed_fields.push(field.to_string());
    }
}

impl<'a, T: ?Sized + Send + Sync + 'static> Drop for GlobalStateWriteGuard<'a, T> {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            // Release the write lock
            drop(state);
            // Run callbacks after releasing the lock
            for field in &self.changed_fields {
                self.global_state.run_callbacks(field);
            }
            self.global_state.changed_fields.write().clear();
        }
    }
}

impl<'a, T: ?Sized + Send + Sync + 'static> std::ops::Deref for GlobalStateWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.state.as_ref().unwrap()
    }
}

impl<'a, T: ?Sized + Send + Sync + 'static> std::ops::DerefMut for GlobalStateWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.state.as_mut().unwrap()
    }
}

static GLOBAL_STATES: Lazy<RwLock<HashMap<String, Arc<dyn Any + Send + Sync>>>> = 
    Lazy::new(|| RwLock::new(HashMap::new()));

pub fn register_global_state<T: 'static + Send + Sync>(name: &str, state: GlobalState<T>) {
    GLOBAL_STATES.write().insert(name.to_string(), Arc::new(state));
}

pub fn get_global_state<T: 'static + Send + Sync>(name: &str) -> Option<Arc<GlobalState<T>>> {
    GLOBAL_STATES.read()
        .get(name)
        .and_then(|state| state.clone().downcast::<GlobalState<T>>().ok())
}

pub fn unregister_global_state(name: &str) {
    GLOBAL_STATES.write().remove(name);
}