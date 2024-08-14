package net.casqan.backendspringboot.controller;

import net.casqan.backendspringboot.data.dto.ChannelDTO;
import net.casqan.backendspringboot.data.mapper.ChannelMapper;
import net.casqan.backendspringboot.data.models.Channel;
import net.casqan.backendspringboot.services.ChannelService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.net.URI;
import java.util.Collection;
import java.util.UUID;

import static net.casqan.backendspringboot.data.Constants.CHANNEL_BASE_URL;

@RestController
@RequestMapping(CHANNEL_BASE_URL)
public class ChannelController {

    private final ChannelService channelService;

    @Autowired
    public ChannelController(ChannelService channelService) {
        this.channelService = channelService;
    }

    @GetMapping
    public ResponseEntity<Collection<Channel>> getChannels() {
        return ResponseEntity.ok().body(channelService.getChannels());
    }

    @PostMapping("")
    public ResponseEntity<Channel> createChannel() {
        var channel = channelService.createChannel(new UUID(0, 0));
        URI location = URI.create(CHANNEL_BASE_URL + channel.getId().toString());
        return ResponseEntity.created(location).body(channel);
    }

    @PutMapping("{id}")
    public ResponseEntity<Channel> updateChannel(@PathVariable UUID id, @RequestBody ChannelDTO channel) {
        if (channel.getId() == null || id != channel.getId()) return ResponseEntity.badRequest().build();
        try {
            var result = channelService.updateChannel(ChannelMapper.ChannelDTOToChannel(channel));
            return ResponseEntity.ok().body(result);
        } catch (IllegalArgumentException e) {
            return ResponseEntity.notFound().build();
        }
    }

    @DeleteMapping("{id}")
    public ResponseEntity<String> deleteChannel(@PathVariable UUID id) {
        channelService.deleteChannel(id);
        return ResponseEntity.ok().body("Channel with id: " + id + " deleted!");
    }
}
