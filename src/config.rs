use std::error::Error;
use std::str::from_utf8;

use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Deserialize, Serialize, Clone)]

pub(crate) struct Config {
	config_path: String,
	keyring_key: [u8; 32],
	connection_string: String,
}

impl Config {
	pub(crate) fn default() -> Self {
		Self {
			config_path: "config.toml".to_string(),
			keyring_key: [0u8; 32],
			connection_string: "NaN".to_string(),
		}
	}
	
	pub(crate) fn build(self) -> Self {
		Self {
			config_path: self.config_path,
			keyring_key: self.keyring_key,
			connection_string: self.connection_string,
		}
	}

	pub(crate) fn key(&self) -> [u8; 32] {
		self.keyring_key
	}

	pub(crate) fn conn_str(&self) -> &String {
		&self.connection_string
	}

	pub(crate) fn cfg_path(&self) -> &String {
		&self.config_path
	}

	pub(crate) fn with_key(mut self, key: [u8; 32]) -> Self {
		self.keyring_key = key;
		self
	}

	pub(crate) fn with_conn_str(mut self, conn_str: String) -> Self {
		self.connection_string = conn_str;
		self
	}

	pub(crate) fn def_file(mut self, path: String) -> Self {
		self.config_path = path;
		self
	}

	pub(crate) async fn get_from_toml(&self) -> Result<Config, Box<dyn Error + Send + Sync>> {
		let path: &Path = Path::new(&self.config_path);

		let mut file: File = File::open(path).await?;

		let mut contents: Vec<u8> = vec![];
		file.read_to_end(&mut contents).await?;

		let cont: &str = from_utf8(&contents)?;
		let config: Config = toml::from_str(cont)?;
		Ok(config)
	}

	pub fn get_toml(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
		let toml = toml::to_string_pretty(&self)?;

		Ok(toml)
	}
}
