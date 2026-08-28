# Contributing to TaskBell

Thanks for helping improve TaskBell.

## Development setup

TaskBell currently requires Rust 1.74 or newer.

```bash
git clone https://github.com/BLCCoreStudio/TaskBell.git
cd TaskBell
cargo test
```

Before opening a pull request, run:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
git diff --check
```

## Pull requests

Keep changes focused. Explain the user-visible behavior being changed and add or update tests when behavior is testable without relying on a desktop environment.

Do not add telemetry, analytics, remote command execution, or silent network communication.

## Issues

For bugs, include your operating system, TaskBell version or commit, the command shape that triggered the issue, expected behavior, and actual behavior. Remove secrets or sensitive arguments before posting logs publicly.

Security vulnerabilities should not be reported in public issues. See [SECURITY.md](SECURITY.md).
