use std::error::Error;
use std::str::from_utf8;

use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Deserialize, Serialize)]

struct Config {
	config_path: String,
	keyring_key: [u8; 32],
}

impl Config {
	pub(crate) fn with_key(&mut self, key: [u8; 32]) -> &mut Self {
		self.keyring_key = key;
		self
	}

	pub(crate) fn def_file(&mut self, path: String) -> &mut Self {
		self.config_path = path;
		self
	}

	pub(crate) async fn get_from_toml(&self) -> Result<Config, Box<dyn Error + Send + Sync>> {
		let path = Path::new(&self.config_path);

		let mut file = File::open(path).await?;

		let mut contents = vec![];
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
