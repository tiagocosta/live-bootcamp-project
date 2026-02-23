use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{ErrorResponse, app_state::AppState, domain::{error::AuthAPIError, user::User}};

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
    let email = request.email.trim();
    let password = request.password.trim();

    if email.is_empty() || !email.contains("@") || password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }

    // Create a new `User` instance using data in the `request`
    let user = User::new(email.to_string(), password.to_string(), request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    if let Ok(_) = user_store.get_user(email) {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    // Add `user` to the `user_store`. Simply unwrap the returned `Result` enum type for now.
    if let Err(_) = user_store.add_user(user) {
        return Err(AuthAPIError::UnexpectedError);
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}