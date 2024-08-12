use std::fmt;
use std::sync::{Arc, Mutex};
use parking_lot::RwLock;
use std::any::Any;
use crate::GlobalState;

pub struct Signal<T: 'static + Send + Sync + Clone> {
    value: Arc<RwLock<T>>,
    callbacks: Arc<Mutex<Vec<Box<dyn Fn(&T, &T) + Send + Sync + 'static>>>>,
    field_name: String,
    parent: Option<Arc<dyn Any + Send + Sync>>,
}

impl<T: Clone + 'static + Send + Sync + Default + fmt::Debug> fmt::Debug for Signal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Signal")
            .field("value", &*self.value.read())
            .finish()
    }
}

impl<T: Clone + 'static + Send + Sync> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Signal {
            value: Arc::clone(&self.value),
            callbacks: Arc::clone(&self.callbacks),
            field_name: self.field_name.clone(),
            parent: self.parent.clone(),
        }
    }
}


impl<T: Clone + 'static + Send + Sync + Default + PartialEq> PartialEq for Signal<T> {
    fn eq(&self, other: &Self) -> bool {
        *self.value.read() == *other.value.read()
    }
}

impl<T: 'static + Send + Sync + Clone + Default> Default for Signal<T> {
    fn default() -> Self {
        Signal::new(T::default(), String::from("default"))
    }
}

impl<T: 'static + Send + Sync + Clone> Signal<T> {
    pub fn new(initial_value: T, field_name: String) -> Self {
        Signal {
            value: Arc::new(RwLock::new(initial_value)),
            callbacks: Arc::new(Mutex::new(Vec::new())),
            field_name,
            parent: None,
        }
    }

    pub fn with_parent(mut self, parent: Arc<dyn Any + Send + Sync>) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn set(&self, new_value: T) {
        let old_value = self.value.read().clone();
        *self.value.write() = new_value.clone();
    
        let callbacks = self.callbacks.lock().unwrap();
        for callback in callbacks.iter() {
            callback(&old_value, &new_value);
        }
    
        if let Some(parent) = &self.parent {
            if let Some(global_state) = parent.downcast_ref::<GlobalState<dyn Any + Send + Sync>>() {
                global_state.register_change(&self.field_name);
            }
        }
    }

    pub fn get(&self) -> T {
        self.value.read().clone()
    }

    pub fn on_change<F>(&self, callback: F)
    where
        F: Fn(&T, &T) + Send + Sync + 'static,
    {
        self.callbacks.lock().unwrap().push(Box::new(callback));
    }
}