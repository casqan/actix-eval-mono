
package net.casqan.backendspringboot.data.models;

import java.util.List;
import java.util.UUID;


public class Channel {

    private String createdAt;
    private String description;
    private UUID id;
    private boolean isPublic;
    private List<UUID> members;
    private List<UUID> messages;
    private String name;
    private UUID ownerId;
    private String updatedAt;

    public Channel() {
    }

    public Channel(String createdAt, String description, UUID id, boolean isPublic, List<UUID> members, List<UUID> messages, String name, UUID ownerId, String updatedAt) {
        this.createdAt = createdAt;
        this.description = description;
        this.id = id;
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

    public void setCreatedAt(String createdAt) {
        this.createdAt = createdAt;
    }

    public String getDescription() {
        return description;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    public UUID getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public boolean getIsPublic() {
        return isPublic;
    }

    public void setIsPublic(boolean isPublic) {
        this.isPublic = isPublic;
    }

    public List<UUID> getMembers() {
        return members;
    }

    public void setMembers(List<UUID> members) {
        this.members = members;
    }

    public List<UUID> getMessages() {
        return messages;
    }

    public void setMessages(List<UUID> messages) {
        this.messages = messages;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public UUID getOwnerId() {
        return ownerId;
    }

    public void setOwnerId(UUID ownerId) {
        this.ownerId = ownerId;
    }

    public String getUpdatedAt() {
        return updatedAt;
    }

    public void setUpdatedAt(String updatedAt) {
        this.updatedAt = updatedAt;
    }

}
