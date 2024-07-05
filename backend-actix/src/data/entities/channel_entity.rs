use sea_orm::FromQueryResult;
use sea_query::{ColumnDef, Iden, Table};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow, FromQueryResult)]
pub struct ChannelEntity{
    id : Uuid,
    name : String,
    description : String,
    is_public : bool,
    owner_id : Uuid,
    created_at : i32,
    updated_at : i32
}

#[derive(Iden)]
enum ChannelColumns {
    Channels,
    Id,
    Name,
    Description,
    IsPublic,
    OwnerId,
    CreatedAt,
    UpdatedAt
}

async fn create_table(){
    let sql = Table::create()
        .table(ChannelColumns::Channels)
        .if_not_exists()
        .col(ColumnDef::new(ChannelColumns::Id)
            .uuid()
            .not_null()
            .generated((), false)
            .primary_key())
}