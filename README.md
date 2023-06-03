# Axum server template

## Features implemented

- Axum
  - Query and Path get examples
  - login with cookies
  - REST CRUD with in-memory mock model
- Manual Error handling without 3rd party crates
  - Errors respond with request IDs
  - Debug and Display variants for server and client
- Request json logs
- Spellcheck with cspell
- graphql with async-graphql

## TODOs:

- gql
  - state - replicate rest api (model_controller)
  - auth - restriction, ctx
- CI with github actions
- DB with maybe surrealDb
