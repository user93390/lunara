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
use std::sync::LazyLock;
use tokio::sync::Mutex;

pub static DB_HOST: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));
pub static DB_PORT: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));
pub static DB_NAME: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));
pub static DB_USER: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));
pub static DB_PASSWORD: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));

pub async fn database() -> Database {
    let host = DB_HOST.lock().await.clone();
    let port = DB_PORT.lock().await.clone();
    let name = DB_NAME.lock().await.clone();
    let user = DB_USER.lock().await.clone();
    let password = DB_PASSWORD.lock().await.clone();

    let connection_string = if !password.is_empty() {
        format!("host={} port={} dbname={} user={} password={}", host, port, name, user, password)
    } else {
        format!("host={} port={} dbname={} user={}", host, port, name, user)
    };

    Database::connect(&connection_string)
        .await
        .expect("Failed to connect to database")
}
