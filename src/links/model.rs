use chrono::{NaiveDateTime, Utc};
use diesel::dsl::Nullable;
use diesel::sql_types::{Timestamptz};
use serde::{Serialize, Serializer, Deserialize};
use crate::db;
use crate::errors::ApiError;
use crate::schema::links;
use diesel::prelude::*;
use diesel::types::FromSql;
use uuid::{Uuid, UuidVersion};

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

#[derive(Deserialize)]
pub struct CreateLinkPayload {
    pub name: String,
    pub target_url: String
}

impl Link {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let connection = db::get_connection()?;
        let results = links::table.load::<Link>(&connection)?;
        Ok(results)
    }

    pub fn create(link_payload: CreateLinkPayload) -> Result<Self, ApiError> {
        let connection = db::get_connection()?;

        let link = Link::from(link_payload);
        let created_link = diesel::insert_into(links::table).values(link).get_result(&connection)?;

        Ok(created_link)
    }
}

impl From<CreateLinkPayload> for Link {
    fn from(payload: CreateLinkPayload) -> Self {
        return Link {
            id: Uuid::new_v4(),
            name: payload.name,
            target_url: payload.target_url,
            shortened_url: format!("https://re.di.rect/{}", Uuid::new_v4()),
            created_at: Utc::now().naive_utc(),
            deleted_at: None,
            version: 0
        }
    }
}

