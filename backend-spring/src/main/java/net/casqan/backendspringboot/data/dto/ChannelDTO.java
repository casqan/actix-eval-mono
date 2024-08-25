package net.casqan.backendspringboot.data.dto;

import java.util.List;
import java.util.UUID;

public class ChannelDTO {

    private String createdAt;
    private String description;
    private boolean isPublic;

    private List<UUID> members;
    private List<UUID> messages;
    private String name;
    private UUID ownerId;
    private String updatedAt;

    public ChannelDTO(String createdAt, String description, boolean isPublic, List<UUID> members, List<UUID> messages, String name, UUID ownerId, String updatedAt) {
        this.createdAt = createdAt;
        this.description = description;
        this.isPublic = isPublic;
        this.members = members;
        this.messages = messages;
        this.name = name;
        this.ownerId = ownerId;
        this.updatedAt = updatedAt;
    }

    public String getCreatedAt() {
        return createdAt;
    }

    public String getDescription() {
        return description;
    }

    public boolean isPublic() {
        return isPublic;
    }

    public List<UUID> getMembers() {
        return members;
    }

    public List<UUID> getMessages() {
        return messages;
    }

    public String getName() {
        return name;
    }

    public UUID getOwnerId() {
        return ownerId;
    }

    public String getUpdatedAt() {
        return updatedAt;
    }
}
