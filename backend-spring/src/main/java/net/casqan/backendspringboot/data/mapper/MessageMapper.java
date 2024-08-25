package net.casqan.backendspringboot.data.mapper;

import net.casqan.backendspringboot.data.entities.MessageEntity;
import net.casqan.backendspringboot.data.models.Message;

public final class MessageMapper {
    public static Message mapToMessage(MessageEntity messageEntity){
        return new Message(messageEntity.getId(), messageEntity.getContent(),
                messageEntity.getType(),
                messageEntity.getSender().getId(),
                messageEntity.getChannel().getId(),
                messageEntity.getCreatedAt(),
                messageEntity.getUpdatedAt());
    }
}
