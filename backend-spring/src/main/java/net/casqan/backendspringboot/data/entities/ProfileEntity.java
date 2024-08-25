package net.casqan.backendspringboot.data.entities;

import jakarta.persistence.*;

import java.util.List;
import java.util.UUID;

@Entity
@Table(name = "profile_entity")
public class ProfileEntity {
    @Id
    @GeneratedValue(strategy = GenerationType.AUTO)
    @Column(name = "id", nullable = false)
    private UUID id;

    public UUID getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    @Column(name = "username", nullable = false)
    private String username;

    public String getUsername() {
        return username;
    }

    public void setUsername(String username) {
        this.username = username;
    }

    @Column(name = "email", nullable = false)
    private String email;

    public String getEmail() {
        return email;
    }

    public void setEmail(String email) {
        this.email = email;
    }

    @Column(name = "sudo_code", nullable = false)
    private String sudoCode;

    public String getSudoCode() {
        return sudoCode;
    }

    public void setSudoCode(String sudoCode) {
        this.sudoCode = sudoCode;
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

    @OneToMany(cascade = CascadeType.ALL,
            mappedBy = "owner")
    private List<ChannelEntity> ownedChannels;

    public List<ChannelEntity> getOwnedChannels() {
        return ownedChannels;
    }

    public void setOwnedChannels(List<ChannelEntity> ownedChannels) {
        this.ownedChannels = ownedChannels;
    }

    @OneToMany(
            cascade = CascadeType.ALL,
            mappedBy = "sender")
    private List<MessageEntity> sentMessages;

    public List<MessageEntity> getSentMessages() {
        return sentMessages;
    }

    public void setSentMessages(List<MessageEntity> sentMessages) {
        this.sentMessages = sentMessages;
    }
}