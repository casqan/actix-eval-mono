package net.casqan.backendspringboot.data.repositories;

import net.casqan.backendspringboot.data.entities.ChannelEntity;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.Collection;
import java.util.UUID;

public interface ChannelEntityRepository extends JpaRepository<ChannelEntity, UUID> {
    public Collection<ChannelEntity> findAllByOwnerId(UUID ownerId);

}