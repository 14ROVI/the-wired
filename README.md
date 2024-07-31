# The wired

This is a small project I am doing to learn htmx.

It uses:

- htmx
- maud
- auxm
- tailwind

## How to run

First, you will need to either install tailwind by using node or downloading a stand alone binary. As long as `tailwindcss` is available in path then the project can be built.

Then install dependencies and run the program:

```sh
cargo install
cargo run
```

## Development

I recommend installing cargo watch (`cargo install cargo-watch`) which allows you to run `cargo watch -x run`. This will automatically rebuild the program and run it on every file change. Ignored files are inherited from `.gitignore`. And thanks to `build.rs`, tailwind is also compiled automatically.