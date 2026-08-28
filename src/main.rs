use std::env;
use std::io::{self, Write};
use std::process::{self, Command, ExitStatus};
use std::time::Instant;

use taskbell::{format_duration, parse_args, Config, ParseOutcome};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    match parse_args(env::args().skip(1)) {
        Ok(ParseOutcome::Help) => {
            print_help();
        }
        Ok(ParseOutcome::Version) => {
            println!("taskbell {VERSION}");
        }
        Ok(ParseOutcome::Run(config)) => run(config),
        Err(error) => {
            eprintln!("taskbell: {error}");
            eprintln!("Try `taskbell --help` for usage.");
            process::exit(2);
        }
    }
}

fn run(config: Config) -> ! {
    let started = Instant::now();
    let status = match Command::new(&config.command[0])
        .args(&config.command[1..])
        .status()
    {
        Ok(status) => status,
        Err(error) => {
            eprintln!("taskbell: failed to start `{}`: {error}", config.command[0]);
            process::exit(127);
        }
    };

    let elapsed_secs = started.elapsed().as_secs();
    let outcome = if status.success() { "finished" } else { "failed" };
    let message = format!(
        "{} {} after {}",
        config.command[0],
        outcome,
        format_duration(elapsed_secs)
    );

    eprintln!("taskbell: {message}");

    if elapsed_secs >= config.min_duration_secs {
        if config.no_notify || !send_notification(&config.title, &message) {
            ring_terminal_bell();
        }
    }

    exit_with_status(status)
}

#[cfg(target_os = "linux")]
fn send_notification(title: &str, message: &str) -> bool {
    Command::new("notify-send")
        .arg("--app-name=TaskBell")
        .arg(title)
        .arg(message)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

#[cfg(not(target_os = "linux"))]
fn send_notification(_title: &str, _message: &str) -> bool {
    false
}

fn ring_terminal_bell() {
    let _ = io::stderr().write_all(b"\x07");
    let _ = io::stderr().flush();
}

#[cfg(unix)]
fn exit_with_status(status: ExitStatus) -> ! {
    use std::os::unix::process::ExitStatusExt;

    if let Some(code) = status.code() {
        process::exit(code);
    }

    if let Some(signal) = status.signal() {
        process::exit(128 + signal);
    }

    process::exit(1)
}

#[cfg(not(unix))]
fn exit_with_status(status: ExitStatus) -> ! {
    process::exit(status.code().unwrap_or(1))
}

fn print_help() {
    println!(
        "TaskBell {VERSION}\n\
Get notified when long-running terminal commands finish.\n\n\
USAGE:\n    taskbell [OPTIONS] -- <COMMAND> [ARGS...]\n\n\
OPTIONS:\n    --min-duration <SECONDS>  Notify only when the command runs at least this long [default: 0]\n    --title <TITLE>           Notification title [default: TaskBell]\n    --no-notify               Skip desktop notification and use the terminal bell\n    -h, --help                Print help\n    -V, --version             Print version\n\n\
EXAMPLES:\n    taskbell -- cargo build --release\n    taskbell --min-duration 30 -- make -j4\n    taskbell --title \"Backup finished\" -- rsync -a src/ backup/"
    );
}
