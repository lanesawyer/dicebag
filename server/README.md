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

### Known Issues
- The code isn't updating on restart so I've been removing the image and re-running `docker compose up` to get the latest. I know there's a way to get around it but I haven't gotten to fixing it yet since I don't do a ton of API updates right now.

## Technologies Used
- [Rocket](https://rocket.rs/)
- [Juniper](https://github.com/graphql-rust/juniper)
- [Diesel](https://diesel.rs/)
