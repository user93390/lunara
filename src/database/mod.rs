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

pub mod db;

pub use db::Database;

use crate::keyring::KeyringService;
use log::info;
use once_cell::sync::Lazy;
use std::error::Error;
use std::sync::Mutex;

static CONNECTION_STRING: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

pub async fn database() -> Result<Database, Box<dyn Error + Send + Sync>> {
	if let Some(conn_str) = CONNECTION_STRING.lock().ok().and_then(|g| g.clone()) {
		info!("Using cached database connection string");
		return Ok(Database::connect(&conn_str).await?);
	}

	info!("Loading database configuration from keyring");

	let keyring_service = KeyringService::new("Lunara");

	let host = keyring_service.get_secret("db.host").await?;
	let port = keyring_service.get_secret("db.port").await?;
	let name = keyring_service.get_secret("db.name").await?;
	let user = keyring_service.get_secret("db.user").await?;
	let password = keyring_service.get_secret("db.password").await?;

	let connection_string = format!(
		"host={} port={} dbname={} user={} password={}",
		host, port, name, user, password
	);

	info!("Connecting to database at {}:{}", host, port);

	if let Ok(mut cached) = CONNECTION_STRING.lock() {
		*cached = Some(connection_string.clone());
	}

	Ok(Database::connect(&connection_string).await?)
}
