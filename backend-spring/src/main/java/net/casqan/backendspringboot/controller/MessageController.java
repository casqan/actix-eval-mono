package net.casqan.backendspringboot.controller;

import net.casqan.backendspringboot.data.models.Channel;
import net.casqan.backendspringboot.data.models.Message;
import net.casqan.backendspringboot.services.ChannelService;
import net.casqan.backendspringboot.services.MessageService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.net.URI;
import java.util.Collection;
import java.util.UUID;

import static net.casqan.backendspringboot.data.Constants.CHANNEL_BASE_URL;
import static net.casqan.backendspringboot.data.Constants.MESSAGE_BASE_URL;

@RestController
@RequestMapping(MESSAGE_BASE_URL)
public class MessageController {

    private final ChannelService channelService;
    private final MessageService messageService;

    @Autowired
    public MessageController(ChannelService channelService,
                             MessageService messageService) {
        this.channelService = channelService;
        this.messageService = messageService;
    }

    @GetMapping()
    public ResponseEntity<Collection<Message>> getMessage(@PathVariable UUID channelId) {
        if (channelService.getChannelById(channelId) == null) return ResponseEntity.notFound().build();
        return ResponseEntity.ok().body(messageService.getMessagesByChannel(channelId));
    }

    @PostMapping()
    public ResponseEntity<Message> postMessage(@PathVariable UUID channelId) {
        var res = messageService.createMessage(channelId, new UUID(0, 0));
        var uri = URI.create(CHANNEL_BASE_URL + "/" + channelId + "/" + res.getId());
        return ResponseEntity.created(uri).body(res);
    }

    @PutMapping("/{id}")
    public ResponseEntity<Message> updateMessage(@PathVariable UUID channelId, @PathVariable UUID id, @RequestBody Message message) {
        var res = messageService.updateMessage(channelId, id, message);
        return ResponseEntity.ok().body(res);
    }

    @DeleteMapping("/{id}")
    public ResponseEntity<String> deleteMessage(@PathVariable UUID channelId, @PathVariable UUID id) {
        messageService.deleteMessage(channelId, id);
        return ResponseEntity.ok().body("Message with id: " + id + " deleted!");
    }
}
