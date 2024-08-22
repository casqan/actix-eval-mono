use ::entity::{channel_entity, channel_entity::Entity as channel};
use sea_orm::*;
use sea_orm::sqlx::types::uuid;
use uuid::uuid;

pub struct ChannelService {}

impl ChannelService{
    pub async fn create_channel(
        db: &DbConn,
        data_channel: channel_entity::Model,
    ) -> Result<channel_entity::ActiveModel, DbErr> {

        channel_entity::ActiveModel{
            id: uuid!("aadd6eff-a188-4618-94c1-7608e2216bf9").into_active_value(),
            is_public: data_channel.is_public.into_active_value(),
            owner_id: data_channel.owner_id.into_active_value(),
            created_at: data_channel.created_at.into_active_value(),
            description: data_channel.description.into_active_value(),
            name: data_channel.name.into_active_value(),
            updated_at: data_channel.updated_at.into_active_value()
        }.save(db).await
    }
}