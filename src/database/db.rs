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

use std::sync::Arc;
use tokio_postgres::{Client, Error, NoTls, Row};

#[derive(Clone)]
pub struct Database {
    client: Arc<Client>,
}

impl Database {
    pub async fn connect(connection_string: &str) -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub async fn execute(
        &self,
        query: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Error> {
        self.client.execute(query, params).await
    }

    pub async fn query(
        &self,
        query: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<Row>, Error> {
        self.client.query(query, params).await
    }

    pub async fn query_one(
        &self,
        query: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Row, Error> {
        self.client.query_one(query, params).await
    }

    pub async fn insert(
        &self,
        table: &str,
        columns: &[&str],
        values: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Error> {
        let placeholders: Vec<String> = (1..=values.len()).map(|i| format!("${}", i)).collect();
        let query = format!("INSERT INTO {} ({}) VALUES ({})", table, columns.join(", "), placeholders.join(", "));
        self.execute(&query, values).await
    }

    pub async fn delete(
        &self,
        table: &str,
        where_clause: &str,
        where_values: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Error> {
        let query = format!("DELETE FROM {} WHERE {}", table, where_clause);
        self.execute(&query, where_values).await
    }

    pub async fn select(
        &self,
        table: &str,
        columns: &[&str],
        where_clause: Option<&str>,
        where_values: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<Row>, Error> {
        let query = match where_clause {
            Some(clause) => format!("SELECT {} FROM {} WHERE {}", columns.join(", "), table, clause),
            None => format!("SELECT {} FROM {}", columns.join(", "), table),
        };
        self.query(&query, where_values).await
    }

    pub async fn select_one(
        &self,
        table: &str,
        columns: &[&str],
        where_clause: &str,
        where_values: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Row, Error> {
        let query = format!("SELECT {} FROM {} WHERE {}", columns.join(", "), table, where_clause);
        self.query_one(&query, where_values).await
    }

    pub async fn exists(
        &self,
        table: &str,
        where_clause: &str,
        where_values: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<bool, Error> {
        let query = format!("SELECT EXISTS(SELECT 1 FROM {} WHERE {})", table, where_clause);
        let row = self.query_one(&query, where_values).await?;
        Ok(row.get(0))
    }
}

#[macro_export]
macro_rules! params {
    ($($param:expr),* $(,)?) => {
        &[$(&$param as &(dyn tokio_postgres::types::ToSql + Sync)),*]
        as &[&(dyn tokio_postgres::types::ToSql + Sync)]
    };
}
