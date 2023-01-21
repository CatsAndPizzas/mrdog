# MR. Dog

![](https://i.imgur.com/YZO8zBH.png)

## Dev notes
Run CLI without params locally to hit a TODO error:
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/mrdog`
thread 'main' panicked at 'not yet implemented: initialize_providers', src/mrdog.rs:30:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Run CLI help (notice the `--` delim, to pass args to `mrdog` and not `cargo`)
```
$ cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/mrdog --help`
MR. Dog groups and watches merge requests across all your *hubs

Usage: mrdog [COMMAND]

Commands:
  set   Set config value
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Run set command (to hit a todo error in code again)
```
$ cargo run -- set github foobar
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/mrdog set github foobar`
thread 'main' panicked at 'not yet implemented: Set config GitHubApiToken("foobar")', src/mrdog/config_storage.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```