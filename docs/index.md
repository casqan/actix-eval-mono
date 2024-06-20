# ACTIX Evaluation

## Introduction
The aim of this Project is to evaluate the differences between actix-web and springboot for APIs & Websockets. 
The evaluation will be based on the following criteria:

- Performance
- Ease of use
- Documentation
- Community support
- Learning curve
- Testing

## The Project

The Project will involve a simple Messaging API that will be implemented in both actix-web and springboot. 
A full list of all endpoints can be found here: [API Endpoints](./api/endpoints)

- GET /api/v1/channels
- POST /api/v1/channels
- GET /api/v1/channels/{id}
- PUT /api/v1/channels/{id}
- DELETE /api/v1/channels/{id}

- GET /api/v1/channels/{id}/messages
- POST /api/v1/channels/{id}/messages
- GET /api/v1/channels/{id}/messages/{id}
- PUT /api/v1/channels/{id}/messages/{id}
- DELETE /api/v1/channels/{id}/messages/{id}

The API will also have a websocket endpoint that will allow clients to connect and send messages.

- ws://localhost:8080/{userid}

## Dependencies
Rust:
- actix-web
- actix-rt
- actix-web-actors
- serde
- diesel
- amqrs
- tokio

Java:
- Spring Boot
- Spring Web
- Spring Data JPA
- Spring Security
- Spring Websocket
- Spring Boot Starter Test
- RabbitMQ

Frontend:
- Vue
- Vuetify
- Axios
- Vue-Router
- Vuex

