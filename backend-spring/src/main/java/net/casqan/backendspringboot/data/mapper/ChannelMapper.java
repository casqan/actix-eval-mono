package net.casqan.backendspringboot.data.mapper;

import net.casqan.backendspringboot.data.dto.ChannelDTO;
import net.casqan.backendspringboot.data.entities.ChannelEntity;
import net.casqan.backendspringboot.data.entities.MessageEntity;
import net.casqan.backendspringboot.data.entities.ProfileEntity;
import net.casqan.backendspringboot.data.models.Channel;

import java.util.UUID;

public final class ChannelMapper {
    public static Channel ChannelEntityToChannel(ChannelEntity channelEntity) {
        return new Channel(
                channelEntity.getCreatedAt(),
                channelEntity.getDescription(),
                channelEntity.getId(),
                channelEntity.getIsPublic(),
                channelEntity.getMembers().stream().map(ProfileEntity::getId).toList(),
                channelEntity.getMessages().stream().map(MessageEntity::getId).toList(),
                channelEntity.getName(),
                channelEntity.getOwner() != null ? channelEntity.getOwner().getId() : null,
                channelEntity.getUpdatedAt());
    }

    public static Channel ChannelDTOToChannel(ChannelDTO channelDTO) {
        return new Channel(
                channelDTO.getCreatedAt(),
                channelDTO.getDescription(),
                channelDTO.getId(),
                channelDTO.isPublic(),
                channelDTO.getMembers(),
                channelDTO.getMessages(),
                channelDTO.getName(),
                channelDTO.getOwnerId(),
                channelDTO.getUpdatedAt());
    }

    public static ChannelDTO ChannelToChannelDTO(Channel channel) {
        return new ChannelDTO(
                channel.getCreatedAt(),
                channel.getDescription(),
                channel.getIsPublic(),
                channel.getId(),
                channel.getMembers(),
                channel.getMessages(),
                channel.getName(),
                channel.getOwnerId(),
                channel.getUpdatedAt());
    }
}
