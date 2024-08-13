package net.casqan.backendspringboot.services;

import net.casqan.backendspringboot.data.entities.MessageEntity;
import net.casqan.backendspringboot.data.mapper.MessageMapper;
import net.casqan.backendspringboot.data.models.Channel;
import net.casqan.backendspringboot.data.models.Message;
import net.casqan.backendspringboot.data.repositories.ChannelEntityRepository;
import net.casqan.backendspringboot.data.repositories.MessageEntityRepository;
import net.casqan.backendspringboot.data.repositories.ProfileEntityRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.time.LocalDateTime;
import java.util.Collection;
import java.util.UUID;

@Service
public class MessageService {

    private final ChannelEntityRepository channelEntityRepository;
    private final MessageEntityRepository messageEntityRepository;
    private final ProfileEntityRepository profileEntityRepository;

    @Autowired
    public MessageService(MessageEntityRepository messageEntityRepository,
                          ProfileEntityRepository profileEntityRepository,
                          ChannelEntityRepository channelEntityRepository){
        this.messageEntityRepository = messageEntityRepository;
        this.profileEntityRepository = profileEntityRepository;
        this.channelEntityRepository = channelEntityRepository;
    }

    public Collection<Message> getMessagesByChannel(UUID channelId){
        return messageEntityRepository.findAllByChannelId(channelId).stream()
                .map(MessageMapper::mapToMessage)
                .toList();
    }

    public Message createMessage(UUID channelId, UUID senderId){
        var sender = profileEntityRepository.findById(senderId).orElse(null);
        if (sender == null) throw new IllegalArgumentException("Sender not found");

        var channelEntity = channelEntityRepository.findById(channelId).orElse(null);
        if (channelEntity == null) throw new IllegalArgumentException("Channel not found");


        var saved = messageEntityRepository.save(new MessageEntity(channelEntity,sender));

        messageEntityRepository.save(saved);

        return MessageMapper.mapToMessage(saved);
    }

    public Message updateMessage(UUID channelId, UUID messageId, Message message) {
        var sender = profileEntityRepository.findById(message.getSenderId()).orElse(null);
        if (sender == null) throw new IllegalArgumentException("Sender not found");

        var channelEntity = channelEntityRepository.findById(channelId).orElse(null);
        if (channelEntity == null) throw new IllegalArgumentException("Channel not found");

        var messageEntity = messageEntityRepository.findById(messageId).orElse(null);
        if (messageEntity == null) throw new IllegalArgumentException("Message not found");

        messageEntity.setContent(message.getContent());

        messageEntityRepository.save(messageEntity);

        return MessageMapper.mapToMessage(messageEntity);
    }

    public void deleteMessage(UUID channelId, UUID messageId) {
        messageEntityRepository.deleteById(messageId);
    }
}
