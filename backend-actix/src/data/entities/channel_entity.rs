use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct ChannelEntity{
    id : Uuid,
    name : String,
    description : String,
    is_public : bool,
    owner_id : Uuid,
    created_at : i32,
    updated_at : i32
}