# Resources and Async Data Loading

Resources wrap async operations and integrate with Leptos reactivity.

## Resource Types

| Type            | SSR | Use Case                                   |
| --------------- | --- | ------------------------------------------ |
| `Resource`      | ✅  | Default for SSR apps, serializes to client |
| `LocalResource` | ❌  | CSR-only, `!Send` browser APIs             |
| `OnceResource`  | ✅  | Load once, never refetch                   |

## Resource (SSR)

Takes two functions: **source** (tracked) and **fetcher** (untracked):

```rust
let (count, set_count) = signal(0);

// Reloads when count changes
let data = Resource::new(
    move || count.get(),           // source - tracked
    |count| load_data(count)       // fetcher - untracked
);
```

**Why two functions?**

- Source signals are tracked for reactivity
- Fetcher is untracked to avoid re-running during hydration
- Allows SSR serialization without running fetcher again on client

## LocalResource (CSR)

Single function that tracks signals:

```rust
let (count, set_count) = signal(0);

// Simpler API, but no SSR serialization
let data = LocalResource::new(move || load_data(count.get()));
```

## OnceResource

Loads once, never refetches:

```rust
let initial_data = OnceResource::new(load_initial_config());
```

## Accessing Resource Values

Resources implement signal traits but return `Option<T>`:

```rust
// .get() - clones value, returns None while loading
let value: Option<Data> = data.get();

// .read() - returns reference, None while loading
if let Some(d) = data.read().as_ref() {
    println!("Data: {:?}", d);
}

// .with() - callback with Option<&T>
data.with(|opt| {
    if let Some(d) = opt {
        println!("Loaded: {}", d.name);
    }
});
```

## Manual Refetch

```rust
let data = Resource::new(|| (), |_| fetch_data());

view! {
    <button on:click=move |_| data.refetch()>
        "Reload Data"
    </button>
}
```

## Pattern: Loading State

```rust
let data = Resource::new(move || id.get(), fetch_item);

view! {
    {move || match data.get() {
        None => view! { <p>"Loading..."</p> }.into_any(),
        Some(Ok(item)) => view! { <ItemDisplay item/> }.into_any(),
        Some(Err(e)) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
    }}
}
```

## With Suspense (Recommended)

```rust
let data = Resource::new(move || id.get(), fetch_item);

view! {
    <Suspense fallback=|| "Loading...">
        {move || data.get().map(|result| match result {
            Ok(item) => view! { <ItemDisplay item/> },
            Err(e) => view! { <p>{e.to_string()}</p> },
        })}
    </Suspense>
}
```

## Multiple Resources

```rust
let user = Resource::new(|| user_id.get(), fetch_user);
let posts = Resource::new(|| user_id.get(), fetch_posts);

view! {
    <Suspense fallback=|| "Loading...">
        <h1>{move || user.get().map(|u| u.name)}</h1>
        <ul>
            {move || posts.get().map(|posts| {
                posts.into_iter().map(|p| view! { <li>{p.title}</li> }).collect_view()
            })}
        </ul>
    </Suspense>
}
```

## Resource Dependencies

Resources automatically track signals in their source function:

```rust
let (category, set_category) = signal("all".to_string());
let (page, set_page) = signal(1);

// Refetches when category OR page changes
let items = Resource::new(
    move || (category.get(), page.get()),
    |(cat, pg)| fetch_items(cat, pg)
);
```

## Complete Example

```rust
async fn load_data(count: i32) -> i32 {
    gloo_timers::future::TimeoutFuture::new(1_000).await;
    count * 10
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    let async_data = LocalResource::new(move || load_data(count.get()));
    let stable = LocalResource::new(|| load_data(1));

    view! {
        <button on:click=move |_| *set_count.write() += 1>
            "Click me"
        </button>
        <p>"Stable: " {move || stable.get()}</p>
        <p>"Count: " {count}</p>
        <p>"Async: "
            {move || async_data.get()
                .map(|v| format!("Result: {v}"))
                .unwrap_or_else(|| "Loading...".into())
            }
        </p>
    }
}
```
