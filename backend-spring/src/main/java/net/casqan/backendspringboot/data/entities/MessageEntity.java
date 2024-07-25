package net.casqan.backendspringboot.data.entities;

import jakarta.persistence.*;

import java.util.UUID;

@Entity
@Table(name = "message_entity")
public class MessageEntity {
    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    @Column(name = "id", nullable = false)
    private UUID id;

    public UUID getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    @Column(name = "content", nullable = false)
    private String content;

    public String getContent() {
        return content;
    }

    public void setContent(String content) {
        this.content = content;
    }

    @Column(name = "type", nullable = false)
    private String type;

    public String getType() {
        return type;
    }

    public void setType(String type) {
        this.type = type;
    }

    @Column(name = "created_at", nullable = false)
    private String createdAt;

    public String getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(String createdAt) {
        this.createdAt = createdAt;
    }

    @Column(name = "updated_at", nullable = false)
    private String updatedAt;

    public String getUpdatedAt() {
        return updatedAt;
    }

    public void setUpdatedAt(String updatedAt) {
        this.updatedAt = updatedAt;
    }

    @ManyToOne
    @JoinColumn(name = "channel_id", referencedColumnName = "id", insertable = false, updatable = false)
    private ChannelEntity channel;

    public ChannelEntity getChannel() {
        return channel;
    }

    public void setChannel(ChannelEntity channel) {
        this.channel = channel;
    }

    @ManyToOne
    @JoinColumn(name = "sender_id", referencedColumnName = "id", insertable = false, updatable = false)
    private ProfileEntity sender;

    public ProfileEntity getSender() {
        return sender;
    }

    public void setSender(ProfileEntity sender) {
        this.sender = sender;
    }

    public MessageEntity(ChannelEntity channel, ProfileEntity sender) {
        this.channel = channel;
        this.sender = sender;
    }
}