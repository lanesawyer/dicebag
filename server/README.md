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

## Technologies Used
- [Rocket](https://rocket.rs/)
- [Juniper](https://github.com/graphql-rust/juniper)
