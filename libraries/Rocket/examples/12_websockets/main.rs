// # WebSockets

// This example demonstrates how to use WebSockets with Rocket. It uses the `rocket_ws` crate, which provides a WebSocket implementation for Rocket.

#[macro_use]
extern crate rocket;

use rocket_ws::{Stream, WebSocket};

#[get("/echo")]
fn echo_compose(ws: WebSocket) -> Stream!['static] {
    ws.stream(|io| io)
}

// As with `async` streams, `rocket_ws` also supports using generator syntax for WebSocket messages:

#[get("/echo-generator")]
fn echo_stream(ws: WebSocket) -> Stream!['static] {
    Stream! { ws =>
        for await message in ws {
            yield message?;
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![echo_compose, echo_stream])
}
