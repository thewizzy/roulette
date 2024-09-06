use leptos::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Roulette {
    pub uuid: Uuid,
    pub admin_code: Uuid,
    pub name: String,
    pub email: String,
    pub description: String,
}


#[server]
pub async fn get_roulette(uuid_s: String) -> Result<Roulette, ServerFnError> {
    use crate::db::ssr::get_db;
    let uuid = Uuid::parse_str(uuid_s.as_str());
    let uuid = match uuid {
        Ok(uuid) => uuid,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string()))
    };
    sqlx::query_as::<_, Roulette>("SELECT * FROM roulette WHERE uuid = $1")
        .bind(uuid)
        .fetch_one(get_db())
        .await.map_err(|error| ServerFnError::ServerError(error.to_string()))
}

#[server]
pub async fn get_roulette_admin(uuid_s: String) -> Result<Roulette, ServerFnError> {
    use crate::db::ssr::get_db;
    let uuid = Uuid::parse_str(uuid_s.as_str());
    let uuid = match uuid {
        Ok(uuid) => uuid,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string()))
    };
    sqlx::query_as::<_, Roulette>("SELECT * FROM roulette WHERE admin_code = $1")
        .bind(uuid)
        .fetch_one(get_db())
        .await.map_err(|error| ServerFnError::ServerError(error.to_string()))
}
