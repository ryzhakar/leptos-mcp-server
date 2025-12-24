# Forms in Leptos

## Basic Controlled Input

```rust
use leptos::prelude::*;

#[component]
fn ControlledInput() -> impl IntoView {
    let (name, set_name) = signal(String::new());

    view! {
        <input
            type="text"
            prop:value=move || name.get()
            on:input=move |ev| set_name.set(event_target_value(&ev))
        />
        <p>"Hello, " {move || name.get()}</p>
    }
}
```

## ActionForm with Server Function

```rust
#[server(Login)]
pub async fn login(email: String, password: String) -> Result<(), ServerFnError> {
    // Validate and authenticate
    Ok(())
}

#[component]
fn LoginForm() -> impl IntoView {
    let action = ServerAction::<Login>::new();
    let pending = action.pending();
    let result = action.value();

    view! {
        <ActionForm action=action>
            <input type="email" name="email" required />
            <input type="password" name="password" required />
            <button type="submit" disabled=pending>
                {move || if pending.get() { "Logging in..." } else { "Login" }}
            </button>
        </ActionForm>

        {move || result.get().map(|r| match r {
            Ok(_) => view! { <p class="success">"Logged in!"</p> }.into_any(),
            Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
        })}
    }
}
```

## Form Validation

```rust
#[component]
fn ValidatedForm() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (error, set_error) = signal(Option::<String>::None);

    let validate_email = move |_| {
        let value = email.get();
        if !value.contains('@') {
            set_error.set(Some("Invalid email".to_string()));
        } else {
            set_error.set(None);
        }
    };

    view! {
        <input
            type="email"
            prop:value=move || email.get()
            on:input=move |ev| set_email.set(event_target_value(&ev))
            on:blur=validate_email
        />
        {move || error.get().map(|e| view! { <span class="error">{e}</span> })}
    }
}
```

## Multi-field Form

```rust
#[component]
fn RegisterForm() -> impl IntoView {
    let (form, set_form) = signal(RegisterData::default());

    let update_field = move |field: &str, value: String| {
        set_form.update(|f| match field {
            "name" => f.name = value,
            "email" => f.email = value,
            _ => {}
        });
    };

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        // Handle submission
        let data = form.get();
        // ...
    };

    view! {
        <form on:submit=on_submit>
            <input
                type="text"
                prop:value=move || form.get().name
                on:input=move |ev| update_field("name", event_target_value(&ev))
            />
            <input
                type="email"
                prop:value=move || form.get().email
                on:input=move |ev| update_field("email", event_target_value(&ev))
            />
            <button type="submit">"Register"</button>
        </form>
    }
}
```

## Best Practices

1. Use `ActionForm` for server function submissions
2. Use `prop:value` (not `value`) for controlled inputs
3. Use `event_target_value(&ev)` to get input value
4. Check `action.pending()` for loading states
5. Handle both success and error cases from `action.value()`
