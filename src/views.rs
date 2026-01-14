use super::models::{Board, Post};
use maud::{DOCTYPE, Markup, html};

pub fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (page_title) }
        link rel="stylesheet" href="/assets/css/style.css";
    }
}

pub fn page(title: &str) -> Markup {
    html! {
        (header(title))
        h1 { (title) }
    }
}

pub fn boards_partial(boards: Vec<Board>) -> Markup {
    html! {
            @for board in boards {
                a href="/b/g" { (board.name) }
            }
    }
}

pub fn index_page(boards: Markup) -> Markup {
    html! {
        (page("brd"))
        p { "welcome to brd" }

        h2 { "boards" }
        (boards)

        h2 { "description" }
        p { "A simple imageboard site that supports private walled-garden communication" }
    }
}

pub fn board_partial(board: &Board) -> Markup {
    html! {
        (page(&board.name))
    }
}

pub fn posts_partial(posts: Vec<Post>) -> Markup {
    html! {
        @for post in (posts.iter().rev()) {
            pre {
                "id: " (post.id) "\n"
                "body: " (post.body) "\n"
            }
        }
    }
}

pub fn board_page(board: Markup, posts: Markup) -> Markup {
    html! {
        (board)
        a href="/" { "<= Go back" }

        br; br;
        form method="post" {
            textarea name="body" rows="6" cols="36" {}
            br;
            input name="submit" type="submit" value="submit";
        }
        br;

        (posts)
    }
}
