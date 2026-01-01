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

use crate::api::auth::login::try_login;
use crate::api::auth::signup::create_account;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use log::info;

pub(crate) async fn auth_api() -> Router {
    Router::new()
        .route("/signup/{uuid}/{username}/{password}/", get(create_account))
        .route("/login/{username}/{password}/", get(login))
}

async fn login(Path((username, password)): Path<(String, String)>) -> Result<StatusCode, StatusCode> { 
    let uuid = try_login(Path((username, password)))
        .await
        .ok_or(StatusCode::BAD_REQUEST)?;

    info!("Logged in as {}!", uuid);

    Ok(StatusCode::OK)
}