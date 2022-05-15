use actix_web::{HttpResponse, web};
use chrono::{NaiveDateTime, Utc};
use diesel::dsl::Nullable;
use diesel::sql_types::{Timestamptz};
use serde::{Serialize, Serializer, Deserialize};
use crate::db;
use crate::errors::ApiError;
use crate::schema::links;
use diesel::prelude::*;
use uuid::{Uuid, UuidVersion};

#[derive(Serialize, Deserialize, Queryable, Insertable)]
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

#[derive(Deserialize, AsChangeset)]
#[table_name = "links"]
pub struct UpdateLinkPayload {
    pub name: String,
    pub target_url: String,
    pub shortened_url: String,
}

impl Link {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let connection = db::get_connection()?;
        let results = links::table.load::<Link>(&connection)?;
        Ok(results)
    }

    pub async fn create(link_payload: CreateLinkPayload) -> Result<Self, ApiError> {
        info!("creating link payload");

        let connection = db::get_connection().unwrap();
        let link = Link::from(link_payload);

        // According to actix docs the v1 implementation of diesel isn't async
        // This basically is an async wrapper of a sync method
        let created_link = web::block(move || {
            diesel::insert_into(links::table)
                .values(link)
                .get_result(&connection)
                .expect("Failed to insert new link record into DB")
        }).await.map_err(|e| {
           ApiError {
               status_code: 500,
               message: "Failed to persist link".to_string()
           }
        }).unwrap();

        Ok(created_link)
    }

    pub async fn update(record_id: Uuid, payload: UpdateLinkPayload) -> Result<Self, ApiError> {
        info!("updated link record");

        let connection = db::get_connection().unwrap();
        let updated_link = web::block(
            move ||
                diesel::update(links::table)
                    .filter(links::id.eq(record_id))
                    .set(payload)
                    .get_result(&connection)
                    .expect("Failed to update link record on DB")
        ).await.map_err(|e| {
            ApiError {
                status_code: 500,
                message: "Failed to update link".to_string()
            }
        }).unwrap();
        Ok(updated_link)
    }
}

impl From<CreateLinkPayload> for Link {
    fn from(payload: CreateLinkPayload) -> Self {
        Link {
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
