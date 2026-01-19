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
        @let most = &boards[..boards.len()-1];
        @let last = &boards[boards.len()-1];

        @for board in most {
            a href={ "/b/" (board.name) } { (board.name) }
            span { " / " }
        }

        a href={ "/b/" (last.name) } { (last.name) }
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

pub fn post_partial(board: &Board, post: &Post, link: bool) -> Markup {
    html! {
        pre class="post" {
            @if link {
                a href={ "/b/" (board.name)  "/" (post.id) } { "id: " (post.id) "\n" }
            } @else {
                "id: " (post.id) "\n"
            }

            @if let Some(file_name) = &post.file_name {
                img src={ "/" (file_name) };
            }
            "body: " (post.body) "\n"
        }
    }
}

pub fn threads_partial(board: &Board, posts: Vec<Post>) -> Markup {
    html! {
        @for post in (posts.iter().rev()) {
            (post_partial(board, post, true))
        }
    }
}

pub fn post_form() -> Markup {
    html! {
        form method="post" enctype="multipart/form-data" {
            textarea name="body" rows="6" cols="36" {}
            br;
            input type="file" name="file";
            br;
            input name="submit" type="submit" value="submit";
        }
    }
}

pub fn board_page(board: &Board, threads_partial: Markup) -> Markup {
    html! {
        (page(&board.name))
        h2 { (&board.description) }
        a href="/" { "<= Go back" }

        br; br;
        p { "Post a thread on this board" }
        (post_form())
        br;

        (threads_partial)
    }
}

pub fn thread_page(board: &Board, posts: Vec<Post>) -> Markup {
    html! {
        @let first = &posts[0];
        (page(&format!("{} / thread {}", &board.name, first.id)))
        a href={ "/b/" (board.name) } { "<= Go back" }

        br; br;
        p { "Post a reply to this thread"  }
        (post_form())
        br;

        @for post in &posts[..] {
            (post_partial(board, post, false))
        }
    }
}
