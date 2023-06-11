# database

An interactive interface to the OSF supabase database

## Development

### Prerequisites

- Rust
  - Install [Rust 1.70](https://www.rust-lang.org/tools/install)
- Shuttle
  - Install [Shuttle 0.18.0](https://docs.shuttle.rs/introduction/installation)

### Setup

- Create a fork and clone the [repository](https://github.com/Manipal-OSF/database)
- Open the cloned repository in your editor of choice and access the terminal
- Move to the `api` directory - `cd api`
- Create a new file - `Secrets.toml` and populate it as per `.env-example`
  - Note - only the variables marked **DEV** and the **SECRET** variable are required for dev
- Run the project - `cargo shuttle run`
- Access the API using something like Postman or by running the frontend locally.
