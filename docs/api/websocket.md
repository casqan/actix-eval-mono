# Websockets

## Endpoints

`/users/{id}/ws` - Connect to a user's websocket.

Automatically subscribes to all channels the user is a member of.

## Messages
Publish Subscribe Channels:

- `channel:{id}` - Channel's message channel.

(RabbitMQ)

## Events