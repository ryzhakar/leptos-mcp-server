//! Leptos documentation content
//!
//! Contains documentation sections for the Leptos framework.

/// Documentation section
#[derive(Debug, Clone)]
pub struct DocSection {
    pub title: String,
    pub path: String,
    pub use_cases: String,
    pub content: String,
}

/// Get all available documentation sections
pub fn list_sections() -> Vec<DocSection> {
    vec![
        DocSection {
            title: "Getting Started".to_string(),
            path: "getting-started".to_string(),
            use_cases: "new project, setup, installation, basics, hello world".to_string(),
            content: include_str!("../docs/getting-started.md").to_string(),
        },
        DocSection {
            title: "Components".to_string(),
            path: "components".to_string(),
            use_cases: "UI, view, component, props, children, #[component], always".to_string(),
            content: include_str!("../docs/components.md").to_string(),
        },
        DocSection {
            title: "Signals".to_string(),
            path: "signals".to_string(),
            use_cases: "state, reactivity, signals, derived, effects, get, set, read, write, update, always".to_string(),
            content: include_str!("../docs/signals.md").to_string(),
        },
        DocSection {
            title: "Views".to_string(),
            path: "views".to_string(),
            use_cases: "view macro, dynamic classes, dynamic styles, attributes, class:, style:, events, always".to_string(),
            content: include_str!("../docs/views.md").to_string(),
        },
        DocSection {
            title: "Resources".to_string(),
            path: "resources".to_string(),
            use_cases: "async, data loading, Resource, LocalResource, OnceResource, fetch, API".to_string(),
            content: include_str!("../docs/resources.md").to_string(),
        },
        DocSection {
            title: "Actions".to_string(),
            path: "actions".to_string(),
            use_cases: "mutations, POST, forms, ActionForm, ServerAction, submit, create, update, delete".to_string(),
            content: include_str!("../docs/actions.md").to_string(),
        },
        DocSection {
            title: "Server Functions".to_string(),
            path: "server-functions".to_string(),
            use_cases: "backend, API, database, server, SSR, #[server], extractors, Axum".to_string(),
            content: include_str!("../docs/server-functions.md").to_string(),
        },
        DocSection {
            title: "Routing".to_string(),
            path: "routing".to_string(),
            use_cases: "navigation, pages, routes, params, nested routes, Router".to_string(),
            content: include_str!("../docs/routing.md").to_string(),
        },
        DocSection {
            title: "Forms".to_string(),
            path: "forms".to_string(),
            use_cases: "form, input, validation, submit, controlled input, prop:value".to_string(),
            content: include_str!("../docs/forms.md").to_string(),
        },
        DocSection {
            title: "Error Handling".to_string(),
            path: "error-handling".to_string(),
            use_cases: "errors, ErrorBoundary, Result, ServerFnError, try".to_string(),
            content: include_str!("../docs/error-handling.md").to_string(),
        },
        DocSection {
            title: "Suspense".to_string(),
            path: "suspense".to_string(),
            use_cases: "loading, async, Suspense, Transition, streaming, fallback".to_string(),
            content: include_str!("../docs/suspense.md").to_string(),
        },
    ]
}

/// Get a specific documentation section by path or title
pub fn get_section(query: &str) -> Option<DocSection> {
    let query_lower = query.to_lowercase();
    list_sections().into_iter().find(|s| {
        s.path.to_lowercase().contains(&query_lower)
            || s.title.to_lowercase().contains(&query_lower)
    })
}
