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

extern crate log;
mod account;
mod api;
mod config;
mod database;
mod keyring;
mod routes;

use crate::{
	database::Database,
	routes::{api_route, auth_route},
};

use crate::keyring::KeyringService;

use axum::Router;

use log::{info, warn, LevelFilter};
use std::error::Error;
use std::rc::Rc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

const SERVER_ADDR: &str = "0.0.0.0";
const SERVER_PORT: u16 = 5000;

// Fallbacks or defaults
const POSTGRES_PORT_DEF: &str = "5432";
const POSTGRES_HOST_DEF: &str = "postgres_database";
const POSTGRES_NAME_DEF: &str = "postgres";
const POSTGRES_USER_DEF: &str = "postgres";
const POSTGRES_PASSWORD_DEF: &str = "postgres";

/// <p>Create an asynchronous router.</p>
/// <p>Return all routes by nesting them inside a brand new route.</p>
/// Pretty expensive fuction.
pub async fn app() -> Result<Router, Box<dyn Error + Send + Sync>> {
	// create a smart pointer just for database.
	let database_rc: Rc<Database> = Rc::new(database().await?);

	let auth_route: Router<_> = auth_route::auth_api((*database_rc).clone()).await;
	let api_route: Router<_> = api_route::user_api((*database_rc).clone()).await;

	let flutter_dir = ServeDir::new("flutter/build/web");

	Ok(Router::new()
		.nest("/auth/v1", auth_route)
		.nest("/api", api_route)
		.fallback_service(flutter_dir))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
	let mut first_time: bool = false;

	color_eyre::install()?;

	env_logger::builder()
		.format_timestamp_secs()
		.format_level(true)
		.filter_level(LevelFilter::Info)
		.filter(Some("zbus"), LevelFilter::Warn)
		.filter(Some("tracing"), LevelFilter::Warn)
		.init();

	info!("Running Lunara.");

	let keyring_service: KeyringService = KeyringService::new("Lunara");

	let key: bool = keyring_service.secret_exists("key").await;

	if !key {
		first_time = true;

		warn!("This is your first time running Lunara.");

		// gen new 128 key
		let new_key: [u8; 32] = KeyringService::generate_key_128();
		keyring_service
			.set_secret("key", &hex::encode(new_key))
			.await?;
	}

	info!("First time? {}", first_time);


	// Should we init keys?
	if first_time {
		keyring_service
			.set_secret("db.host", POSTGRES_HOST_DEF)
			.await?;
		info!("Set db.host: {}", POSTGRES_HOST_DEF);
		keyring_service
			.set_secret("db.port", POSTGRES_PORT_DEF)
			.await?;
		info!("Set db.port: {}", POSTGRES_PORT_DEF);
		keyring_service
			.set_secret("db.name", POSTGRES_NAME_DEF)
			.await?;
		info!("Set db.name: {}", POSTGRES_NAME_DEF);
		keyring_service
			.set_secret("db.user", POSTGRES_USER_DEF)
			.await?;
		info!("Set db.user: {}", POSTGRES_USER_DEF);
		keyring_service
			.set_secret("db.password", POSTGRES_PASSWORD_DEF)
			.await?;
		info!("Set db.password: {}", POSTGRES_PASSWORD_DEF);
	}

	// Wait for app
	let app: Router = app().await?;

	let string_addr: String = format!("{}:{}", SERVER_ADDR, SERVER_PORT);
	let alt_addr: String = format!("localhost:{}", SERVER_PORT);

	info!(
		"Done! Now serving {} \n Alternatively: {}",
		string_addr, alt_addr
	);

	let listener: TcpListener = TcpListener::bind(string_addr).await?;

	axum::serve(listener, app).await?;

	info!("Shutting down...");

	Ok(())
}

/// <p>Provides a database.</p>
/// Note: This instance of this database IS asynchronous.
pub async fn database() -> Result<Database, Box<dyn Error + Send + Sync>> {
	Ok(database::database().await?)
}
