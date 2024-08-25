use ::entity::{channel_entity, utils::time::get_current_time};
use sea_orm::*;
use uuid::Uuid;

pub struct ChannelService {}

impl ChannelService{
    pub async fn create_channel(
        db: &DbConn,
    ) -> Result<channel_entity::Model, DbErr> {
        println!("Creating new Entity");
        let new_entity = channel_entity::ActiveModel::new();

        println!("Saving Entity");
        let result = new_entity.insert(db).await;
        if result.is_err() {
            println!("Error saving to Database");
            match result.as_ref().err() {
                None => {
                    println!("unknown error");
                    return result;
                },
                Some(e) => {
                    println!("{:#?}", e);
                    return result;
                }
            };
        } else{
            return result;
        }
    }

    pub async fn get_all_channels(db: &DbConn) -> Result<Vec<channel_entity::Model>, DbErr>{
        channel_entity::Entity::find().all(db).await
    }

    pub async fn get_channel(db: &DbConn, id: Uuid) -> Result<Option<channel_entity::Model>, DbErr>{
        channel_entity::Entity::find_by_id(id).one(db).await
    }

    pub async fn update_channel(db: &DbConn, data: channel_entity::Model) -> Result<channel_entity::Model, DbErr> {
        let existing_channel = Self::get_channel(db,data.id).await?;
        if existing_channel.is_none() {
            return Result::Err(DbErr::RecordNotFound("The Requested channel, could not be updated".to_owned()));
        }
        let mut model: channel_entity::ActiveModel = existing_channel.unwrap().into();

        model.updated_at = Set(get_current_time().to_rfc3339());
        model.name = Set(data.name);
        model.description = Set(data.description);
        return model.update(db).await;
    }

    pub async fn delete_channel(db: &DbConn, id: Uuid) -> Result<DeleteResult, DbErr>{
        channel_entity::Entity::delete_by_id(id).exec(db).await
    }
}