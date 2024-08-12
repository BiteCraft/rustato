use rustato::*;
use std::sync::{Arc, Mutex};

#[test]
fn test_create_signal() {
    let age = create_signal!(42);
    assert_eq!(age.get(), 42);

    age.set(43);
    assert_eq!(age.get(), 43);

    let changed = Arc::new(Mutex::new(false));
    let changed_clone = Arc::clone(&changed);

    age.on_change(move |old_value, new_value| {
        assert_eq!(*old_value, 43);
        assert_eq!(*new_value, 44);
        *changed_clone.lock().unwrap() = true;
    });

    age.set(44);
    assert!(*changed.lock().unwrap());
}


#[AutoState]
#[derive(Default)]
pub struct CounterState {
    pub count: Signal<i32>,
}

impl CounterState {
    pub fn increment(&self) {
        self.count.set(self.count.get() + 1);
    }
}

#[test]
fn test_create_global_state() {
    // Verificar se o estado global foi criado corretamente
    assert!(get_state!(CounterState).read().count.get() == 0, "Initial state should be 0");

    let changed = Arc::new(Mutex::new(false));
    let changed_clone = Arc::clone(&changed);
    let callback_count = Arc::new(Mutex::new(0));
    let callback_count_clone = Arc::clone(&callback_count);

    on_state_change!(CounterState, move |field, state| {
        assert_eq!(field, "count", "Changed field should be 'count'");
        *changed_clone.lock().unwrap() = true;
        *callback_count_clone.lock().unwrap() += 1;
    });

    // Primeiro incremento
    {
        let counter_state = get_state!(CounterState);
        let mut write_guard = counter_state.write();
        write_guard.increment();
        write_guard.register_change("count");
    }

    assert_eq!(get_state!(CounterState).read().count.get(), 1, "Count should be 1 after first increment");
    assert!(*changed.lock().unwrap(), "Changed flag should be true after first increment");
    assert_eq!(*callback_count.lock().unwrap(), 1, "Callback should have been called once");

    // Resetar o flag de mudança
    *changed.lock().unwrap() = false;

    // Segundo incremento
    {
        let counter_state = get_state!(CounterState);
        let mut write_guard = counter_state.write();
        write_guard.increment();
        write_guard.register_change("count");
    }

    assert_eq!(get_state!(CounterState).read().count.get(), 2, "Count should be 2 after second increment");
    assert!(*changed.lock().unwrap(), "Changed flag should be true after second increment");
    assert_eq!(*callback_count.lock().unwrap(), 2, "Callback should have been called twice");
}