# `Rocket` 🚀

https://rocket.rs/

Rocket is a web framework for Rust. It provides primitives to build web servers and applications in Rust.

> [!NOTE]
> Rocket v0.5 uses the `tokio` runtime.

---

## Install

To install Rocket, you can add it as a dependency in your `Cargo.toml` file:

```toml
...

[dependencies]
rocket = "0.5.1"
```
or you can use Cargo to add it directly:

```bash
cargo add rocket
```

## Usage

See [`00_hello_world`](examples/00_hello_world/main.rs) for a simple example of using Rocket to create a web server that responds with "Hello, world!" when accessed at the root URL (`/`).

---

