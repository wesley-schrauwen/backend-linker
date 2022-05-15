use chrono::NaiveDateTime;
use diesel::dsl::Nullable;
use diesel::sql_types::{Timestamptz};
use serde::{Serialize, Serializer, Deserialize};
use crate::db;
use crate::errors::ApiError;
use crate::schema::links;
use diesel::prelude::*;
use diesel::types::FromSql;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Link {
    pub id: Uuid,
    pub name: String,
    pub target_url: String,
    pub shortened_url: String,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub version: i32,
}

impl Link {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let connection = db::get_connection()?;

        let results = links::table.load::<Link>(&connection)?;

        Ok(results)
    }
}