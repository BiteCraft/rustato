<h1 align="center">
  <br>
  <img width="100%" src="./rustato_logo.png" alt="Coinbase Commerce Client">
  <br>
  Rustato State Manager
  <br>
</h1>

<h4 align="center">A generical thread-safe global state manager for Rust</h4>

<p align="center">
	<img alt="Crates.io Version" src="https://img.shields.io/crates/v/rustato">
    <img src="https://img.shields.io/crates/d/rustato"  alt="Total Downloads">
    <img src="https://img.shields.io/crates/size/rustato" alt="size">
    <img src="https://img.shields.io/github/license/BiteCraft/rustato" alt="License">
</p>

<p align="center">
  <a href="#introduction">Introduction</a> •
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#advanced-usage">Advanced Usage</a> •
  <a href="#api-reference">Api Reference</a> •
  <a href="#contributing">Contributing</a>
</p>
<sub>This library is a global state management solution for Rust projects, inspired by state management concepts from various ecosystems. Our goal is to provide a robust and easy-to-use tool for Rust developers who need a centralized state management system in their applications.</sub>

## Table of Contents

1. [Introduction](#introduction)
2. [Features](#features)
3. [Installation](#installation)
4. [Usage](#usage)
   - [Creating States](#creating-states)
   - [Accessing States](#accessing-states)
   - [Modifying States](#modifying-states)
   - [Registering Callbacks](#registering-callbacks)
5. [Advanced Usage](#advanced-usage)
   - [Using with Tauri](#using-with-tauri)
   - [Using in Multithreaded Applications](#using-in-multithreaded-applications)
   - [Custom State Types](#custom-state-types)
6. [API Reference](#api-reference)
7. [Best Practices](#best-practices)
8. [Contributing](#contributing)
9. [License](#license)

## Introduction

The Rustato Library is a powerful and flexible solution for managing application state in Rust projects. It provides a simple and intuitive API for creating, accessing, and modifying states, as well as registering callbacks for state changes. This library is particularly useful for applications that require a centralized state management system, such as desktop applications, web servers, or complex CLI tools.

## Features

- Global state management with a singleton pattern
- Type-safe state creation and access
- Automatic generation of getters and setters
- Event system for state changes
- Thread-safe state manipulation
- Easy integration with existing Rust projects
- Macro-based API for reduced boilerplate

## Installation

To use the Rustato Library in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
rustato = "0.1.1"
```

Then, add the following to your main Rust file:

```rust
use rustato::*;
```

## Usage

### Creating States

To create a new state, use the `create_state!` macro:

```rust
use rustato::*;

create_state!(struct AppState {
    status: String,
    window_visible: bool,
    user_count: u32,
});
```

This macro creates a new struct `AppState` with the specified fields and registers it with the global state manager.

### Accessing States

To access a state for read-only operations, use the `get_state!` macro and call the `read` mode:

```rust
let app_state = get_state!(AppState).read();

println!("Current status: {}", app_state.status);
println!("Window visible: {}", app_state.window_visible);
println!("User count: {}", app_state.user_count);
```


### Modifying States

To modify a state, use the `get_state!` macro and call the `write` mode:

```rust
let mut app_state = get_state!(AppState).write();

app_state.status = "Running".to_string();
app_state.window_visible = true;
app_state.user_count = 42;
```

### Registering Callbacks

To register a callback that will be called when a state changes, use the `on_state_change!` macro:

```rust
on_state_change!(CounterState, |field_changed: &str, state: &CounterState| {
    println!("Field changed: {}", field_changed);
    println!("New counter value: {}", state.counter);
});
```

## Advanced Usage

### Using with Tauri

The Rustato Library can be easily integrated with Tauri applications. Here's an example of how to use it in a Tauri app:

```rust
use rustato::*;
use tauri::Manager;

#[tauri::command]
fn increment_counter() -> i32 {
    let mut app_state = get_state!(AppState).write();
    let new_value = app_state.counter + 1;
    app_state.counter = new_value;
    new_value
}

fn main() {
    create_state!(AppState {
        counter: i32,
    });

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            on_state_change!(AppState, |field_changed: &str, state: &CounterState| {
                if field_changed == "counter" {
                    app_handle.emit_all("counter-changed", state.counter).unwrap();
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![increment_counter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

In this example, we create an `AppState` with a `counter` field. We register a callback that emits a Tauri event whenever the counter changes. The `increment_counter` function is exposed as a Tauri command that increments the counter and returns the new value.

### Using in Multithreaded Applications

The Rustato Library is designed to be thread-safe. Here's an example of using it in a multithreaded application:

```rust
use rustato::*;
use std::thread;

fn main() {
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
    println!("Final value: {}", final_state.value);
}
```

In this example, we create 10 threads that all modify the same shared state. The library ensures that these modifications are thread-safe.

### Custom State Types

You can use custom types in your states as long as they implement the necessary traits. Here's an example:


```rust
use rustato::*;

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

fn main() {
    {
        let mut state = get_state!(UserState).write();
        state.current_user = User {
            id: 1,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };
        state.logged_in = true;
    }

    let user_state = get_state!(UserState).read();
    println!("Current user: {:?}", user_state.current_user);
    println!("Logged in: {}", user_state.logged_in);
}
```

## API Reference

### Macros

- `create_state!(struct_definition)`: Creates a new state with the given ID and struct definition.
- `get_state!(struct_definition).read() -> &State`: Returns a reference to the state with the given struct definition.
- `get_state!(struct_definition).write() -> &mut State`: Returns a mutable reference to the state with the given struct definition.
- `on_state_change!(struct_definition, callback: Fn(&str, &State))`: Registers a callback to be called when the state with the given struct definition changes.

### StateManager

- `StateManager::new() -> StateManager`: Creates a new StateManager instance.
- `StateManager::register_state<T: 'static + Clone + Send + Sync>(&self, id: &str, state: T)`: Registers a new state with the given ID and initial value.
- `StateManager::get_state<T: 'static + Send + Sync>(&self, id: &str) -> Option<State<T>>`: Returns the state with the given ID, if it exists.
- `StateManager::register_callback<T: 'static + Send + Sync>(&self, id: &str, callback: StateChangeCallback<T>)`: Registers a callback for the state with the given ID.
- `StateManager::notify_state_change<T: 'static + Send + Sync>(&self, id: &str, field: &str, state: &T)`: Notifies registered callbacks about a state change.

## Best Practices

1. **State Granularity**: Create separate states for different parts of your application to improve performance and reduce unnecessary updates.

2. **Immutable Access**: Use `get_state!(struct_definition).read()` when you only need to read state values to prevent accidental modifications.

3. **Callback Efficiency**: Keep callbacks lightweight and avoid performing heavy computations or I/O operations directly in the callback. Instead, use the callback to trigger other parts of your application.

4. **State Initialization**: Initialize your states as early as possible in your application lifecycle, preferably during the setup phase.

5. **Error Handling**: Always handle potential errors when accessing states, especially when using `get_state!(struct_definition).read()` or `get_state!(struct_definition).write()` with an unknown struct definition.

6. **Thread Safety**: While the library is designed to be thread-safe, be cautious when sharing state across threads and consider using appropriate synchronization primitives when necessary.

7. **Testing**: Create unit tests for your state management logic to ensure that state changes and callbacks work as expected.

## Contributing

Contributions to the Rustato Library are welcome! Please feel free to submit issues, fork the repository and send pull requests!

When contributing to this repository, please first discuss the change you wish to make via issue, email, or any other method with the owners of this repository before making a change.

Please note we have a code of conduct, please follow it in all your interactions with the project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.