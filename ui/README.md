# Dicebag UI
This is the UI project powering Dicebag.

## Getting Started
This project uses [Trunk](https://trunkrs.dev) for development.

```
trunk serve
```

### Environment Variables
The `dicebag` frontend uses the following environment variables:
- `API_URL`: Defaults to `http://127.0.0.1:8080/graphql` if not provided

I would like to use `dotenv` like the `server` project does, but Trunk requires the environment variables to be included in the build at build time. Unfortunately, this doesn't work with CI/CD because there is no .env file. I would prefer to have a 
default as I do with the `option_env!` macro, but `dotenv` doesn't support this at the time of this writing.

**When building `dicebag` for production use, you must include the `API_URL` in the build command.**

### Getting the Schema
This project uses the [Juniper `graphql-client` CLI tool](https://github.com/graphql-rust/graphql-client/tree/master/graphql_client_cli) to fetch the server schema.

While running the `server` project locally, you can use the following command to update the schema at any time:

`graphql-client introspect-schema --output src/graphql/schema.json http://127.0.0.1:8000`

### Generating Types
This project uses the [Juniper `graphql-client` CLI tool](https://github.com/graphql-rust/graphql-client/tree/master/graphql_client_cli)

`graphql-client generate --schema-path src/graphql/schema.json src/graphql/queries.graphql`

## Technologies Used
- [WebAssembly](https://webassembly.org/)
- [Yew](https://yew.rs)
- [Sass](https://sass-lang.com/)
- [Open Iconic](https://useiconic.com/open/)

## Attributions
### Music
- [Night Owl by Broke For Free](https://freemusicarchive.org/music/Broke_For_Free/Directionless_EP/Broke_For_Free_-_Directionless_EP_-_01_Night_Owl)
