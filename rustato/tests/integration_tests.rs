use rustato::*;
use std::thread;

#[test]
fn test_create_and_use_state() {
    create_state!(struct AppState {
        status: String,
        window_visible: bool,
        user_count: u32,
    });

    {
        let mut app_state = get_state!(AppState).write();
        app_state.status = "Running".to_string();
        app_state.window_visible = true;
        app_state.user_count = 42;
    }

    {
        let app_state = get_state!(AppState).read();
        assert_eq!(app_state.status, "Running".to_string());
        assert_eq!(app_state.window_visible, true);
        assert_eq!(app_state.user_count, 42);
    }
}

#[test]
fn test_multithreaded_state() {
    create_state!(struct SharedState {
        value: i32,
    });

    let handles: Vec<_> = (0..10).map(|i| {
        thread::spawn(move || {
            let mut state = get_state!(SharedState).write();
            state.value += i;
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let final_state = get_state!(SharedState).read();
    assert_eq!(final_state.value, 45); // 0 + 1 + 2 + ... + 9 = 45
}

#[test]
fn test_custom_state_types() {
    #[derive(Clone, Default)]
    struct User {
        id: u64,
        name: String,
        email: String,
    }

    create_state!(struct UserState {
        current_user: User,
        logged_in: bool,
    });

    {
        let mut state = get_state!(UserState).write();
        state.current_user = User {
            id: 1,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };
        state.logged_in = true;
    }

    {
        let user_state = get_state!(UserState).read();
        assert_eq!(user_state.current_user.id, 1);
        assert_eq!(user_state.current_user.name, "John Doe");
        assert_eq!(user_state.current_user.email, "john@example.com");
        assert!(user_state.logged_in);
    }
}

#[test]
fn test_state_change_callback() {
    use std::sync::{Arc, Mutex};

    create_state!(struct CounterState {
        counter: i32,
    });

    let callback_called = Arc::new(Mutex::new(false));
    let callback_called_clone = Arc::clone(&callback_called);

    on_state_change!(CounterState, move |field_changed: &str, state: &CounterState| {
        println!("Callback called with field: {}, counter: {}", field_changed, state.counter);
        if field_changed == "all" {
            assert_eq!(state.counter, 1);
            *callback_called_clone.lock().unwrap() = true;
        }
    });

    {
        let mut state = get_state!(CounterState).write();
        state.counter = 1;
    }

    // Add a small delay to ensure the callback has time to execute
    std::thread::sleep(std::time::Duration::from_millis(100));

    assert!(*callback_called.lock().unwrap(), "Callback was not called");
}

#[test]
fn test_multiple_states() {
    create_state!(struct StateA {
        value_a: i32,
    });

    create_state!(struct StateB {
        value_b: String,
    });

    {
        let mut state_a = get_state!(StateA).write();
        state_a.value_a = 42;
    }

    {
        let mut state_b = get_state!(StateB).write();
        state_b.value_b = "Hello, World!".to_string();
    }

    {
        let state_a = get_state!(StateA).read();
        let state_b = get_state!(StateB).read();
        assert_eq!(state_a.value_a, 42);
        assert_eq!(state_b.value_b, "Hello, World!");
    }
}