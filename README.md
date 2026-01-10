# brd
A simple imageboard site that supports private walled-garden communication

## Building & running
`cargo run`

## Notes on configurability
Some users of this software may not want signups enabled (just anonymouse posts), or may want open signups (signups without a key).

## Simple todo
Boilerplate
- sqlx setup

Admin utilities
- GET boards
- POST board
- DELETE board
- UPDATE board

Imageboard
- Require an image to create a post, with optional text content.
- POST post
- GET posts
- Users can reply to posts, which are essentially just more posts linked to the original one
- POST reply
- GET replies

Site
- Homepage (see all boards)
- Boards page (see all posts in board)
- Post page (see all replies to post & post)
- User page (see user and last posts/replies)
- Users page (see all users)

Users
- POST user
- GET users
- Signup to create user (with key?)
- Login to post & view posts
- Only "admin user" can execute admin api
