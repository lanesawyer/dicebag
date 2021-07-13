# Dicebag Server 
This is the server project powering Dicebag.

## Getting Started
To run the server, run the following command:

```
cargo run
```

## Docker
To build the server, run the following command from the `server` folder:

```
docker build -t dicebag-server .
```

To run the built image, run the following command:

```
docker run -p 8000:8000 -it --name dicebag-server dicebag-server
```

## Docker Compose
To run both the server and a Postgres database, run the following command from the `server` folder:

```
docker compose up
```

The database data will be saved in the `postgres-data` Docker volume between runs.

**Current Errors:** When running `docker compose`, the `ROCKET_DATABASE` environment variable is not working. I think it's something with the network connection between the two containers but I haven't been able to figure it out yet.

## Technologies Used
- [Rocket](https://rocket.rs/)
- [Juniper](https://github.com/graphql-rust/juniper)
