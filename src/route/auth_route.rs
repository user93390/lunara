/*
 * Copyright 2025 seasnail1
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::api::authentication::Authentication;
use crate::api::authentication::login::LoginAuth;
use crate::api::authentication::signup::SignupAuth;
use crate::database::Database;
use crate::entity::accounts::{Column, Entity};
use axum::Router;
use axum::extract::Path;
use axum::response::Response;
use axum::routing::get;
use base64::engine::general_purpose;
use base64::{Engine, alphabet, engine};
use log::{info, warn};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::sync::Arc;
use uuid::Uuid;

use axum::body::Body;
use axum::http::StatusCode;
use axum_cookie::{CookieLayer, CookieManager};
use axum_cookie::cookie::Cookie;

pub(crate) async fn auth_api(db: Database) -> Router {
	Router::new()
		.route("/signup/{uuid}/{username}/{password}", get(signup))
		.route("/login/{username}/{password}", get(login))
		.with_state(Arc::new(db))
		.layer(CookieLayer::default())
}

async fn signup(
	axum::extract::State(db): axum::extract::State<Arc<Database>>,
	Path((uuid_b64, username_b64, password_b64)): Path<(String, String, String)>,
) -> Response {
	let Ok(username) = decode_b64_string(&username_b64) else {
		return response(StatusCode::BAD_REQUEST, "Invalid username encoding.");
	};

	let Ok(password) = decode_b64(&password_b64) else {
		return response(StatusCode::BAD_REQUEST, "Invalid password encoding.");
	};

	let Ok(uuid) = decode_b64_uuid(&uuid_b64) else {
		return response(StatusCode::BAD_REQUEST, "Invalid uuid encoding.");
	};

	let signup = SignupAuth {
		uuid,
		password,
		nickname: username,
		db,
	};

	match signup.await_signup(signup.clone()).await {
		Ok(status) => {
			if status.is_success() {
				response(StatusCode::OK, "Signed up!")
			} else {
				response(StatusCode::BAD_REQUEST, "Bad request")
			}
		}
		Err(e) => {
			warn!("Signup failed: {}", e);
			response(StatusCode::INTERNAL_SERVER_ERROR, "Signup failed.")
		}
	}
}

async fn login(
	manager: CookieManager,
	axum::extract::State(db): axum::extract::State<Arc<Database>>,
	Path((uuid_b64, password_b64)): Path<(String, String)>,
) -> Response {
	let Ok(password_bytes) = decode_b64(&password_b64) else {
		return response(StatusCode::BAD_REQUEST, "Invalid password encoding.");
	};

	let Ok(uuid) = decode_b64_uuid(&uuid_b64) else {
		return response(StatusCode::BAD_REQUEST, "Invalid uuid encoding.");
	};

	let login = LoginAuth {
		uuid,
		password: password_bytes,
		db: db.clone(),
	};

	info!("Authenticating for {}", login.uuid);

	let account = match Entity::find()
		.filter(Column::Uid.eq(login.uuid))
		.one(db.conn())
		.await
	{
		Ok(Some(acc)) => acc,
		Ok(None) => {
			warn!("Account not found for UUID: {}", login.uuid);
			return response(StatusCode::UNAUTHORIZED, "Authentication failed.");
		}
		Err(e) => {
			warn!("Database error during account lookup: {}", e);
			return response(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error.");
		}
	};

	let Ok(password_str) = String::from_utf8(login.password.clone()) else {
		warn!("Invalid UTF-8 in password for UUID: {}", login.uuid);
		return response(StatusCode::BAD_REQUEST, "Invalid password format.");
	};

	if account.password.eq(&password_str) {
		manager.add(Cookie::new("username", account.username.clone()));

		info!("Authorized as {}!", &account.username);
		return response(StatusCode::ACCEPTED, "Logged in!");
	}

	warn!("Bad credentials.");
	response(StatusCode::UNAUTHORIZED, "Authentication failed.")
}

fn response(status: StatusCode, msg: &'static str) -> Response {
	Response::builder()
		.status(status)
		.body(Body::from(msg))
		.unwrap_or_else(|_| Response::new(Body::from(msg)))
}

fn decode_b64(input: &str) -> Result<Vec<u8>, ()> {
	engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
		.decode(input)
		.map_err(|_| ())
}

fn decode_b64_string(input: &str) -> Result<String, ()> {
	let bytes = decode_b64(input)?;
	String::from_utf8(bytes).map_err(|_| ())
}

fn decode_b64_uuid(input: &str) -> Result<Uuid, ()> {
	let s = decode_b64_string(input)?;
	Uuid::parse_str(&s).map_err(|_| ())
}

#[cfg(test)]
mod tests {
	use super::*;
	use axum::body::Body;
	use axum::http::{Request, StatusCode};
	use axum::response::Response;
	use base64::{Engine, alphabet, engine::GeneralPurpose, engine::general_purpose};
	use tower::ServiceExt;

	fn encode_base64(input: &str) -> String {
		GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD).encode(input)
	}

	#[test]
	fn base64_encodes_fine() {
		let expected = String::from("Hello, World!");

		let encoded = String::from("SGVsbG8sIFdvcmxkIQ");
		let decoded = GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
			.decode(encoded)
			.unwrap();

		let decoded_str = String::from_utf8(decoded).unwrap();

		assert_eq!(expected, decoded_str)
	}

	#[tokio::test]
	async fn auth_api_router_has_signup_route() {
		let db = mock_database().await;

		if let Some(db) = db {
			let app = auth_api(db).await;
			let uuid = encode_base64(&Uuid::new_v4().to_string());
			let username = encode_base64("testuser");
			let password = encode_base64("testpass");

			let response: Response = app
				.oneshot(
					Request::builder()
						.uri(format!("/signup/{}/{}/{}", uuid, username, password))
						.body(Body::empty())
						.unwrap(),
				)
				.await
				.unwrap();

			assert_ne!(response.status(), StatusCode::NOT_FOUND);
		}
	}

	#[tokio::test]
	async fn auth_api_router_has_login_route() {
		let db = mock_database().await;

		if let Some(db) = db {
			let app = auth_api(db).await;
			let username = encode_base64(&Uuid::new_v4().to_string());
			let password = encode_base64("testpass");

			let response: Response = app
				.oneshot(
					Request::builder()
						.uri(format!("/login/{}/{}", username, password))
						.body(Body::empty())
						.unwrap(),
				)
				.await
				.unwrap();

			assert_ne!(response.status(), StatusCode::NOT_FOUND);
		}
	}

	#[tokio::test]
	async fn auth_api_returns_404_for_unknown_route() {
		let db = mock_database().await;

		if let Some(db) = db {
			let app = auth_api(db).await;

			let response: Response = app
				.oneshot(
					Request::builder()
						.uri("/nonexistent")
						.body(Body::empty())
						.unwrap(),
				)
				.await
				.unwrap();

			assert_eq!(response.status(), StatusCode::NOT_FOUND);
		}
	}

	async fn mock_database() -> Option<Database> {
		Database::connect("postgres://postgres:postgres@localhost:5432/lunara")
			.await
			.ok()
	}
}
