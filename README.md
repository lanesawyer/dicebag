# dicebag
A collection of useful tools for running or participating in D&D.

The idea behind this is to provide a grab bag of tooling that groups can choose to use as they see fit.

This is also an experiment in writing a web app using WASM and Rust with a GraphQL-powered backend.

## Roadmap
- Dice Roller: In Progress
- Character Sheet: Not Started
- Encounter Builder: Not Started
- Campaign Manager: Not Started
- Local Storage: Not Started
- Server-side Storage: Not Started

## Getting Started
The project is split into two main sections, `core` and `cli`.

The idea behind `core` is to contain all the logic and data structures that other parts of the project will use. Then, when there's more than just a `cli`, it'll be easier to add in functionality to a different front-end.

To get started, you'll need to clone the repository and build the project.

```bash
cargo build
```

### What happened to the web UI?
This project used to have a Yew and Rocket.rs frontend and backend, but they were no longer buildable. It was an ambitious project and I want to start with the core before adding a full website. I'd rather focus on something that works over the local network anyway, since this is intended to be for in-person play enhancers.
