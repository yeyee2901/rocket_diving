# Rust + Rocket Web Framework for Microservice
Interesting toy I found while studying as a backend engineer. If you ever built a web service using Golang, and wanted to another statically typed language for web services, you can take a look at my aproach in it.

# Project Structure
```bash

.
├── Cargo.toml          # only specify workspace members
├── Cargo.lock          # project metadata lock
├── README.md
├── rustfmt.toml        # formatter rule
├── first_gateway       # package: first service
│   ├── Cargo.toml
│   └── src
│       ├── config.rs
│       ├── handlers
│       │   ├── index.rs
│       │   └── mod.rs
│       ├── lib.rs
│       └── main.rs
└── second_gateway      # package: second service
    ├── Cargo.toml
    └── src
        ├── lib.rs
        └── main.rs

5 directories, 13 files
```

# Adding New Service to The Project Workspace
- Each time you want to add a new service, run:
```bash
cargo new <second_gateway>

```
- If you've successfully added it, then you will get something like this:
```bash
warning: compiling this new package may not work due to invalid workspace configuration

current package believes it's in a workspace when it's not:
current:   /home/yeyee/DATA-SDA4/yeyee/Documents/Rust/rocket_diving/second_gateway/Cargo.toml
workspace: /home/yeyee/DATA-SDA4/yeyee/Documents/Rust/rocket_diving/Cargo.toml

this may be fixable by adding `second_gateway` to the `workspace.members` array of the manifest located at: /home/yeyee/DATA-SDA4/yeyee/Documents/Rust/rocket_diving/Cargo.toml
Alternatively, to keep it out of the workspace, add the package to the `workspace.exclude` array, or add an empty `[workspace]` table to the package's manifest.
     Created library `second_gateway` package
```
- Now, add your new service to the list of workspace in the root `Cargo.toml` file:
```toml
[workspace]
members = [
    "first_gateway",
    "second_gateway"
]
```
- Next, work on the `Cargo.toml` file for your new service
```toml
[package]
name = "second_gateway"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
name = "second_gateway"

[[bin]]
path = "src/main.rs"
name = "second_gateway"

# specify your dependencies ...
[dependencies]
```
- Finally, to fetch & check the metadata, run:
```bash
cargo check --all-targets
```

# Wanna Try?
- Running the binaries
```bash
cargo run --bin first_gateway

# cargo run --bin <your_service_binary_crate>
```

- Building the binaries:
```bash
cargo build --release --bin <service_name>

# build all
cargo build --release --all
```

- Not getting linting errors / completions? This maybe because the cargo is not done fetching the project metadata, or you lost / corrupted your `Cargo.lock` file. If this happens, run:
```bash
cargo check --all-targets
```
