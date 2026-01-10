use maud::html;
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::any().map(|| html! { h1 { "Hello, world!" } });
    warp::serve(hello).run(([127, 0, 0, 1], 8000)).await;
}
