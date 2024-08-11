use std::any::Any;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::marker::PhantomData;

pub type StateChangeCallback<T> = Box<dyn Fn(&str, &T) + Send + Sync>;

pub struct StateManager {
    states: RwLock<HashMap<String, Arc<RwLock<Box<dyn Any + Send + Sync>>>>>,
    callbacks: RwLock<HashMap<String, Vec<Box<dyn Any + Send + Sync>>>>,
}

impl StateManager {
    pub fn new() -> Self {
        StateManager {
            states: RwLock::new(HashMap::new()),
            callbacks: RwLock::new(HashMap::new()),
        }
    }

    pub fn register_state<T: 'static + Clone + Send + Sync>(&self, id: &str, state: T) {
        println!("Registering state: {}", id);
        let boxed_state: Box<dyn Any + Send + Sync> = Box::new(state);
        self.states.write().unwrap().insert(id.to_string(), Arc::new(RwLock::new(boxed_state)));
    }

    pub fn get_state<T: 'static + Send + Sync>(&self, id: &str) -> Option<State<T>> {
    println!("Getting state: {}", id);
    self.states.read().unwrap().get(id).cloned().map(|inner| State::new(inner, id.to_string()))
}

    pub fn register_callback<T: 'static + Send + Sync>(&self, id: &str, callback: StateChangeCallback<T>) {
        println!("Registering callback for: {}", id);
        let mut callbacks = self.callbacks.write().unwrap();
        callbacks.entry(id.to_string()).or_insert_with(Vec::new).push(Box::new(callback));
    }

    pub fn notify_state_change<T: 'static + Send + Sync>(&self, id: &str, field: &str, state: &T) {
        println!("Notifying state change for: {}, field: {}", id, field);
        if let Some(callbacks) = self.callbacks.read().unwrap().get(id) {
            for callback in callbacks {
                if let Some(typed_callback) = callback.downcast_ref::<StateChangeCallback<T>>() {
                    typed_callback(field, state);
                }
            }
        }
    }
}

pub static GLOBAL_STATE_MANAGER: Lazy<StateManager> = Lazy::new(StateManager::new);

pub struct State<T: 'static + Send + Sync> {
    inner: Arc<RwLock<Box<dyn Any + Send + Sync>>>,
    id: String,
    _phantom: PhantomData<T>,
}

impl<T: 'static + Send + Sync> State<T> {
    pub fn new(inner: Arc<RwLock<Box<dyn Any + Send + Sync>>>, id: String) -> Self {
        State {
            inner,
            id,
            _phantom: PhantomData,
        }
    }

    pub fn read(&self) -> StateReadGuard<T> {
        StateReadGuard(self.inner.read().unwrap(), PhantomData)
    }

    pub fn write(&self) -> StateWriteGuard<T> {
        StateWriteGuard::new(self.inner.write().unwrap(), self.id.clone())
    }
}

pub struct StateReadGuard<'a, T: 'static + Send + Sync>(
    std::sync::RwLockReadGuard<'a, Box<dyn Any + Send + Sync>>,
    PhantomData<T>,
);

impl<'a, T: 'static + Send + Sync> std::ops::Deref for StateReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.downcast_ref::<T>().unwrap()
    }
}

pub struct StateWriteGuard<'a, T: 'static + Send + Sync> {
    inner: std::sync::RwLockWriteGuard<'a, Box<dyn Any + Send + Sync>>,
    _phantom: PhantomData<T>,
    id: String,
}

impl<'a, T: 'static + Send + Sync> StateWriteGuard<'a, T> {
    pub fn new(inner: std::sync::RwLockWriteGuard<'a, Box<dyn Any + Send + Sync>>, id: String) -> Self {
        StateWriteGuard {
            inner,
            _phantom: PhantomData,
            id,
        }
    }
}

impl<'a, T: 'static + Send + Sync> std::ops::Deref for StateWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.downcast_ref::<T>().unwrap()
    }
}

impl<'a, T: 'static + Send + Sync> std::ops::DerefMut for StateWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.downcast_mut::<T>().unwrap()
    }
}

impl<'a, T: 'static + Send + Sync> Drop for StateWriteGuard<'a, T> {
    fn drop(&mut self) {
        let state = self.inner.downcast_ref::<T>().unwrap();
        GLOBAL_STATE_MANAGER.notify_state_change(&self.id, "all", state);
    }
}