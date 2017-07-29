# `Manual definition of model and schema`

Diesel's `Getting Started` guide but without the magic.
It shows how to define the schema using `table!` and the model by implementing `Queryable` and `Insertable`.

## Usage

```
$ echo "DATABASE_URL=file:test.db" > .env
$ diesel migration run

$ cargo run --bin show_posts

$ cargo run --bin write_post
# write your post

$ cargo run --bin publish_post 1

$ cargo run --bin show_posts
# your post will be printed here

$ cargo run --bin delete_post <title of post>

$ cargo run --bin show_posts
# observe that no posts are shown
```
