#![feature(async_await)]
use warp::Filter;
use futures::{join, future};

async fn start() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("hello, {}", name));

    let client = redis
        ::Client
        ::open("redis://127.0.0.1")
        .unwrap();

    let redisConnection = client.get_async_connection();


    let httpServe = warp::serve(hello)
        .run(([127, 0, 0, 1], 3030));
    let wait_both = join!(redisConnection, httpServe);
    return wait_both;
}

#[tokio::main]
async fn main() {
    let (a, b) = start().await;
    println!("OK! got a: {}", a);
}