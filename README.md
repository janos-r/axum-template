# Axum server template

## Features implemented

- Axum
  - Query and Path get example - [routes_hello.rs](./src/web/routes_hello.rs)
  - login with cookies
  - middleware Response mapping
  - REST CRUD with in-memory mock model
- Manual Error handling without 3rd party crates
- Spellcheck with cspell

## TODOs:

- log every request - for analysis and error-by-uuid
  - compose uuid from the beginning on a layer and save in extension
  - extract it in an error
  - add the error to the extensions
  - compose a full request log at the last layer
  - extract the client error into it

- graphql with async-graphql
- CI with github actions
- DB with maybe surrealDb
