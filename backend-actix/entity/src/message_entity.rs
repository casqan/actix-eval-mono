//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize)]
#[sea_orm(table_name = "message_entity")]
pub struct Model {
    pub channel_id: Option<Uuid>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub created_at: String,
    pub r#type: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::channel_entity::Entity",
        from = "Column::ChannelId",
        to = "super::channel_entity::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ChannelEntity,
    #[sea_orm(
        belongs_to = "super::profile_entity::Entity",
        from = "Column::SenderId",
        to = "super::profile_entity::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ProfileEntity,
}

impl Related<super::channel_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChannelEntity.def()
    }
}

impl Related<super::profile_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProfileEntity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
