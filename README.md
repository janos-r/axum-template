<p align="center">
    <img src="assets/logo.png" />
</p>

## [📜️ Article on my web](https://radim.xyz/project/axum-template/)

---

# Axum server, Async-GraphQl, SurrealDB template

Run without any prior setup, DB is in memory:

```sh
cargo run
```

To use routes other than `/hello`, login with:

```json
// POST on localhost:8080/api/login
{ "email": "joe@example.com", "password": "123" }
```

## Features implemented

- Axum
  - Query and Path get examples
  - REST CRUD with in-memory mock model
- JWT
  - login with expiration
  - saved in cookies HttpOnly
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

Detailed description linked in article above.
