use leptos::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct RouletteUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub frequency: i32,
}
#[server]
pub async fn get_user(uuid_s: String) -> Result<User, ServerFnError> {
    use crate::db::ssr::get_db;
    let uuid = Uuid::parse_str(uuid_s.as_str());
    let uuid = match uuid {
        Ok(uuid) => uuid,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string()))
    };
    sqlx::query_as::<_, User>("SELECT * FROM \"user\" WHERE uuid = $1")
        .bind(uuid)
        .fetch_one(get_db())
        .await.map_err(|error| ServerFnError::ServerError(error.to_string()))
}

#[server]
pub async fn get_roulette_users(uuid_s: String) -> Result<Vec<RouletteUser>, ServerFnError> {
    use crate::db::ssr::get_db;
    let uuid = Uuid::parse_str(uuid_s.as_str());
    let uuid = match uuid {
        Ok(uuid) => uuid,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string()))
    };
    sqlx::query_as::<_, RouletteUser>(
        r#"SELECT "user".uuid, "user".first_name, "user".last_name, "user".email, "user".phone, roulette_user.frequency
                                   FROM "user"
                                   JOIN roulette_user ON "user".uuid = roulette_user.user_uuid
                                   WHERE roulette_user.roulette_uuid = $1"#)
        .bind(uuid)
        .fetch_all(get_db())
        .await.map_err(|error| ServerFnError::ServerError(error.to_string()))
}