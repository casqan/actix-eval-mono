package net.casqan.backendspringboot.data.repositories;

import net.casqan.backendspringboot.data.entities.MessageEntity;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.Collection;
import java.util.List;
import java.util.UUID;

public interface MessageEntityRepository extends JpaRepository<MessageEntity, UUID> {
    Collection<MessageEntity> findAllByChannelId(UUID channelId);
}