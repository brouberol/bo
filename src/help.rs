use crate::{utils, Commands};
use std::collections::HashMap;

pub struct Section {
    pub title: String,
    pub entries: HashMap<String, &'static str>,
}

impl Section {
    /// Returns the size of the biggest entry key in the section
    fn max_entry_key_size(&self) -> usize {
        if let Some(key) = self.entries.keys().max_by_key(|k| k.len()) {
            key.len()
        } else {
            0
        }
    }

    #[must_use]
    pub fn format(&self) -> String {
        let title_str = utils::as_bold(self.title.as_str());
        let mut body: Vec<String> = Vec::new();
        let whitespace_offset = self.max_entry_key_size();
        for (cmd, cmd_help_msg) in &self.entries {
            let whitespace = " ".repeat(whitespace_offset - cmd.len() + 1);
            body.push(format!("{cmd}{whitespace}=> {cmd_help_msg}"));
        }
        body.sort();
        let body_str = body.join("\n  ");
        format!("{title_str}\n  {body_str}")
    }
}
pub struct Help {
    pub sections: Vec<Section>,
}

impl Help {
    #[must_use]
    pub fn new() -> Help {
        Help {
            sections: vec![
                Section {
                    title: String::from("Normal commands"),
                    entries: HashMap::from([
                        (
                            "j".to_owned(),
                            "move cursor down one row (<n>j moves it by n rows)",
                        ),
                        (
                            "k".to_owned(),
                            "move cursor up one row (<n>k moves it by n rows)",
                        ),
                        ("h".to_owned(), "move cursor left (<n>h moves it n times)"),
                        ("l".to_owned(), "move cursor right (<n>l moves it n times)"),
                        (
                            "}".to_owned(),
                            "move to the end of the current paragraph (<n>} moves n times)",
                        ),
                        (
                            "{".to_owned(),
                            "move to the start of the current paragraph (<n>{ moves n times)",
                        ),
                        (
                            "w".to_owned(),
                            "move to the end of the current word (<n>w moves n times)",
                        ),
                        (
                            "b".to_owned(),
                            "move to the start of the current word (<n>b moves n times)",
                        ),
                        ("i".to_owned(), "switch to insert mode"),
                        ("g".to_owned(), "go to beginining of document"),
                        ("G".to_owned(), "go to end of document"),
                        ("0".to_owned(), "go to first character in line"),
                        (
                            "^".to_owned(),
                            "go to first non-whitespace character in line",
                        ),
                        ("$".to_owned(), "go to end of line"),
                        ("H".to_owned(), "go to first line in screen"),
                        ("M".to_owned(), "go to line in the middle of the screen"),
                        ("L".to_owned(), "go to last line in screen"),
                        ("n%".to_owned(), "move to n% in the file"),
                        ("/".to_owned(), "open search prompt"),
                        ("n".to_owned(), "go to next search match"),
                        ("N".to_owned(), "go to previous search match"),
                        ("d".to_owned(), "delete current line"),
                        ("x".to_owned(), "delete current character"),
                        (
                            "o".to_owned(),
                            "insert newline after current line & enter insert mode",
                        ),
                        (
                            "O".to_owned(),
                            "insert newline before current line & enter insert mode",
                        ),
                        ("A".to_owned(), "go to end of line & enter insert mode"),
                        ("J".to_owned(), "join the current line with the next one"),
                        (":".to_owned(), "open command prompt"),
                    ]),
                },
                Section {
                    title: String::from("Prompt commands"),
                    entries: HashMap::from([
                        (format!("{}", Commands::Help), "display this help screen"),
                        (format!("{}", Commands::LineNumbers), "toggle line numbers"),
                        (format!("{} <filename>", Commands::Open), "open a new file"),
                        (
                            format!("{}/{} <filename>", Commands::Open, Commands::OpenShort),
                            "open a file",
                        ),
                        (format!("{}", Commands::Quit), "quit bo"),
                        (
                            format!("{}", Commands::LineNumbers),
                            "toggle line/word stats",
                        ),
                        (format!("{} <new_name>", Commands::New), "save"),
                        (format!("{}", Commands::SaveAnQuit), "save and quit"),
                    ]),
                },
                Section {
                    title: String::from("Insert commands"),
                    entries: HashMap::from([("Esc".to_owned(), "go back to normal mode")]),
                },
            ],
        }
    }

    #[must_use]
    pub fn format(&self) -> String {
        let mut out: Vec<String> = Vec::new();
        for section in &self.sections {
            let section_format = section.format();
            out.push(section_format);
        }
        out.join("\n\n")
    }
}

impl Default for Help {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "./help_test.rs"]
mod help_test;
