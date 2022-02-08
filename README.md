# tokio-graceful-shutdown-example

This repo contains a UNIX-friendly example of implementing graceful shutdown for programs using [tokio.](https://tokio.rs/), which run [tokio::process::Command.](https://docs.rs/tokio/latest/tokio/process/struct.Command.html)

It's also possible to use [futures::select](https://docs.rs/futures/latest/futures/macro.select.html) for a cleaner code style with [select!](https://docs.rs/futures/latest/futures/macro.select.html) macro.

## Running locally

```
cargo run
# Ctrl + C for SIGINT
# Ctrl + \ for SIGQUIT
```