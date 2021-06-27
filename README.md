# Rust experimental HTTP API 

Experimental project to test the Rust language and the 'warp' and 'tokio' libraries

```rust
// GET / => 200 OK with body "It works!"
let default = warp::path::end().map(|| "It works!");

// GET /hello/warp => 200 OK with body "Hello, warp!"
let hello = warp::path!("hello" / String)
    .map(|name| format!("Hello, {}!", name));

// GET /bye/nurio => 200 OK with body "Bye, nurio!"
let bye = warp::path!("bye" / String)
    .map(|name| format!("Bye, {}!", name));

// Map filters
let routes = default.or(hello).or(bye);

// Start the http server
warp::serve(routes)
    .run(([127, 0, 0, 1], 3030))
    .await;
```
