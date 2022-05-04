use std::fmt;
use std::str;

#[derive(Debug, PartialEq)]
pub enum Commands {
    Debug,
    ForceQuit,
    Help,
    LineNumbers,
    New,
    Open,
    OpenShort,
    Quit,
    Save,
    SaveAnQuit,
    Stats,
    Unknown,
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let command_str = match self {
            Commands::Debug => "debug",
            Commands::ForceQuit => "wq!",
            Commands::Help => "help",
            Commands::LineNumbers => "ln",
            Commands::New => "new",
            Commands::Open => "open",
            Commands::OpenShort => "o",
            Commands::Quit => "q",
            Commands::Save => "w",
            Commands::SaveAnQuit => "wq",
            Commands::Stats => "stats",
            Commands::Unknown => "",
        };
        write!(f, "{}", command_str)
    }
}
impl Commands {
    #[must_use]
    pub fn as_str(&self) -> String {
        format!("{}", &self)
    }

    #[must_use]
    pub fn parse(s: &str) -> Self {
        match s {
            "debug" => Commands::Debug,
            "wq!" => Commands::ForceQuit,
            "help" => Commands::Help,
            "ln" => Commands::LineNumbers,
            "new" => Commands::New,
            "open" => Commands::Open,
            "o" => Commands::OpenShort,
            "q" => Commands::Quit,
            "w" => Commands::Save,
            "wq" => Commands::SaveAnQuit,
            "stats" => Commands::Stats,
            _ => Commands::Unknown,
        }
    }
}

#[cfg(test)]
#[path = "./commands_test.rs"]
mod commands_test;
