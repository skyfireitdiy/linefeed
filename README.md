# `linefeed2`

`linefeed2` is a configurable, concurrent, extensible, interactive input reader
for Unix terminals and Windows console.

[API Documentation](https://docs.rs/linefeed2/)

`linefeed2` follows the paradigm of GNU Readline, binding key sequences to
commands that modify input state. `linefeed2` supports many GNU Readline commands.
However, `linefeed2` does not implement all commands supported by GNU Readline.
If there's a command you want to be implemented, file an issue!

`linefeed2` also interprets GNU Readline `inputrc` configuration files.
First, it will check for a filename in the environment variable `INPUTRC`.
Then, on Unix, it will check `$HOME/.inputrc` or `/etc/inputrc`;
while, on Windows, it will check `%APPDATA%\linefeed\inputrc`.
Only the first of these that is present is parsed and evaluated.

## Building

To include `linefeed2` in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
linefeed2 = "0.6"
```

### Demo

The `linefeed2` project contains a demo program for testing functionality.
To run the demo, run the following from a clone of the `linefeed2` project:

    cargo run --example demo

## License

`linefeed2` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
