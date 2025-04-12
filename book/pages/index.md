# warp

[warp](https://crates.io/crates/warp) is a super-easy, composable, web server framework for warp speeds. If you are hard-core, you can learn from the [documentation of warp](https://docs.rs/warp/),
I personally prefer looking at working examples.

This books is mostly sharing the examples that can be found in the [source repository of warp](https://github.com/seanmonstar/warp)
with some extra examples an explanation.

## The goal of this book

Make it easier to get started using **warp** and to contribute back examples and documentation to the warp project.

As of this writing in April 2025, the last commit to the warp repository was on  Jul 23, 2024 and the last release of warp is `v0.3.7` that came out April 5th 2024.
It seem warp is currently not being maintained so for now I won't even attempt to send contributions to the project.

Nevertheless it seems to be in use including in the [mdbook](https://rust-lang.github.io/mdBook/) project and that is the main motivation for me to learn a bit about warp.

## How to use the examples?

In order to try the examples you can clone our fork of the [repository](https://github.com/szabgab/warp/).

```
git clone https://github.com/szabgab/warp/
```

cd into the created folder:

```
cd warp
```

Run the [Hello World](./hello-world.md) example:

```
cargo run --example hello
```

