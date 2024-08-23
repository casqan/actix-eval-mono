use ::entity::{message_entity, utils::time::get_current_time};
use sea_orm::*;
use uuid::Uuid;

pub struct MessageService {}

impl MessageService {
    pub async fn create_message(
        db: &DbConn,
        channel_id: Uuid,
        sender_id: Uuid
    ) -> Result<message_entity::Model, DbErr> {
        println!("Creating new Entity");
        let mut new_entity = message_entity::ActiveModel::new();
        new_entity.channel_id = Set(Option::Some(channel_id));
        new_entity.sender_id = Set(Option::Some(sender_id));
        
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

    pub async fn get_all_messagess(db: &DbConn, channel_id: Uuid) -> Result<Vec<message_entity::Model>, DbErr>{
        message_entity::Entity::find()
            .filter(message_entity::Column::ChannelId.eq(channel_id))
            .all(db).await
    }

    pub async fn get_message(db: &DbConn, id: Uuid, channel_id: Uuid) -> Result<Option<message_entity::Model>, DbErr>{
        message_entity::Entity::find_by_id(id)
            .filter(message_entity::Column::ChannelId.eq(channel_id))
            .one(db).await
    }

    pub async fn update_message(db: &DbConn, channel_id: Uuid, data: message_entity::Model) -> Result<message_entity::Model, DbErr> {
        let existing_message = Self::get_message(db,data.id, channel_id).await?;
        if existing_message.is_none() {
            return Result::Err(DbErr::RecordNotFound("The Requested message, could not be updated".to_owned()));
        }
        let mut model: message_entity::ActiveModel = existing_message.unwrap().into();

        model.updated_at = Set(get_current_time().to_rfc3339());
        model.content = Set(data.content);
        model.r#type = Set(data.r#type);
        return model.update(db).await;
    }

    pub async fn delete_message(db: &DbConn, id: Uuid) -> Result<DeleteResult, DbErr>{
        message_entity::Entity::delete_by_id(id).exec(db).await
    }
}