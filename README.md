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
- Graphql with async-graphql

## TODOs:

- fix - log errors from gql
  - somehow store in the request or ctx - to pick up with the logger mw
  - or separate error logs from request logs
- CI with github actions
- DB with maybe surrealDb
