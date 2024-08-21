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
    @Column(name = "content", nullable = false)
    private String content;
    @Column(name = "type", nullable = false)
    private String type;
    @Column(name = "created_at", nullable = false)
    private String createdAt;
    @Column(name = "updated_at", nullable = false)
    private String updatedAt;
    @ManyToOne
    @JoinColumn(name = "channel_id", referencedColumnName = "id")
    private ChannelEntity channel;
    @ManyToOne
    @JoinColumn(name = "sender_id", referencedColumnName = "id")
    private ProfileEntity sender;

    public MessageEntity(ChannelEntity channel, ProfileEntity sender) {
        this.channel = channel;
        this.sender = sender;
        this.content = "";
        this.type = "text";
        this.createdAt = java.time.LocalDateTime.now().toString();
        this.updatedAt = java.time.LocalDateTime.now().toString();
    }

    public MessageEntity() {
        
    }

    public UUID getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public String getContent() {
        return content;
    }

    public void setContent(String content) {
        this.content = content;
    }

    public String getType() {
        return type;
    }

    public void setType(String type) {
        this.type = type;
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

    public ChannelEntity getChannel() {
        return channel;
    }

    public void setChannel(ChannelEntity channel) {
        this.channel = channel;
    }

    public ProfileEntity getSender() {
        return sender;
    }

    public void setSender(ProfileEntity sender) {
        this.sender = sender;
    }
}