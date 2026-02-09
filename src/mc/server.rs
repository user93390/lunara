/*
Copyright 2026 seasnail1

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

	http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

*/
use crate::mc::plugin::Plugin;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum ServerBrand {
	Vanilla,
	Paper,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct BuildInfo {
	pub(crate) version: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct MinecraftServer {
	brand: ServerBrand,
	build: BuildInfo,
	name: Option<String>,
	plugins: Option<Vec<Plugin>>,
}

impl MinecraftServer {
	pub(crate) fn new() -> Self {
		Self {
			brand: ServerBrand::Vanilla,
			build: BuildInfo {
				version: String::from("NaN"),
			},
			name: None,
			plugins: None,
		}
	}

	pub(crate) fn with_version(&mut self, version: BuildInfo) -> &mut Self {
		self.build = version;
		self
	}

	pub(crate) fn with_name(&mut self, name: Option<String>) -> &mut Self {
		self.name = name;
		self
	}

	pub(crate) fn with_brand(&mut self, brand: ServerBrand) -> &mut Self {
		self.brand = brand;
		self
	}

	pub(crate) fn build_info(&self) -> &BuildInfo {
		&self.build
	}

	pub(crate) fn plugins(&self) -> Option<&Vec<Plugin>> {
		self.plugins.as_ref()
	}

	pub(crate) fn name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	pub(crate) fn brand(&self) -> &ServerBrand {
		&self.brand
	}

	pub(crate) fn version(&self) -> &str {
		&self.build.version
	}

	pub(crate) fn build(&self) -> &MinecraftServer {
		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_server_has_defaults() {
		let server = MinecraftServer::new();

		assert!(matches!(server.brand(), ServerBrand::Vanilla));
		assert_eq!(server.version(), "NaN");
		assert!(server.name().is_none());
		assert!(server.plugins().is_none());
	}

	#[test]
	fn server_builder_with_name() {
		let mut server = MinecraftServer::new();
		server.with_name(Some(String::from("TestServer")));

		assert_eq!(server.name(), Some(&String::from("TestServer")));
	}

	#[test]
	fn server_builder_with_brand() {
		let mut server = MinecraftServer::new();
		server.with_brand(ServerBrand::Paper);

		assert!(matches!(server.brand(), ServerBrand::Paper));
	}

	#[test]
	fn server_builder_with_version() {
		let mut server = MinecraftServer::new();
		server.with_version(BuildInfo {
			version: String::from("1.20.4"),
		});

		assert_eq!(server.version(), "1.20.4");
	}

	#[test]
	fn server_builder_chain() {
		let mut server = MinecraftServer::new();
		server
			.with_name(Some(String::from("ChainTest")))
			.with_brand(ServerBrand::Paper)
			.with_version(BuildInfo {
				version: String::from("1.19.2"),
			});

		assert_eq!(server.name(), Some(&String::from("ChainTest")));
		assert!(matches!(server.brand(), ServerBrand::Paper));
		assert_eq!(server.version(), "1.19.2");
	}

	#[test]
	fn server_build_returns_self() {
		let server = MinecraftServer::new();
		let built = server.build();

		assert_eq!(built.version(), server.version());
	}

	#[test]
	fn build_info_version() {
		let mut server = MinecraftServer::new();
		server.with_version(BuildInfo {
			version: String::from("1.21"),
		});

		assert_eq!(server.build_info().version, "1.21");
	}

	#[test]
	fn server_serialize_deserialize() {
		let mut server = MinecraftServer::new();
		server
			.with_name(Some(String::from("SerdeServer")))
			.with_brand(ServerBrand::Paper)
			.with_version(BuildInfo {
				version: String::from("1.20.1"),
			});

		let json = serde_json::to_string(&server).unwrap();
		let deserialized: MinecraftServer = serde_json::from_str(&json).unwrap();

		assert_eq!(deserialized.name(), server.name());
		assert_eq!(deserialized.version(), server.version());
	}

	#[test]
	fn server_brand_debug() {
		let vanilla = ServerBrand::Vanilla;
		let paper = ServerBrand::Paper;

		assert_eq!(format!("{:?}", vanilla), "Vanilla");
		assert_eq!(format!("{:?}", paper), "Paper");
	}
}
