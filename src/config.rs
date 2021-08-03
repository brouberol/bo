#[derive(Default, Debug)]
pub struct Config {
    pub display_line_numbers: bool,
    pub display_stats: bool,
}

impl Config {
    #[must_use]
    pub fn toggle(config: bool) -> bool {
        !config
    }
}

#[cfg(test)]
#[path = "./config_test.rs"]
mod config_test;
