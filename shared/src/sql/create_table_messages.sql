CREATE TABLE messages{
    id UUID PRIMARY KEY,
    content TEXT NOT NULL,
    senderId UUID NOT NULL,
    channelId UUID NOT NULL,
    createdAt TIMESTAMP NOT NULL,
    updatedAt TIMESTAMP NOT NULL
}