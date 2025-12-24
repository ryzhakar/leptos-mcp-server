# Leptos Components

## Creating a Component

Components are the building blocks of Leptos applications. They are functions marked with `#[component]`.

```rust
use leptos::prelude::*;

#[component]
fn MyComponent() -> impl IntoView {
    view! {
        <div>"Hello from my component!"</div>
    }
}
```

## Props

Components can accept props (properties):

```rust
#[component]
fn Greeting(
    name: String,                           // Required prop
    #[prop(optional)] greeting: String,     // Optional (defaults to Default::default())
    #[prop(default = "Hello")] prefix: &'static str, // With default value
    #[prop(into)] title: String,            // Auto-convert with Into trait
) -> impl IntoView {
    view! {
        <p>{prefix} " " {name}</p>
    }
}

// Usage
view! {
    <Greeting name="World".to_string() />
    <Greeting name="Leptos".to_string() prefix="Hi" />
}
```

## Children

Accept child content with `Children`:

```rust
#[component]
fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="card">
            {children()}
        </div>
    }
}

// Usage
view! {
    <Card>
        <p>"Card content here"</p>
    </Card>
}
```

## Optional Children

```rust
#[component]
fn OptionalCard(
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class="card">
            {children.map(|c| c())}
        </div>
    }
}
```

## Component Naming

- Use **PascalCase** for component names
- Component names must start with uppercase letter
- Example: `MyButton`, `UserProfile`, `NavBar`

## Best Practices

1. Keep components small and focused
2. Use `#[prop(into)]` for flexible string props
3. Use `#[prop(optional)]` with `Option<T>` for optional props
4. Extract reusable logic into separate components
