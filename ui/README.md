# Dicebag UI
This is the UI project powering Dicebag.

## Getting Started
This project uses [Trunk](https://trunkrs.dev) for development.

```
trunk serve
```

### Getting the Schema
This project uses the [Juniper `graphql-client` CLI tool](https://github.com/graphql-rust/graphql-client/tree/master/graphql_client_cli) to fetch the server schema.

While running the `server` project locally, you can use the following command to update the schema at any time:

`graphql-client introspect-schema --output src/schema.json http://127.0.0.1:8000`

### Generating Types
This project uses the [Juniper `graphql-client` CLI tool](https://github.com/graphql-rust/graphql-client/tree/master/graphql_client_cli)

`graphql-client generate --schema-path src/schema.json src/queries.graphql`

## Technologies Used
- [WebAssembly](https://webassembly.org/)
- [Yew](https://yew.rs)
- [Sass](https://sass-lang.com/)
