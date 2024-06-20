# Channel

A channel is a way to group and organize messages. Channels can be public or private. Public channels are open to everyone in the workspace, while private channels are only accessible to invited members.

```json
{
  "id": "UUID",
  "name": "string",
  "description": "string",
  "isPublic": "boolean",
  "ownerId": "UUID",
  "members": ["UUID"],
  "messages": ["UUID"],
  "createdAt": "string",
  "updatedAt": "string"
}
```