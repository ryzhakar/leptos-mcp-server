# Suspense and Async in Leptos

## Suspense Component

Handle loading states for async operations:

```rust
use leptos::prelude::*;

#[component]
fn AsyncData() -> impl IntoView {
    let data = Resource::new(|| (), |_| fetch_data());

    view! {
        <Suspense fallback=|| view! { <p>"Loading..."</p> }>
            {move || data.get().map(|result| match result {
                Ok(data) => view! { <p>{data}</p> }.into_any(),
                Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
            })}
        </Suspense>
    }
}
```

## Custom Loading Spinners

```rust
#[component]
fn LoadingSpinner() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center p-4">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
        </div>
    }
}

// Usage
view! {
    <Suspense fallback=LoadingSpinner>
        <AsyncContent />
    </Suspense>
}
```

## Transition Component

Keep showing old content while loading new:

```rust
use leptos::prelude::*;
use leptos_router::components::Transition;

#[component]
fn SmoothNavigation() -> impl IntoView {
    let data = Resource::new(|| (), fetch_page_data);

    view! {
        <Transition fallback=|| "Loading...">
            {move || data.get().map(|d| view! { <PageContent data=d /> })}
        </Transition>
    }
}
```

## Resource with Dependencies

```rust
#[component]
fn FilteredList() -> impl IntoView {
    let (filter, set_filter) = signal(String::new());

    // Resource re-fetches when filter changes
    let items = Resource::new(
        move || filter.get(),
        |filter| fetch_items(filter)
    );

    view! {
        <input
            type="text"
            prop:value=move || filter.get()
            on:input=move |ev| set_filter.set(event_target_value(&ev))
        />

        <Suspense fallback=|| "Loading...">
            {move || items.get().map(|result| {
                // Render items
            })}
        </Suspense>
    }
}
```

## Await Component

Simpler async handling for one-shot data:

```rust
#[component]
fn SimpleAsync() -> impl IntoView {
    async fn load_data() -> String {
        // Async operation
        "Data loaded!".to_string()
    }

    view! {
        <Await
            future=load_data()
            let:data
        >
            <p>{data}</p>
        </Await>
    }
}
```

## Nested Suspense

```rust
view! {
    <Suspense fallback=|| "Loading page...">
        <PageLayout>
            <Suspense fallback=|| "Loading sidebar...">
                <Sidebar />
            </Suspense>
            <Suspense fallback=|| "Loading content...">
                <MainContent />
            </Suspense>
        </PageLayout>
    </Suspense>
}
```

## Best Practices

1. Always provide meaningful fallback content
2. Use `Transition` for navigation to prevent jarring UX
3. Handle both loading and error states
4. Use nested `Suspense` for independent loading sections
5. Consider skeleton loaders for better perceived performance
