package net.casqan.backendspringboot.data.entities;

import jakarta.persistence.*;

import java.util.List;
import java.util.UUID;

@Entity
@Table(name = "channel_entity")
public class ChannelEntity {
    @Id
    @GeneratedValue(strategy = GenerationType.AUTO)
    @Column(name = "id", nullable = false)
    private UUID id;

    @Column(name = "name", nullable = false)
    private String name;

    @Column(name = "description", nullable = false)
    private String description;

    @Column(name = "is_public", nullable = false)
    private boolean isPublic;

    @Column(name = "created_at", nullable = false)
    private String createdAt;

    @Column(name = "updated_at", nullable = false)
    private String updatedAt;

    @ManyToOne
    @JoinColumn(name = "owner_id", referencedColumnName = "id")
    private ProfileEntity owner;

    @ManyToMany
    @JoinTable(
            name = "channel_members",
            joinColumns = @JoinColumn(name = "channel_id"),
            inverseJoinColumns = @JoinColumn(name = "profile_id")
    )
    private List<ProfileEntity> members;

    @OneToMany(cascade = CascadeType.ALL, mappedBy = "channel")
    private List<MessageEntity> messages;

    public ChannelEntity(ProfileEntity owner) {
        this.owner = owner;
        this.description = "Default description";
        this.name = "Default name";
        this.createdAt = java.time.LocalDateTime.now().toString();
        this.updatedAt = java.time.LocalDateTime.now().toString();
        this.members = List.of(owner);
        this.isPublic = false;
        this.messages = List.of();
    }

    public ChannelEntity() {
    }

    public UUID getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String getDescription() {
        return description;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    public boolean getIsPublic() {
        return isPublic;
    }

    public void setIsPublic(boolean isPublic) {
        this.isPublic = isPublic;
    }

    public String getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(String createdAt) {
        this.createdAt = createdAt;
    }

    public String getUpdatedAt() {
        return updatedAt;
    }

    public void setUpdatedAt(String updatedAt) {
        this.updatedAt = updatedAt;
    }

    public ProfileEntity getOwner() {
        return owner;
    }

    public void setOwner(ProfileEntity owner) {
        this.owner = owner;
    }

    public List<ProfileEntity> getMembers() {
        return members;
    }

    public void setMembers(List<ProfileEntity> members) {
        this.members = members;
    }

    public List<MessageEntity> getMessages() {
        return messages;
    }

    public void setMessages(List<MessageEntity> messages) {
        this.messages = messages;
    }

}