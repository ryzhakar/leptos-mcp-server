# Getting Started with Leptos

## What is Leptos?

Leptos is a full-stack, isomorphic Rust web framework. It allows you to write both your frontend and backend in Rust, with seamless SSR (Server-Side Rendering) and hydration.

## Installation

```bash
# Install cargo-leptos
cargo install cargo-leptos --locked

# Create new project
cargo leptos new --git leptos-rs/start-axum

# Add WASM target
rustup target add wasm32-unknown-unknown
```

## Project Structure

```
my-leptos-app/
├── src/
│   ├── main.rs      # Server entry point
│   ├── lib.rs       # Client entry point (WASM)
│   └── app.rs       # Main application component
├── Cargo.toml       # Dependencies and leptos config
└── style/           # CSS/Tailwind files
```

## First Component

```rust
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"Hello, Leptos!"</h1>
    }
}
```

## Running the Development Server

```bash
cargo leptos watch
```

This starts both the server and enables hot-reloading for development.
