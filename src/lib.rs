use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub min_duration_secs: u64,
    pub title: String,
    pub no_notify: bool,
    pub command: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseOutcome {
    Run(Config),
    Help,
    Version,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError(pub String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ParseError {}

pub fn parse_args<I, S>(args: I) -> Result<ParseOutcome, ParseError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut args = args.into_iter().map(Into::into).peekable();
    let mut min_duration_secs = 0;
    let mut title = String::from("TaskBell");
    let mut no_notify = false;
    let mut command = Vec::new();
    let mut parsing_options = true;

    while let Some(arg) = args.next() {
        if !parsing_options {
            command.push(arg);
            command.extend(args);
            break;
        }

        match arg.as_str() {
            "--" => parsing_options = false,
            "-h" | "--help" => return Ok(ParseOutcome::Help),
            "-V" | "--version" => return Ok(ParseOutcome::Version),
            "--no-notify" => no_notify = true,
            "--min-duration" => {
                let value = args
                    .next()
                    .ok_or_else(|| ParseError("--min-duration requires a value".into()))?;
                min_duration_secs = value.parse::<u64>().map_err(|_| {
                    ParseError(
                        "--min-duration must be a non-negative integer number of seconds".into(),
                    )
                })?;
            }
            "--title" => {
                title = args
                    .next()
                    .ok_or_else(|| ParseError("--title requires a value".into()))?;
                if title.trim().is_empty() {
                    return Err(ParseError("--title cannot be empty".into()));
                }
            }
            _ if arg.starts_with('-') => {
                return Err(ParseError(format!("unknown option: {arg}")));
            }
            _ => {
                command.push(arg);
                command.extend(args);
                break;
            }
        }
    }

    if command.is_empty() {
        return Err(ParseError(
            "no command provided; use `taskbell -- <COMMAND> [ARGS...]`".into(),
        ));
    }

    Ok(ParseOutcome::Run(Config {
        min_duration_secs,
        title,
        no_notify,
        command,
    }))
}

pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        return format!("{seconds}s");
    }

    let minutes = seconds / 60;
    let secs = seconds % 60;
    if minutes < 60 {
        return format!("{minutes}m {secs}s");
    }

    let hours = minutes / 60;
    let mins = minutes % 60;
    format!("{hours}h {mins}m {secs}s")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_command_after_separator() {
        let parsed = parse_args(["--min-duration", "30", "--", "cargo", "test"]).unwrap();
        assert_eq!(
            parsed,
            ParseOutcome::Run(Config {
                min_duration_secs: 30,
                title: "TaskBell".into(),
                no_notify: false,
                command: vec!["cargo".into(), "test".into()],
            })
        );
    }

    #[test]
    fn parses_command_without_separator() {
        let parsed = parse_args(["echo", "hello"]).unwrap();
        match parsed {
            ParseOutcome::Run(config) => assert_eq!(config.command, ["echo", "hello"]),
            _ => panic!("expected run outcome"),
        }
    }

    #[test]
    fn rejects_missing_command() {
        let err = parse_args(["--min-duration", "10"]).unwrap_err();
        assert!(err.to_string().contains("no command provided"));
    }

    #[test]
    fn rejects_invalid_duration() {
        let err = parse_args(["--min-duration", "nope", "--", "true"]).unwrap_err();
        assert!(err.to_string().contains("non-negative integer"));
    }

    #[test]
    fn formats_duration() {
        assert_eq!(format_duration(9), "9s");
        assert_eq!(format_duration(65), "1m 5s");
        assert_eq!(format_duration(3_661), "1h 1m 1s");
    }
}
