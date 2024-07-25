package net.casqan.backendspringboot.data.repositories;

import net.casqan.backendspringboot.data.entities.ProfileEntity;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.UUID;

public interface ProfileEntityRepository extends JpaRepository<ProfileEntity, UUID> {
}