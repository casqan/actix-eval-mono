//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize)]
#[sea_orm(table_name = "profile_entity")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: String,
    pub email: String,
    pub sudo_code: String,
    pub updated_at: String,
    pub username: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::channel_entity::Entity")]
    ChannelEntity,
    #[sea_orm(has_many = "super::channel_members::Entity")]
    ChannelMembers,
    #[sea_orm(has_many = "super::message_entity::Entity")]
    MessageEntity,
}

impl Related<super::channel_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChannelEntity.def()
    }
}

impl Related<super::channel_members::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChannelMembers.def()
    }
}

impl Related<super::message_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MessageEntity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
