use maud::{DOCTYPE, Markup, html};
use warp::Filter;

fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8"
        title { (page_title)}
    }
}

fn page(title: &str) -> Markup {
    html! {
        (header(title))
        h1 { (title) }
    }
}

#[tokio::main]
async fn main() {
    let index_page = warp::any().map(|| page("brd"));
    warp::serve(index_page).run(([127, 0, 0, 1], 8000)).await;
}
