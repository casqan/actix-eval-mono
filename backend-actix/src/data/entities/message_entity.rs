use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct MessageEntity{
    id: Uuid,
    content: String,
    channel_id: Uuid,
    created_at: i32,
    updated_at: i32
}