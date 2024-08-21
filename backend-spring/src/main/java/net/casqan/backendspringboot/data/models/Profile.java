package net.casqan.backendspringboot.data.models;

import java.util.UUID;

public class Profile {
    private UUID id;
    private String username;
    private String email;
    private String sudoCode;
    private String createdAt;
    private String updatedAt;

public Profile(UUID id, String username, String email, String sudoCode, String createdAt, String updatedAt) {
        this.id = id;
        this.username = username;
        this.email = email;
        this.sudoCode = sudoCode;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    public UUID getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public String getUsername() {
        return username;
    }

    public void setUsername(String username) {
        this.username = username;
    }

    public String getEmail() {
        return email;
    }

    public void setEmail(String email) {
        this.email = email;
    }

    public String getSudoCode() {
        return sudoCode;
    }

    public void setSudoCode(String sudoCode) {
        this.sudoCode = sudoCode;
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
}


