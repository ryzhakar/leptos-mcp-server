# Leptos Signals (Reactivity System)

Signals are the core reactivity primitive in Leptos. They hold a value and notify subscribers when the value changes.

## Creating Signals

```rust
use leptos::prelude::*;

// Create a signal with initial value
let (count, set_count) = signal(0);
```

## Reading Signals

| Method             | Description                     | Use Case                                 |
| ------------------ | ------------------------------- | ---------------------------------------- |
| `.get()`           | Clones value, tracks reactivity | Most common, for `Clone` types           |
| `.read()`          | Returns read guard (reference)  | Avoid cloning, e.g. `names.read().len()` |
| `.with(\|v\| ...)` | Takes callback with `&T`        | Complex operations with reference        |

```rust
// .get() - clones the value (most common)
let value = count.get();

// .read() - returns a reference without cloning
let len = names.read().len();

// .with() - callback with reference
count.with(|n| println!("Count is {}", n));
```

## Writing Signals

| Method               | Description               | Use Case                 |
| -------------------- | ------------------------- | ------------------------ |
| `.set(value)`        | Replace entire value      | Most common              |
| `.write()`           | Returns mutable reference | In-place mutation        |
| `.update(\|v\| ...)` | Callback with `&mut T`    | Complex in-place updates |

```rust
// .set() - replace value (most common)
set_count.set(5);

// .write() - get mutable reference
set_count.write().push("Alice".to_string());

// .update() - callback for in-place update
set_count.update(|n| *n += 1);
```

## Efficiency: read/write vs get/set

Using `.read()` and `.write()` can be more efficient for complex types:

```rust
// Inefficient: clones entire Vec
if names.get().is_empty() {
    set_names(vec!["Alice".to_string()]);
}

// Efficient: no clone, in-place mutation
if names.read().is_empty() {
    set_names.write().push("Alice".to_string());
}
```

## Derived Signals

Create computed values that automatically update:

```rust
let (count, set_count) = signal(1);

// Derived signal - recalculates when count changes
let double_count = move || count.get() * 2;

// Memoized - caches result, only recalculates when dependencies change
let memoized = Memo::new(move |_| count.get() * 2);
```

## Using Signals in Views

**IMPORTANT**: Always use `move ||` in views for reactive updates!

```rust
view! {
    // ✅ CORRECT - reactive
    <p>{move || count.get()}</p>

    // ❌ WRONG - not reactive, only renders initial value
    <p>{count.get()}</p>

    // ✅ CORRECT - derived signal
    <p>{double_count}</p>
}
```

## Nightly Syntax

With `nightly` feature enabled:

```rust
// Shorthand syntax
set_count(1);        // same as set_count.set(1)
count()              // same as count.get()
```

## Thread-Local Signals

For `!Send` types (browser APIs), use local variants:

| Standard          | Local                   |
| ----------------- | ----------------------- |
| `signal()`        | `signal_local()`        |
| `RwSignal::new()` | `RwSignal::new_local()` |
| `Resource`        | `LocalResource`         |
| `Action::new()`   | `Action::new_local()`   |

## Signal Dependencies

**Best Practices:**

1. **B = f(A)**: Use derived signal or memo

```rust
let double = move || count.get() * 2;
```

2. **C = f(A, B)**: Derive from multiple signals

```rust
let full_name = move || format!("{} {}", first_name.read(), last_name.read());
```

3. **A and B updated together**: Update both explicitly

```rust
let clear = move |_| {
    set_a.set(0);
    set_b.set(0);
};
```

4. **Avoid**: Effects that write to signals (causes reactive spaghetti)
