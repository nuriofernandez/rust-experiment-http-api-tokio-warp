use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    // GET /bye/nurio => 200 OK with body "Bye, nurio!"
    let bye = warp::path!("bye" / String)
        .map(|name| format!("Bye, {}!", name));

    // Map filters
    let routes = hello.or(bye);

    // Start the http server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}