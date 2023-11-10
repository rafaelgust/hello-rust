use warp::Filter;
#[tokio::main]
async fn main() {
   simple_api().await;
}


async fn simple_api() {
    let hello = warp::path!("hello" / String)
    .map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}