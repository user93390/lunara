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

use crate::database::Database;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub(crate) async fn user_api(database: Database) -> Router {
    Router::new()
        .route("/users", get(users))
        .with_state(Arc::new(database.clone()))
        .route("/users/search/{uuid}", get(search_user))
        .with_state(Arc::new(database))
}

#[axum::debug_handler]
async fn users(State(db): State<Arc<Database>>) -> Result<Json<HashMap<Uuid, String>>, (StatusCode, String)> {
    let mut users: HashMap<Uuid, String> = HashMap::new();

    let rows = db
        .select("accounts", &["uid", "username"], None, &[])
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for row in rows {
        let uid: Uuid = row.get(0);
        let username: String = row.get(1);

        users.insert(uid, username);
    }

    Ok(Json(users))
}

#[axum::debug_handler(state = Arc<Database>)]
async fn search_user(
    Path(uuid): Path<Uuid>,
    State(db): State<Arc<Database>>,
) -> Result<Json<HashMap<Uuid, String>>, (StatusCode, String)> {
    let mut users: HashMap<Uuid, String> = HashMap::new();

    let rows = db
        .select("accounts", &["uid", "username"], Some("uid = $1"), &[&uuid])
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for row in rows {
        let uid: Uuid = row.get(0);
        let username: String = row.get(1);

        if uuid == uid {
            users.insert(uid, username);
        }
    }

    Ok(Json(users))
}
