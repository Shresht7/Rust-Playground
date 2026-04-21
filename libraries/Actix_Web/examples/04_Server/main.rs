// # Server

// The `HttpServer` type is responsible for serving HTTP requests.

// `HttpServer` accepts an application factory as a parameter, and the application factory must have `Send` + `Sync` boundaries.

// To start the web server it must first be bound to a network socket. Use `HttpServer::bind()` with a socket address tuple or string such as
// `("127.0.0.1", 8080)` or `"127.0.0.1:8080"`. This will fail if the socket is being used by another process.

// After the bind is successful, use `HttpServer::run()` to return a `Server` instance. The server must be `await`ed or `spawn`ed to start processing
// requests and will run until it receives a shutdown signal (such as, by default, a `ctrl+c`)

// ## Multi-Threading

// `HttpServer` automatically starts a number of HTTP workers, by default this number is equal to the number of physical CPUs in the system. This can be overriden with the `HttpServer::workers()` method.
// Once the workers are created, they each receive a separate application instance to handle requests. Application state is not shared between threads
// and handlers are free to manipulate their copy of the state with no concurrency concerns.

// Application state does not need to be `Send` and `Sync`, but application factories must be `Send` + `Sync`.

// To share state between worker threads, use an `Arc`/`Data`.

// Since each worker thread processes its requests sequentially, handlers which block the current thread will cause the current worker to stop processing new requests.
// For this reason, any long, non-cpu bound operation (e.g. I/O, database operations etc) should be expressed as futures or asynchronous functions.
// Async handlers get executed concurrently by worker threads, and thus don't block execution.
