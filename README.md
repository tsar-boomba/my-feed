# my-feed

An RSS feed for me.

## Building

I'm using sqlite as the database so you might need it installed depending on your OS. I think rusqlite/libsqlite3-sys should compile from source for you though.

Install `sqlx-cli` and run `mkdir my-feed-data`, then `sqlx database create` and `sqlx migrate run` to create the sqlite db and run the migrations against it. After that you need to build the frontend with `cd web && pnpm i && pnpm build`. Finally, you can run it with `cargo run`. For FE development, go to `web` and run `pnpm dev`.
