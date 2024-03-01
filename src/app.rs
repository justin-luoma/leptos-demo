use crate::error_template::{AppError, ErrorTemplate};
use crate::User;
use crate::server::*;
use gloo_storage::{LocalStorage, Storage};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::env::*;
use base64::Engine;
use serde_json::Value;


#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    // let (user, set_user, _) = use_local_storage::<User, JsonCodec>("user");
    let refreshed = create_rw_signal(false);
    let user = create_rw_signal(Option::None::<User>);

    create_effect(move |_| {
        if let Ok(u) = LocalStorage::get("user") {
            log::info!("refreshed user");
            user.set(Some(u));
            refreshed.set(true);
        }
    });

    create_effect(move |_| {
        if !refreshed.get() {
            if let Some(user) = user.get() {
                LocalStorage::set("user", user).expect("LocalStorage::set user");
                log::info!("saved user to local storage");
            }
        }
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/demo.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route
                        path="/"
                        view=move || {
                            view! {
                                <Show when=move || user.get().is_some()>
                                    <HomePage user1=user.get().unwrap()/>
                                </Show>
                                <Show when=move || user.get().is_none()>
                                    <div>
                                        <p>No user</p>
                                        <p>{refreshed.get()}</p>
                                        <a href=SUPABASE_URL.to_owned() + SUPABASE_GOOGLE_LOGIN + SUPABASE_REDIRECT>"Login here"</a>
                                        <button on:click=move |_| {
                                            log::info!("Setting user");
                                            user.set(
                                                Some(
                                                    User::new(
                                                        String::new(),
                                                        String::from("uuid"),
                                                        String::new(),
                                                        String::new(),
                                                    ),
                                                ),
                                            );
                                        }>Set User</button>
                                    </div>
                                </Show>
                            }
                        }
                    />
                    <Route
                        path="/redirect"
                        view={move || {
                            let new_user = url_hash_to_user(use_location().hash.get());
                            match new_user {
                                Some(new_user) => {
                                    user.set(Some(new_user));
                                    view! { <Redirect path="/" /> }
                                }
                                None => view! { 
                                    // <Redirect path=SUPABASE_URL.to_owned() + SUPABASE_GOOGLE_LOGIN + SUPABASE_REDIRECT />
                                    <div></div>
                                 }.into_view(),
                            }
                        }}
                    />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(user1: User) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let (user, set_user) = create_signal(Option::None::<String>);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>

        <div>
            <p>{user1.uuid}</p>
        </div>

        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

pub fn url_hash_to_user(mut url_hash: String) -> Option<User> {
    if url_hash.is_empty() {
        return None;
    }
    let mut access_token = None;
    let mut refresh_token = None;
    url_hash.remove(0);
    for q in url_hash.split("&") {
        let Some((key, value)) = q.split_once("=") else {
            break;
        };
        if key == "access_token" {
            access_token = Some(value.to_owned());
        } else if key == "refresh_token" {
            refresh_token = Some(value.to_owned());
        }
    }
    let uuid_email = access_token
        .as_ref()
        .map(|access_token| access_token_to_uuid_email(access_token.as_str()))
        .flatten();
    match (uuid_email, access_token, refresh_token) {
        (Some((uuid, email)), Some(access_token), Some(refresh_token)) => {
            Some(User { uuid, email, access_token, refresh_token })
        }
        _ => None,
    }
}

pub fn access_token_to_uuid_email(token: &str) -> Option<(String, String)> {
    if token.is_empty() {
        return None;
    }
    let output_size = base64::decoded_len_estimate(token.len());
    let mut payload_buffer = Vec::<u8>::with_capacity(output_size);
    let payload_base64 = token.split(".").nth(1)?;
    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode_vec(payload_base64, &mut payload_buffer)
        .ok()?;
    let payload_json: Value = serde_json::from_slice(&payload_buffer[..]).ok()?;
    let uuid = payload_json.get("sub")?.as_str()?.to_owned();
    let email = payload_json.get("email")?.as_str()?.to_owned();
    Some((uuid, email))
}