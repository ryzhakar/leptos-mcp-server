# Leptos Routing

## Setup

```rust
use leptos::prelude::*;
use leptos_router::{
    components::{Router, Routes, Route, ParentRoute},
    StaticSegment, ParamSegment,
};

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found">
                <Route path=StaticSegment("") view=HomePage />
                <Route path=StaticSegment("about") view=AboutPage />
                <Route path=(StaticSegment("user"), ParamSegment("id")) view=UserPage />
            </Routes>
        </Router>
    }
}
```

## Route Types

### Static Segment

```rust
// Matches: /about
<Route path=StaticSegment("about") view=AboutPage />
```

### Param Segment

```rust
// Matches: /user/123, /user/abc
<Route path=(StaticSegment("user"), ParamSegment("id")) view=UserPage />

// Access the param
#[component]
fn UserPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.get().get("id").unwrap_or_default();

    view! { <p>"User ID: " {id}</p> }
}
```

## Nested Routes

```rust
<ParentRoute path=StaticSegment("dashboard") view=DashboardLayout>
    <Route path=StaticSegment("") view=DashboardHome />
    <Route path=StaticSegment("settings") view=DashboardSettings />
    <Route path=StaticSegment("profile") view=DashboardProfile />
</ParentRoute>
```

Layout component uses `<Outlet />`:

```rust
#[component]
fn DashboardLayout() -> impl IntoView {
    view! {
        <div class="dashboard">
            <nav>/* Sidebar */</nav>
            <main>
                <Outlet />  // Child routes render here
            </main>
        </div>
    }
}
```

## Navigation

### Link Component

```rust
use leptos_router::components::A;

view! {
    <A href="/about">"About"</A>
    <A href="/user/123">"User 123"</A>
}
```

### Programmatic Navigation

```rust
use leptos_router::hooks::use_navigate;

#[component]
fn MyComponent() -> impl IntoView {
    let navigate = use_navigate();

    let go_home = move |_| {
        navigate("/", Default::default());
    };

    view! {
        <button on:click=go_home>"Go Home"</button>
    }
}
```

## Query Parameters

```rust
use leptos_router::hooks::use_query_map;

#[component]
fn SearchPage() -> impl IntoView {
    let query = use_query_map();
    let search_term = move || query.get().get("q").unwrap_or_default();

    // URL: /search?q=rust
    view! {
        <p>"Searching for: " {search_term}</p>
    }
}
```

## Best Practices

1. Use `ParentRoute` for shared layouts
2. Define route paths as tuples for multiple segments
3. Use `<A>` component for navigation links
4. Access params with `use_params_map()` or typed `use_params::<T>()`
