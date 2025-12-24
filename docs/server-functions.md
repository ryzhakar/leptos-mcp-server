# Leptos Server Functions

Server functions allow you to call backend code directly from your frontend components. They run only on the server and automatically handle serialization.

## Basic Server Function

```rust
use leptos::prelude::*;
use leptos::server_fn::ServerFnError;

#[server(GetUser)]
pub async fn get_user(id: String) -> Result<User, ServerFnError> {
    // This code runs on the server only
    let user = db.find_user(&id).await?;
    Ok(user)
}
```

## Using with Resources

Resources are the primary way to load data from server functions:

```rust
#[component]
fn UserProfile(id: String) -> impl IntoView {
    // Resource tracks 'id' and reloads when it changes
    let user = Resource::new(
        move || id.clone(),    // source - tracked signals
        |id| get_user(id)      // fetcher - runs on server
    );

    view! {
        <Suspense fallback=|| "Loading...">
            {move || user.get().map(|result| match result {
                Ok(user) => view! { <p>{user.name}</p> }.into_any(),
                Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
            })}
        </Suspense>
    }
}
```

### Resource vs LocalResource

| Type            | SSR Support             | Use Case                   |
| --------------- | ----------------------- | -------------------------- |
| `Resource`      | ✅ Serializes to client | Default for SSR apps       |
| `LocalResource` | ❌                      | Browser-only, `!Send` data |
| `OnceResource`  | ✅                      | Load once, never refetch   |

```rust
// Resource (SSR) - source and fetcher separate
let data = Resource::new(
    move || count.get(),  // source - tracked
    |count| load_data(count)  // fetcher - untracked
);

// LocalResource (CSR) - single fetcher
let data = LocalResource::new(move || load_data(count.get()));

// OnceResource - loads once
let once = OnceResource::new(load_initial_data());
```

## Extractors (Axum)

Access server context using extractors:

```rust
#[server(GetData)]
pub async fn get_data() -> Result<Data, ServerFnError> {
    use leptos_axum::extract;
    use axum::extract::Query;

    // Extract from request
    let Query(params): Query<MyParams> = extract().await?;

    // Access app state via context
    let pool = expect_context::<SqlitePool>();

    let data = sqlx::query_as("SELECT * FROM items")
        .fetch_all(&pool)
        .await?;

    Ok(data)
}
```

### Providing Context for Extractors

```rust
// In main.rs - provide context to server functions
let pool = SqlitePool::connect("...").await?;

let app = Router::new()
    .leptos_routes_with_context(
        &leptos_options,
        routes,
        move || provide_context(pool.clone()),
        || shell(leptos_options.clone()),
    );
```

## Using Actions for Mutations

Actions are for write operations (POST, mutations):

```rust
#[server(CreateUser)]
pub async fn create_user(name: String, email: String) -> Result<User, ServerFnError> {
    let pool = expect_context::<SqlitePool>();
    // Insert into database...
    Ok(new_user)
}

#[component]
fn CreateUserForm() -> impl IntoView {
    let action = ServerAction::<CreateUser>::new();
    let pending = action.pending();

    view! {
        <ActionForm action=action>
            <input type="text" name="name" />
            <input type="email" name="email" />
            <button type="submit" disabled=pending>
                {move || if pending.get() { "Creating..." } else { "Create" }}
            </button>
        </ActionForm>
    }
}
```

## Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found")]
    NotFound,
    #[error("DB error: {0}")]
    Database(#[from] sqlx::Error),
}

#[server(GetItem)]
pub async fn get_item(id: String) -> Result<Item, ServerFnError> {
    let item = db.find(&id).await
        .map_err(|e| ServerFnError::new(format!("DB: {}", e)))?;

    item.ok_or_else(|| ServerFnError::new("Not found"))
}
```

## DTOs (Data Transfer Objects)

Return types must implement `Serialize + Deserialize`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub id: String,
    pub name: String,
}

// Convert from entity
impl From<UserEntity> for UserDto {
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id.to_string(),
            name: entity.name,
        }
    }
}
```

## Best Practices

1. Use **`Resource`** for GET/read operations (with SSR)
2. Use **`ServerAction`** for POST/mutations
3. **Always return `Result<T, ServerFnError>`**
4. Use **DTOs** instead of database entities
5. Access state via **`expect_context::<T>()`** instead of extractors when possible
6. **Separate source and fetcher** in Resources for proper SSR hydration
