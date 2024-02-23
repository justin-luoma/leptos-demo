use crate::error_template::{AppError, ErrorTemplate};
use crate::models::user::User;
use gloo_storage::{LocalStorage, Storage};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
                        path=""
                        view=move || {
                            view! {
                                <Show when=move || user.get().is_some()>
                                    <HomePage user=user.get().unwrap()/>
                                </Show>
                                <Show when=move || user.get().is_none()>
                                    <div>
                                        <p>No user</p>
                                        <p>{refreshed.get()}</p>
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
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(user: User) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>

        <div>
            <p>{user.uuid}</p>
        </div>

        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
