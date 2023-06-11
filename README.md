# Axum server template

## Features implemented

- Axum
  - Query and Path get examples
  - login with cookies
  - REST CRUD with in-memory mock model
- Manual Error handling without 3rd party crates
  - Errors respond with request IDs
  - Debug and Display variants for server and client
- Spellcheck with cspell
- GraphQl with async-graphql
- Request logs
  - For every request one log
  - Include req_id, error, logged in user
  - For both REST and GraphQL
- CI with github actions
- SurrealDb
  - in memory, no setup
  - service structure callable from both rest and GraphQl
  - no-db workaround in both rest routes and GraphQl for testing and debugging
    without a working SurrealDb instance

## TODOs:

- rename routes_tickets to no_db
- reimplement with new DB service
