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
use std::collections::HashMap;

use crate::config::Config;
use crate::keyring::KeyringService;

use axum::Router;

use log::{info, warn, LevelFilter};
use once_cell::sync::Lazy;
use std::error::Error;
use std::rc::Rc;
use tokio::net::TcpListener;
use tokio::sync::{Mutex, MutexGuard};

static CONFIG: Lazy<Mutex<Option<Config>>> = Lazy::new(|| Mutex::new(None));

const SERVER_ADDR: &str = "0.0.0.0";
const SERVER_PORT: u16 = 5000;

// Fallbacks or defaults
const POSTGRES_PORT_DEF: &str = "5432";
const POSTGRES_HOST_DEF: &str = "postgres_database";
const POSTGRES_NAME_DEF: &str = "postgres";
const POSTGRES_USER_DEF: &str = "postgres";
const POSTGRES_PASSWORD_DEF: &str = "postgres";



/// <p>Create an asynchronous router.</p>
/// <p>Return all routes by nesting them inside a brand-new route.</p>
/// Pretty expensive function.
async fn app(config: Config) -> Result<Router, Box<dyn Error + Send + Sync>> {
	let conn_str: &String = config.conn_str();

	info!("{}", conn_str);

	let result: Database = database::database(conn_str).await?;

	// create pointer.
	let pnt: Rc<Database> = Rc::new(result);

	let auth_route: Router<_> = auth_route::auth_api((*pnt).clone()).await;
	let api_route: Router<_> = api_route::user_api((*pnt).clone()).await;

	Ok(
		Router::new()
		.nest("/auth/v1", auth_route)
		.nest("/api", api_route)
	)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
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

	let first_time: bool = !key;

	info!("First time? {}", first_time);

	if first_time {
		warn!("This is your first time running Lunara.");

		// gen new 128 key
		let new_key: [u8; 32] = KeyringService::generate_key_128();
		keyring_service
			.set_secret("key", &hex::encode(new_key))
			.await?;

		init_kering(&keyring_service).await?;
	}

	let host = keyring_service.get_secret("db.host").await?;
	let port = keyring_service.get_secret("db.port").await?;
	let name = keyring_service.get_secret("db.name").await?;
	let user = keyring_service.get_secret("db.user").await?;
	let password = keyring_service.get_secret("db.password").await?;

	let connection_string = format!(
		"host={} port={} dbname={} user={} password={}",
		host, port, name, user, password
	);

	let key: String = keyring_service.get_secret("key").await?;

	let mut key_hex: [u8; 32] = [0u8; 32];
	let bytes: Vec<u8> = hex::decode(key)?;
	key_hex.copy_from_slice(&bytes[..32]);

	let config: Config = Config::default()
		.with_key(key_hex)
		.with_conn_str(connection_string)
		.build();

	*CONFIG.lock().await = Some(config.clone());

	// Wait for app
	let app: Router = app(config).await?;

	let string_addr: String = format!("{}:{}", SERVER_ADDR, SERVER_PORT);
	let alt_addr: String = format!("localhost:{}", SERVER_PORT);

	info!(
		"Done! Now serving {} \n Alternatively: {}",
		string_addr, alt_addr
	);

	let listener: TcpListener = TcpListener::bind(string_addr).await?;

	axum::serve(listener, app).await?;
	Ok(())
}

async fn init_kering(keyring_service: &KeyringService) -> Result<(), Box<dyn Error + Send + Sync>> {
	let secrets = [
		("db.host", POSTGRES_HOST_DEF),
		("db.port", POSTGRES_PORT_DEF),
		("db.name", POSTGRES_NAME_DEF),
		("db.user", POSTGRES_USER_DEF),
		("db.password", POSTGRES_PASSWORD_DEF),
	];

	let hash: HashMap<&str, &str> = secrets.iter()
		.cloned()
		.collect();

	for hashmap in hash {
		keyring_service.set_secret(hashmap.0, hashmap.1).await?;
	}

	Ok(())
}

pub(crate) async fn database() -> Result<Database, Box<dyn Error + Send + Sync>> {
	let config_guard: MutexGuard<Option<Config>> = CONFIG.lock().await;
	let config: &Config = config_guard.as_ref().expect("Config not initialized");
	let conn_str: &String = config.conn_str();
	let result: Database = database::database(conn_str).await?;
	Ok(result)
}