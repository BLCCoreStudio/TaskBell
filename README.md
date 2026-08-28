# TaskBell

**Get notified when long-running terminal commands finish.**

TaskBell is a small, open-source command wrapper for developers who do not want to keep watching a terminal while builds, tests, backups, renders, or other long-running jobs finish.

```bash
taskbell -- cargo build --release
taskbell --min-duration 30 -- make -j4
taskbell --title "Backup finished" -- rsync -a src/ backup/
```

## Why TaskBell?

TaskBell keeps the workflow deliberately simple:

- one command wrapper
- no account, daemon, server, telemetry, or background service
- preserves the wrapped command's exit status
- measures elapsed time
- Linux desktop notifications through `notify-send`
- terminal bell fallback when desktop notifications are unavailable
- optional minimum-duration threshold
- no shell interpolation: commands are executed directly with their argument list

## Status

**TaskBell v0.1.0 is available now as the first public release.**

The prebuilt release currently targets Linux x86_64. Other platforms can build from source while portability work continues.

[Download TaskBell v0.1.0](https://github.com/BLCCoreStudio/TaskBell/releases/tag/v0.1.0)

## Install the Linux release

Download the `TaskBell-v0.1.0-linux-x86_64.tar.gz` asset from the release page, then:

```bash
tar -xzf TaskBell-v0.1.0-linux-x86_64.tar.gz
cd TaskBell-v0.1.0-linux-x86_64
./taskbell --version
```

You can optionally place the binary somewhere on your `PATH`, for example `~/.local/bin`.

The release page also includes a SHA-256 checksum file for verifying the downloaded archive.

## Build from source

Requirements:

- Rust 1.74 or newer
- Linux for desktop notifications (`notify-send` is optional)

```bash
git clone https://github.com/BLCCoreStudio/TaskBell.git
cd TaskBell
cargo build --release
./target/release/taskbell --version
```

## Usage

```text
taskbell [OPTIONS] -- <COMMAND> [ARGS...]
```

Options:

```text
--min-duration <SECONDS>  Notify only if the command runs at least this long
--title <TITLE>           Set the desktop-notification title
--no-notify               Skip desktop notification and use the terminal bell
-h, --help                Print help
-V, --version             Print version
```

TaskBell returns the wrapped command's exit code. On Unix, if the wrapped process terminates because of a signal, TaskBell exits with `128 + signal`.

## Examples

Notify after a Rust release build:

```bash
taskbell -- cargo build --release
```

Ignore commands shorter than 30 seconds:

```bash
taskbell --min-duration 30 -- cargo test
```

Use the terminal bell without attempting a desktop notification:

```bash
taskbell --no-notify -- ./long-running-script.sh
```

## Security and privacy

TaskBell does not send command data anywhere. It has no telemetry, network client, remote backend, or account system.

The command is executed directly rather than through `sh -c`, so TaskBell does not add an extra shell-expansion layer around user-provided arguments.

For vulnerability reports, see [SECURITY.md](SECURITY.md).

## Contributing

Bug reports, focused feature proposals, tests, and portability improvements are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

TaskBell is open source under the [MIT License](LICENSE).

Built by **BLC Core Studio**.
