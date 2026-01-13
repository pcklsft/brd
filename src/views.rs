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

pub fn threads_partial(board: &Board, posts: Vec<Post>) -> Markup {
    html! {
        @for post in posts {
            pre {
                a href={ "/b/" (board.name)  "/" (post.id)  "/" } { "id: " (post.id) "\n" }
                "body: " (post.body) "\n"
            }
        }
    }
}

pub fn board_page(board: &Board, threads_partial: Markup) -> Markup {
    html! {
        (page(&board.name))
        a href="/" { "<= Go back" }
        (threads_partial)
    }
}

pub fn thread_page(board: &Board, posts: Vec<Post>) -> Markup {
    html! {
        (page(&board.name))
        a href={ "/b/" (board.name) } { "<= Go back" }
        @for post in posts {
            p id=(post.id) {
                "id: " (post.id)
                br;
                (post.body)
            }
        }
    }
}
