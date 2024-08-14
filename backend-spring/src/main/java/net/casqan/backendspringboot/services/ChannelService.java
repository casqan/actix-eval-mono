package net.casqan.backendspringboot.services;

import net.casqan.backendspringboot.data.entities.ChannelEntity;
import net.casqan.backendspringboot.data.mapper.ChannelMapper;
import net.casqan.backendspringboot.data.models.Channel;
import net.casqan.backendspringboot.data.repositories.ChannelEntityRepository;
import net.casqan.backendspringboot.data.repositories.ProfileEntityRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.time.LocalDateTime;
import java.util.Collection;
import java.util.UUID;

@Service
public class ChannelService {

    private final ChannelEntityRepository channelEntityRepository;
    private final ProfileEntityRepository profileEntityRepository;

    @Autowired
    public ChannelService(ChannelEntityRepository channelEntityRepository,
                          ProfileEntityRepository profileEntityRepository) {
        this.channelEntityRepository = channelEntityRepository;
        this.profileEntityRepository = profileEntityRepository;
    }

    public Collection<Channel> getChannels() {
        return channelEntityRepository.findAll().stream().map(ChannelMapper::ChannelEntityToChannel).toList();
    }

    public Channel createChannel(UUID ownerId) {

        var owner = profileEntityRepository.findById(ownerId).orElse(null);
        if (owner == null) throw new IllegalArgumentException("Owner not found");

        var channelEntity = new ChannelEntity(owner);
        channelEntity.setOwner(owner);
        channelEntity.setDescription("Default description");
        channelEntity.setName("Default name");
        channelEntity.setCreatedAt(LocalDateTime.now().toString());
        channelEntity.setUpdatedAt(LocalDateTime.now().toString());

        var saved = channelEntityRepository.save(channelEntity);
        return ChannelMapper.ChannelEntityToChannel(saved);
    }

    public Channel updateChannel(Channel channel) {
        var channelEntity = channelEntityRepository.findById(channel.getId()).orElse(null);
        if (channelEntity == null) throw new IllegalArgumentException("Channel not found");

        channelEntity.setDescription(channel.getDescription());
        channelEntity.setIsPublic(channel.getIsPublic());
        channelEntity.setName(channel.getName());
        channelEntity.setUpdatedAt(LocalDateTime.now().toString());

        var saved = channelEntityRepository.save(channelEntity);
        return ChannelMapper.ChannelEntityToChannel(saved);
    }

    public Collection<Channel> getChannelByOwnerId(UUID ownerId) {
        return channelEntityRepository.findAllByOwnerId(ownerId).stream().map(ChannelMapper::ChannelEntityToChannel).toList();
    }

    public Channel getChannelById(UUID id) {

        var channelEntity = channelEntityRepository.findById(id).orElse(null);
        if (channelEntity == null) return null;
        return ChannelMapper.ChannelEntityToChannel(channelEntity);
    }

    public void deleteChannel(UUID id) {
        channelEntityRepository.deleteById(id);
    }

}
