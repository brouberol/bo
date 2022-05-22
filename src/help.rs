use crate::utils;
use std::collections::HashMap;

pub struct Section {
    pub title: String,
    pub entries: HashMap<&'static str, &'static str>,
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
                        ("j", "move cursor down one row (<n>j moves it by n rows)"),
                        ("k", "move cursor up one row (<n>k moves it by n rows)"),
                        ("h", "move cursor left (<n>h moves it n times)"),
                        ("l", "move cursor right (<n>l moves it n times)"),
                        (
                            "}",
                            "move to the end of the current paragraph (<n>} moves n times)",
                        ),
                        (
                            "{",
                            "move to the start of the current paragraph (<n>{ moves n times)",
                        ),
                        (
                            "w",
                            "move to the end of the current word (<n>w moves n times)",
                        ),
                        (
                            "b",
                            "move to the start of the current word (<n>b moves n times)",
                        ),
                        ("i", "switch to insert mode"),
                        ("g", "go to beginining of document"),
                        ("G", "go to end of document"),
                        ("0", "go to first character in line"),
                        ("^", "go to first non-whitespace character in line"),
                        ("$", "go to end of line"),
                        ("H", "go to first line in screen"),
                        ("M", "go to line in the middle of the screen"),
                        ("L", "go to last line in screen"),
                        ("n%", "move to n% in the file"),
                        ("/", "open search prompt"),
                        ("n", "go to next search match"),
                        ("N", "go to previous search match"),
                        ("d", "delete current line"),
                        ("x", "delete current character"),
                        ("o", "insert newline after current line & enter insert mode"),
                        (
                            "O",
                            "insert newline before current line & enter insert mode",
                        ),
                        ("A", "go to end of line & enter insert mode"),
                        ("J", "join the current line with the next one"),
                        (":", "open command prompt"),
                        ("u", "undo last operation"),
                    ]),
                },
                Section {
                    title: String::from("Prompt commands"),
                    entries: HashMap::from([
                        ("help", "display this help screen"),
                        ("ln", "toggle line numbers"),
                        ("new <filename>", "open a new file"),
                        ("open/o <filename>", "open a file"),
                        ("q", "quit bo"),
                        ("stats", "toggle line/word stats"),
                        ("w <new_name>", "save"),
                        ("wq", "save and quit"),
                    ]),
                },
                Section {
                    title: String::from("Insert commands"),
                    entries: HashMap::from([("Esc", "go back to normal mode")]),
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
