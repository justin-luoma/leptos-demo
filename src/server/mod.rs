use leptos::server;
use leptos::ServerFnError;

#[server(Test)]
pub async fn login_user() -> Result<String, ServerFnError> {
    Ok(format!("hello world"))
}