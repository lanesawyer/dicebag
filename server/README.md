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

Make sure to include the `ROCKET_ADDRESS` `.env` variable of `0.0.0.0` so the API will be exposed.

## Technologies Used
- [Rocket](https://rocket.rs/)
- [Juniper](https://github.com/graphql-rust/juniper)
