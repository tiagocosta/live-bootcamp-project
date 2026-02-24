use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{email::Email, error::AuthAPIError, password::Password, user::User}};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // let email = request.email.trim();
    // let password = request.password.trim();

    let email = Email::parse(request.email);
    let password = Password::parse(request.password);

    if email.is_err() || password.is_err() {
        return Err(AuthAPIError::InvalidCredentials);
    }

    let mut user_store = state.user_store.write().await;

    let email = email.unwrap();
    let password = password.unwrap();

    if let Ok(_) = user_store.get_user(&email).await {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    // Create a new `User` instance using data in the `request`
    let user = User::new(email, password, request.requires_2fa);

    // Add `user` to the `user_store`. Simply unwrap the returned `Result` enum type for now.
    if let Err(_) = user_store.add_user(user).await {
        return Err(AuthAPIError::UnexpectedError);
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}