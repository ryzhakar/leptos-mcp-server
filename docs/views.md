# View: Dynamic Classes, Styles, and Attributes

The `view!` macro supports dynamic updates to classes, styles, and attributes.

## Dynamic Classes

Use `class:` syntax for reactive class toggling:

```rust
let (count, set_count) = signal(0);

view! {
    <button
        on:click=move |_| *set_count.write() += 1
        // Add 'red' class when count is odd
        class:red=move || count.get() % 2 == 1
    >
        "Click me"
    </button>
}
```

### Class Name Variations

```rust
// Simple class name
class:active=move || is_active.get()

// Complex class names (with special characters)
class=("button-20", move || count.get() % 2 == 1)

// Multiple classes under one condition
class=(["btn", "rounded", "shadow"], move || is_enabled.get())
```

## Dynamic Styles

Use `style:` syntax for individual CSS properties:

```rust
view! {
    <button
        // Static style attribute
        style="position: absolute"
        // Dynamic CSS properties
        style:left=move || format!("{}px", count.get() + 100)
        style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
        // Static CSS property
        style:max-width="400px"
        // CSS variable
        style=("--columns", move || count.get().to_string())
    >
        "Click to Move"
    </button>
}
```

## Dynamic Attributes

Pass functions to attributes for reactive updates:

```rust
let (count, set_count) = signal(0);

view! {
    // Static attribute
    <progress max="50" />

    // Dynamic attribute - updates reactively
    <progress max="50" value=count />

    // With explicit move || syntax
    <progress max="50" value=move || count.get() />
}
```

### Boolean Attributes

```rust
let (disabled, set_disabled) = signal(false);

view! {
    // Disabled when signal is true
    <button disabled=disabled>"Submit"</button>

    // Disabled based on condition
    <button disabled=move || count.get() >= 10>"Submit"</button>
}
```

## Derived Signals

Reuse computed values across multiple places:

```rust
let (count, set_count) = signal(0);

// Derived signal - reactive computed value
let double_count = move || count.get() * 2;

view! {
    // Use in attribute
    <progress max="100" value=double_count />

    // Use in text
    <p>"Double: " {double_count}</p>

    // Use in class
    <div class:high=move || double_count() > 50>
        "Status"
    </div>
}
```

> **Note**: Derived signals run once per access. For expensive calculations, use `Memo::new()` to cache results.

## Event Handlers

Use `on:event_name` syntax:

```rust
view! {
    <button
        on:click=move |_| set_count.set(count.get() + 1)
        on:mouseover=move |_| set_hovered.set(true)
        on:mouseout=move |_| set_hovered.set(false)
    >
        "Hover me"
    </button>

    // With event parameter
    <input
        on:input=move |ev| {
            let value = event_target_value(&ev);
            set_input.set(value);
        }
    />
}
```

## Raw HTML Injection

⚠️ **Security Warning**: Escape untrusted content to prevent XSS!

```rust
let html = "<p>This HTML will be injected.</p>";

view! {
    <div inner_html=html />
}
```

## Complete Example

```rust
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    view! {
        <button
            on:click=move |_| *set_count.write() += 1
            class:red=move || count.get() % 2 == 1
            style:transform=move || format!("scale({})", 1.0 + count.get() as f64 * 0.1)
        >
            "Click me"
        </button>

        <progress max="50" value=count />
        <progress max="100" value=double_count />

        <p>"Count: " {count}</p>
        <p>"Double: " {double_count}</p>
    }
}
```
