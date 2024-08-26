# ACTIX Evaluation

## Introduction
The aim of this Project is to evaluate the differences between actix-web and springboot for APIs.
The evaluation will be based on the following criteria:

- Performance
- Ease of use
- Documentation

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

## Dependencies

***THIS IS A TEMPORARY LIST, CHANGES ARE TO BE EXPECTED***

Rust:
- actix-web
- actix-web-actors
- serde
- SeaORM

Java:
- Spring Boot
- Spring Web
- Spring Data JPA
- Spring Security
- Spring Boot Starter Test

## Testing

The API will be tested using the following tools:

load tests:
- k6 
