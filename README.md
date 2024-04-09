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

To use routes other than `/hello`, login with (5min expiration):

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

TODO: Future Axum 0.8 should implement async traits. So after it comes out, I
will try to fix the Context extractor.

### Examples

GQL create:

```graphql
# post on `localhost:8080/`
mutation {
  tickets {
    createTicket(ctInput: { title: "First Ticket" }) {
      id
      title
      creator
    }
  }
}
```

```json
{
  "data": {
    "tickets": {
      "createTicket": {
        "id": "6ki4tip4sx33gz622dc0",
        "title": "First Ticket",
        "creator": "joe@example.com"
      }
    }
  }
}
```

GQL client error:

```graphql
mutation {
  tickets {
    deleteTicket(id: "12345"){
      id
      title
      creator
    }
  }
}
```

Displays only the user-facing error text with the request ID to lookup in logs

```json
{
  "data": null,
  "errors": [
    {
      "message": "No result for id 12345",
      "locations": [
        {
          "line": 3,
          "column": 5
        }
      ],
      "path": [
        "tickets",
        "deleteTicket"
      ],
      "extensions": {
        "req_id": "bc857894-412c-4fc7-8040-ffdc1c03aaec"
      }
    }
  ]
}
```

Logs have the real error cause

```bash
->> LOGGER       - mw_req_logger:
    {"req_method":"POST","req_path":"/","user":"joe@example.com",
    "error":"SurrealDbNoResult { source: \"internal\", id: \"12345\" }",
    "timestamp":"1712652314952","req_id":"bc857894-412c-4fc7-8040-ffdc1c03aaec"}
```

Same as with REST

`delete on http://localhost:8080/api/tickets/999`

```json
{
  "error": {
    "error": "No result for id 999",
    "req_id": "82afa2ab-01ff-4c93-b004-98f86f68e9d2"
  }
}
```

```bash
->> LOGGER       - mw_req_logger:
    {"req_method":"DELETE","req_path":"/api/tickets/999","user":"joe@example.com",
    "error":"SurrealDbNoResult { source: \"internal\", id: \"999\" }",
    "timestamp":"1712654372370","req_id":"82afa2ab-01ff-4c93-b004-98f86f68e9d2"}
```
